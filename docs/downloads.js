// Point the Download menu at the release that actually exists.
//
// GitHub Pages redeploys from `main` the moment a PR merges, but the release
// build takes ~40 minutes across four serialized legs and then sits as a
// *draft* until someone publishes it. So the site starts advertising a
// version whose assets do not exist yet: on 2026-08-19 every Download button
// 404'd for about three hours.
//
// Hardcoded hrefs cannot fix that, because the page is deployed by a
// different trigger than the release. Reading the assets at page load can:
// `/releases/latest` **excludes drafts and prereleases**, so during that
// window it keeps returning the last release a user can actually install.
//
// The markup keeps working hrefs as the fallback. This never removes a link
// or leaves the menu empty — no network, a rate-limited API (unauthenticated
// GitHub allows 60 requests/hour/IP), or a shape we do not recognise all end
// with the page exactly as served.

(function () {
  "use strict";

  var API = "https://api.github.com/repos/MikesRuthless12/freally-sourcerer/releases/latest";

  // Which asset belongs in which row. Matched on the filename's tail because
  // the version sits in the middle of every name, which is the part that
  // changes. Each pattern identifies its row on its own — `aarch64.dmg` and
  // `x64.dmg` do not overlap — so no row depends on being tried before
  // another. It also means the `.sig` files and `latest.json` sitting in the
  // same release match nothing and are ignored.
  var ROWS = {
    "win-setup": /x64-setup\.exe$/i,
    "win-msi": /\.msi$/i,
    "mac-arm": /aarch64\.dmg$/i,
    "mac-x64": /x64\.dmg$/i,
    "linux-appimage": /\.AppImage$/i,
    "linux-deb": /\.deb$/i,
    "linux-rpm": /\.rpm$/i
  };

  function megabytes(bytes) {
    return (bytes / (1024 * 1024)).toFixed(2) + " MB";
  }

  function releaseDate(iso) {
    var d = new Date(iso);
    if (isNaN(d.getTime())) return null;
    return d.toLocaleDateString("en-US", { year: "numeric", month: "short", day: "numeric" });
  }

  function apply(release) {
    var assets = release.assets || [];
    if (!assets.length) return;

    var tag = String(release.tag_name || "");
    var version = tag.replace(/^v/, "");
    var matched = 0;

    Object.keys(ROWS).forEach(function (key) {
      var anchor = document.querySelector('[data-dl="' + key + '"]');
      if (!anchor) return;
      var asset = null;
      for (var i = 0; i < assets.length; i++) {
        if (ROWS[key].test(String(assets[i].name || ""))) {
          asset = assets[i];
          break;
        }
      }
      // A row with no matching asset keeps the href it was served with. That
      // is a stale link rather than a broken one, and silently deleting a
      // platform's download would be the worse failure.
      if (!asset || !asset.browser_download_url) return;
      anchor.href = asset.browser_download_url;
      var size = anchor.querySelector("[data-dl-size]");
      // Byte sizes were previously read by hand from the published assets
      // and pasted in, which is why they went stale between releases.
      if (size && typeof asset.size === "number") size.textContent = megabytes(asset.size);
      matched++;
    });

    if (!matched) return;

    var badge = document.querySelector("[data-dl-version]");
    var pageVersion = badge ? badge.textContent.trim() : "";
    if (badge && version) badge.textContent = version;

    var date = document.querySelector("[data-dl-date]");
    var published = releaseDate(release.published_at);
    if (date && published) date.textContent = "Released " + published + " · Latest";

    // Say so when the page is describing a release that is not out yet. The
    // notes below are written per-release, so they now describe something
    // newer than the files these links point at, and saying nothing would
    // make the version badge look like a mistake.
    if (version && pageVersion && version !== pageVersion) {
      var note = document.querySelector("[data-dl-note]");
      if (note) {
        note.textContent =
          "Version " + pageVersion + " is still publishing. These downloads are " +
          version + ", the newest release available now.";
        note.hidden = false;
      }
    }
  }

  function load() {
    if (typeof fetch !== "function") return;
    fetch(API, { headers: { Accept: "application/vnd.github+json" } })
      .then(function (r) {
        if (!r.ok) throw new Error("HTTP " + r.status);
        return r.json();
      })
      .then(apply)
      .catch(function () {
        // Rate limit, offline, blocked by a network policy. The served
        // markup already has working links; leave them alone.
      });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", load);
  } else {
    load();
  }
})();
