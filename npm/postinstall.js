#!/usr/bin/env node

const {
  chmodSync,
  constants,
  copyFileSync,
  createWriteStream,
  existsSync,
  mkdtempSync,
  mkdirSync,
  readFileSync,
  rmSync,
} = require("node:fs");
const { createHash } = require("node:crypto");
const { get } = require("node:https");
const { join } = require("node:path");
const { tmpdir } = require("node:os");

const skipName = "TSS_SKIP_DOWNLOAD";
const repo = "https://github.com/uditgoenka/tss";
const version = require("../package.json").version;
const checksums = require("./checksums.json");
const platform = process.platform;
const arch = process.arch;
const installDir = join(__dirname, "bin-dist", platform, arch);
const binaryName = platform === "win32" ? "tss.exe" : "tss";
const installPath = join(installDir, binaryName);
const trustedHosts = new Set([
  "github.com",
  "objects.githubusercontent.com",
  "github-releases.githubusercontent.com",
  "release-assets.githubusercontent.com",
]);

if (process.env[skipName]) {
  console.log("tss: skipping binary download because TSS_SKIP_DOWNLOAD is set");
  process.exit(0);
}

mkdirSync(installDir, { recursive: true });

if (existsSync(installPath)) {
  process.exit(0);
}

const archiveName = `tss-${version}-${platform}-${arch}`;
const releaseUrl = `${repo}/releases/download/v${version}/${archiveName}`;
const expectedSha256 = expectedChecksum(archiveName, `${platform}-${arch}`);

download(releaseUrl, installPath, expectedSha256)
  .then(() => {
    chmodSync(installPath, 0o755);
    console.log(`tss: installed ${releaseUrl}`);
  })
  .catch((error) => {
    console.error(`tss: failed to install ${releaseUrl}`);
    console.error(error.message);
    console.error("Set TSS_SKIP_DOWNLOAD=1 for source builds, or set TSS_BINARY to a local binary.");
    process.exit(1);
  });

function download(url, destination, expectedSha256, redirects = 0) {
  if (redirects > 5) {
    return Promise.reject(new Error("too many redirects"));
  }

  const tempDir = mkdtempSync(join(tmpdir(), "tss-download-"));
  chmodSync(tempDir, 0o700);
  const temporary = join(tempDir, binaryName);
  const parsedUrl = trustedUrl(url);

  return new Promise((resolve, reject) => {
    const request = get(parsedUrl, (response) => {
      if (
        response.statusCode >= 300 &&
        response.statusCode < 400 &&
        response.headers.location
      ) {
        response.resume();
        let redirectUrl;
        try {
          redirectUrl = trustedUrl(response.headers.location, parsedUrl);
        } catch (error) {
          reject(error);
          return;
        }
        resolve(download(redirectUrl, destination, expectedSha256, redirects + 1));
        return;
      }

      if (response.statusCode !== 200) {
        response.resume();
        reject(new Error(`unexpected status ${response.statusCode}`));
        return;
      }

      const file = createDownloadStream(temporary);
      response.pipe(file);
      file.on("finish", () => {
        file.close((error) => {
          if (error) {
            reject(error);
            return;
          }
          const actualSha256 = sha256File(temporary);
          if (actualSha256 !== expectedSha256) {
            reject(new Error(`checksum mismatch for ${archiveName}`));
            return;
          }
          copyFileSync(temporary, destination, constants.COPYFILE_EXCL);
          resolve();
        });
      });
      file.on("error", reject);
    });

    request.on("error", reject);
    request.setTimeout(30000, () => request.destroy(new Error("download timed out")));
  }).catch((error) => {
    throw error;
  }).finally(() => {
    rmSync(tempDir, { recursive: true, force: true });
  });
}

function expectedChecksum(archiveName, platformKey) {
  const value = checksums[archiveName] || checksums[platformKey];
  if (!/^[a-f0-9]{64}$/i.test(value || "")) {
    throw new Error(`missing SHA-256 checksum for ${archiveName} in npm/checksums.json`);
  }
  return value.toLowerCase();
}

function createDownloadStream(path) {
  return createWriteStream(path, {
    flags: "wx",
    mode: 0o600,
  });
}

function trustedUrl(input, base) {
  const url = new URL(input, base);
  if (url.protocol !== "https:") {
    throw new Error(`untrusted download protocol: ${url.protocol}`);
  }
  if (!trustedHosts.has(url.hostname)) {
    throw new Error(`untrusted download host: ${url.hostname}`);
  }
  return url;
}

function sha256File(path) {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}
