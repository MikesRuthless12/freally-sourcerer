#!/usr/bin/env node
// TASK-103 — prove a published release can actually be updated *to*.
//
// The in-app updater fails closed and fails **silently**: a signature it
// cannot verify surfaces as "no update found", not as an error. So a
// release signed by the wrong key looks exactly like a release nobody
// has published yet, and the first person to notice is a user who never
// gets an update again.
//
// The only thing that proves the CI secret holds the key we think it
// does is comparing the key id *inside a published signature* against
// the key id of the pubkey compiled into the binaries. A repository
// secret cannot be read back, so there is no other way to check it.
//
// Usage:
//   node scripts/verify-updater-chain.mjs            # the latest release
//   node scripts/verify-updater-chain.mjs v0.23.2    # a specific tag
//
// Requires `gh` authenticated against the repo. Exits non-zero on any
// mismatch, so it is safe to gate a release announcement on.
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const tag = process.argv[2];

/** Key id of a minisign public key or signature line.
 *  Layout is 2-byte algorithm, 8-byte key id little-endian, then the key
 *  or signature bytes. Both files are "comment line, then base64", so
 *  one parser serves both. */
function keyId(armored) {
  const text = Buffer.from(armored, "base64").toString("utf8");
  const payload = text.trim().split("\n")[1];
  if (!payload) throw new Error(`no payload line in:\n${text}`);
  const raw = Buffer.from(payload, "base64");
  return Buffer.from(raw.subarray(2, 10)).reverse().toString("hex").toUpperCase();
}

const conf = JSON.parse(
  readFileSync(join(repoRoot, "apps/freally-ui/src-tauri/tauri.conf.json"), "utf8"),
);
const expected = keyId(conf.plugins.updater.pubkey);

const dir = mkdtempSync(join(tmpdir(), "freally-updater-"));
try {
  const args = ["release", "download"];
  if (tag) args.push(tag);
  args.push("-p", "latest.json", "-D", dir);
  execFileSync("gh", args, { cwd: repoRoot, stdio: ["ignore", "inherit", "inherit"] });

  const manifest = JSON.parse(readFileSync(join(dir, "latest.json"), "utf8"));
  console.log(`\nmanifest version : ${manifest.version}`);
  console.log(`published        : ${manifest.pub_date}`);
  console.log(`pubkey key id    : ${expected}   (apps/freally-ui/src-tauri/tauri.conf.json)\n`);

  const platforms = Object.entries(manifest.platforms ?? {});
  if (platforms.length === 0) {
    // An empty manifest is the `createUpdaterArtifacts: false` failure —
    // the build succeeds, the release publishes, and nothing can update.
    console.error("FAIL: the manifest lists no platforms at all.");
    process.exit(1);
  }

  let bad = 0;
  for (const [platform, info] of platforms) {
    let id;
    try {
      id = keyId(info.signature);
    } catch (e) {
      console.error(`  UNREADABLE  ${platform}: ${e.message}`);
      bad++;
      continue;
    }
    const ok = id === expected;
    if (!ok) bad++;
    console.log(`  ${ok ? "ok      " : "MISMATCH"}  ${platform.padEnd(22)} ${id}`);
  }

  console.log();
  if (bad > 0) {
    console.error(
      `FAIL: ${bad} of ${platforms.length} signatures were not made by the key ` +
        `compiled into the binaries. Installs of this version will refuse every ` +
        `future update, and will not say why.`,
    );
    process.exit(1);
  }
  console.log(
    `OK: all ${platforms.length} signatures carry ${expected}. ` +
      `The published release is updatable by shipped binaries.`,
  );
} finally {
  rmSync(dir, { recursive: true, force: true });
}
