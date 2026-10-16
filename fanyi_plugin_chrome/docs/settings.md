# 设置与诊断

设置保存在 `chrome.storage.sync`（`settings` 键），保存后立即生效并在浏览器重启后保留。
源码在 `extension/src/options/` 与 `extension/src/shared/settings.js`。

## 设置项

| 字段 | 默认值 | 说明 |
| --- | --- | --- |
| `model` | `opencode/deepseek-v4.1-flash-free` | 传给 `opencode run --model` 的值，格式 `provider/model#variant`。默认已填一个可用模型（opencode 自身默认模型在本机受区域限制）。 |
| `agent` | 空 | 传给 `opencode run --agent` 的值。留空则不指定。 |
| `targetLang` | `zh-CN` | 目标语言，写入翻译提示。 |
| `batchSize` | `20` | 每批提交的文本片段数（1–200）。越大越快但单次请求越大。 |
| `sites` | `{ "x.com": true, "twitter.com": true }` | 启用站点表，键为主机名，值为是否启用。 |

### 启用站点

- `x.com` 与 `twitter.com` 默认启用。
- 在选项页「启用站点」输入主机名并「添加站点」，浏览器会请求该站点的访问权限；授权后
  内容脚本会通过 `chrome.scripting.registerContentScripts` 动态注册到该站点。
- 取消勾选即可禁用；禁用后该站点不再注入内容脚本，也不会翻译。

对应单元测试：`extension/test/settings.test.js`（默认值、字段校验、启用站点、match pattern 生成）。

## 诊断

选项页「诊断」区域的「检查本地主机与 opencode」会向主机发送 `health` 请求并展示：

- **主机**：是否可用、主机版本、协议版本。
- **opencode**：是否可用、版本；不可用时显示错误码对应的中文提示。

失败时的提示来自 `extension/src/shared/constants.js` 的 `ERROR_MESSAGES`，例如
`HOST_UNAVAILABLE` 会提示运行 `nu justfile.nu install-host`。错误码含义见
[`protocol.md`](./protocol.md)。
