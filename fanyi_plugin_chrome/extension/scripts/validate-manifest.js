/**
 * Static sanity check for the built extension under `dist/`.
 *
 * Catches the common reasons Chrome refuses to load an unpacked extension:
 * a malformed manifest, a missing required permission, or a reference to a
 * file that was not emitted by the build.
 *
 *   bun scripts/validate-manifest.js
 */

import { access } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const dist = join(root, "..", "dist");
const manifestPath = join(dist, "manifest.json");

let manifest;
try {
  manifest = JSON.parse(await Bun.file(manifestPath).text());
} catch (error) {
  console.error(`无法读取 ${manifestPath}：${error.message}`);
  process.exit(1);
}

const errors = [];

if (manifest.manifest_version !== 3) errors.push("manifest_version 必须是 3");
if (!manifest.name) errors.push("缺少 name");
if (!/^\d+(\.\d+){0,3}$/.test(manifest.version || "")) errors.push("version 格式不合法");
if (!manifest.key) errors.push("缺少 key（用于稳定的扩展 ID）");

for (const permission of ["nativeMessaging", "storage", "scripting", "contextMenus", "activeTab"]) {
  if (!(manifest.permissions || []).includes(permission)) errors.push(`缺少权限：${permission}`);
}

const hostPermissions = manifest.host_permissions || [];
if (!hostPermissions.some((pattern) => pattern.includes("x.com"))) {
  errors.push("host_permissions 未包含 x.com");
}
if (!(manifest.optional_host_permissions || []).length) {
  errors.push("缺少 optional_host_permissions（用于启用其它站点）");
}

const serviceWorker = manifest.background?.service_worker;
if (!serviceWorker) errors.push("缺少 background.service_worker");
if (manifest.background?.type !== "module") errors.push("background.type 应为 module");

if (!manifest.commands || Object.keys(manifest.commands).length === 0) errors.push("缺少 commands");

// Files must exist in dist (content.js is registered dynamically, not listed in the manifest).
const referenced = [serviceWorker, manifest.options_page, "content.js", "options.js"].filter(Boolean);
for (const file of referenced) {
  try {
    await access(join(dist, file));
  } catch {
    errors.push(`引用的文件不存在：${file}`);
  }
}

if (errors.length > 0) {
  console.error("扩展清单校验失败：");
  for (const error of errors) console.error(`  - ${error}`);
  process.exit(1);
}

console.log("manifest.json 校验通过");
