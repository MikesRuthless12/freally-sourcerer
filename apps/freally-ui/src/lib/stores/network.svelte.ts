import * as ipc from "../ipc/network";
import type { NetworkStatus } from "../ipc/types";

const DEFAULT_STATUS: NetworkStatus = {
  https_running: false,
  https_bind: "127.0.0.1",
  https_port: 7437,
  https_token_fingerprint: null,
  api_running: false,
  api_port: 7438
};

class NetworkStore {
  status = $state<NetworkStatus>({ ...DEFAULT_STATUS });
  snapshotStatus: NetworkStatus = { ...DEFAULT_STATUS };
  /// Local-only desired bind/port — only pushed to the daemon when the
  /// user clicks Apply or hits Start. Avoids restarting the server on
  /// every keystroke in the port input.
  desiredBind = $state<string>("127.0.0.1");
  desiredPort = $state<number>(7437);
  desiredApiPort = $state<number>(7438);
  forceHttps = $state<boolean>(true);
  legacyAuth = $state<boolean>(false);
  legacyFtp = $state<boolean>(false);

  async hydrate() {
    try {
      this.status = await ipc.status();
      if (this.status.https_bind) this.desiredBind = this.status.https_bind;
      if (this.status.https_port) this.desiredPort = this.status.https_port;
      if (this.status.api_port) this.desiredApiPort = this.status.api_port;
    } catch (e) {
      console.warn("[network] hydrate failed:", e);
    }
  }

  snapshot() {
    this.snapshotStatus = JSON.parse(JSON.stringify(this.status));
  }
  rollback() {
    this.status = JSON.parse(JSON.stringify(this.snapshotStatus));
  }

  async startHttps() {
    this.status = await ipc.startHttps({
      bind: this.desiredBind,
      port: this.desiredPort,
      force_https: this.forceHttps,
      legacy_auth: this.legacyAuth
    });
  }

  async stopHttps() {
    await ipc.stopHttps();
    this.status = { ...this.status, https_running: false };
  }

  async regenToken() {
    const r = await ipc.regenToken();
    this.status = { ...this.status, https_token_fingerprint: r.fingerprint };
  }

  async startApi() {
    await ipc.startApi({ port: this.desiredApiPort, legacy_ftp: this.legacyFtp });
    this.status = { ...this.status, api_running: true, api_port: this.desiredApiPort };
  }

  async stopApi() {
    await ipc.stopApi();
    this.status = { ...this.status, api_running: false };
  }

  async flush(): Promise<void> {
    // Buttons drive the lifecycle directly; nothing to flush.
  }

  async reset(): Promise<void> {
    if (this.status.https_running) await this.stopHttps();
    if (this.status.api_running) await this.stopApi();
    this.desiredBind = DEFAULT_STATUS.https_bind ?? "127.0.0.1";
    this.desiredPort = DEFAULT_STATUS.https_port ?? 7437;
    this.desiredApiPort = DEFAULT_STATUS.api_port ?? 7438;
    this.forceHttps = true;
    this.legacyAuth = false;
    this.legacyFtp = false;
  }
}

export const networkStore = new NetworkStore();
