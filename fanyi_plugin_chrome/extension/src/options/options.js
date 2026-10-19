/** Options page: settings form plus host/opencode diagnostics. */

import { messageForCode } from "../shared/constants.js";
import { MSG } from "../shared/messages.js";
import { loadSettings, saveSettings } from "../shared/settings.js";

const els = {
  form: document.getElementById("settings"),
  backend: document.getElementById("backend"),
  model: document.getElementById("model"),
  agent: document.getElementById("agent"),
  targetLang: document.getElementById("targetLang"),
  batchSize: document.getElementById("batchSize"),
  sites: document.getElementById("sites"),
  newSite: document.getElementById("newSite"),
  addSite: document.getElementById("addSite"),
  saved: document.getElementById("saved"),
  check: document.getElementById("check"),
  status: document.getElementById("status"),
};

let savedTimer = null;
function flash(text) {
  els.saved.textContent = text;
  clearTimeout(savedTimer);
  savedTimer = setTimeout(() => {
    els.saved.textContent = "";
  }, 2500);
}

function requestHost(request) {
  return new Promise((resolve) => {
    chrome.runtime.sendMessage(
      { type: MSG.HOST_REQUEST, id: crypto.randomUUID(), request },
      (response) => {
        if (chrome.runtime.lastError) {
          resolve({ error: { code: "HOST_UNAVAILABLE", message: chrome.runtime.lastError.message } });
        } else {
          resolve(response || { error: { code: "HOST_ERROR", message: "空响应" } });
        }
      },
    );
  });
}

function renderSites(sites) {
  els.sites.textContent = "";
  for (const [host, enabled] of Object.entries(sites)) {
    const label = document.createElement("label");
    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.checked = enabled === true;
    checkbox.addEventListener("change", () => setSite(host, checkbox.checked));
    label.append(checkbox, document.createTextNode(` ${host}`));
    els.sites.append(label);
  }
}

function normalizeHost(value) {
  return String(value || "")
    .trim()
    .toLowerCase()
    .replace(/^https?:\/\//, "")
    .replace(/\/.*$/, "");
}

async function setSite(host, enabled) {
  if (enabled) {
    const granted = await chrome.permissions
      .request({ origins: [`*://${host}/*`] })
      .catch(() => false);
    if (!granted) {
      flash("未获得该站点权限");
      await render();
      return;
    }
  }
  const settings = await loadSettings();
  await saveSettings({ sites: { ...settings.sites, [host]: enabled } });
  flash(enabled ? `已启用 ${host}` : `已禁用 ${host}`);
}

async function render() {
  const settings = await loadSettings();
  els.backend.value = settings.backend;
  els.model.value = settings.model;
  els.agent.value = settings.agent;
  els.targetLang.value = settings.targetLang;
  els.batchSize.value = settings.batchSize;
  renderSites(settings.sites);
}

els.form.addEventListener("submit", async (event) => {
  event.preventDefault();
  await saveSettings({
    backend: els.backend.value,
    model: els.model.value,
    agent: els.agent.value,
    targetLang: els.targetLang.value,
    batchSize: Number(els.batchSize.value),
  });
  await render();
  flash("已保存");
});

els.addSite.addEventListener("click", async () => {
  const host = normalizeHost(els.newSite.value);
  if (!host) return;
  els.newSite.value = "";
  await setSite(host, true);
  await render();
});

els.check.addEventListener("click", async () => {
  els.status.textContent = "检查中…";
  const settings = await loadSettings();
  const result = await requestHost({ type: "health" });
  if (result.error) {
    const label = settings.backend === "youdao" ? "有道接口不可用" : "本地主机不可用";
    els.status.textContent = `${label}：${messageForCode(result.error.code)}\n${result.error.message ?? ""}`;
    return;
  }
  if (settings.backend === "youdao") {
    els.status.textContent = `有道接口：${result.health?.youdao?.ok ? "可用" : "不可用"}`;
    return;
  }
  const { host, opencode } = result.health;
  const lines = [
    `主机：${host.ok ? "可用" : "不可用"}（版本 ${host.version}，协议 ${host.protocol}）`,
    `opencode：${opencode.ok ? `可用（${opencode.version || "未知版本"}）` : `不可用（${messageForCode(opencode.error)}）`}`,
  ];
  els.status.textContent = lines.join("\n");
});

render();
