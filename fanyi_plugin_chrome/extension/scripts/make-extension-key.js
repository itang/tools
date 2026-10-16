/**
 * Generates the RSA keypair used to give the unpacked extension a stable ID.
 *
 * The public key is written into `extension/src/manifest.json` as `key`, and
 * the derived extension ID is used for the native host `allowed_origins`.
 *
 *   bun scripts/make-extension-key.js
 */

import { createHash, generateKeyPairSync } from "node:crypto";

/** Chromium maps the first 16 bytes of SHA256(spki-der) to a-p. */
function extensionIdFromDer(der) {
  const hash = createHash("sha256").update(der).digest();
  let id = "";
  for (let i = 0; i < 16; i += 1) {
    id += String.fromCharCode(97 + (hash[i] >> 4));
    id += String.fromCharCode(97 + (hash[i] & 0x0f));
  }
  return id;
}

const { publicKey } = generateKeyPairSync("rsa", { modulusLength: 2048 });
const der = publicKey.export({ type: "spki", format: "der" });
const key = Buffer.from(der).toString("base64");
const id = extensionIdFromDer(der);

console.log(JSON.stringify({ key, id }, null, 2));
