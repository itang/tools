# 设置与诊断

设置保存在 `chrome.storage.sync`（`settings` 键），保存后立即生效并在浏览器重启后保留。
源码在 `extension/src/options/` 与 `extension/src/shared/settings.js`。

## 设置项

| 字段 | 默认值 | 说明 |
| --- | --- | --- |
| `backend` | `opencode` | 翻译引擎。`opencode`：经 native host 调用本地 `opencode run`；`youdao`：service worker 直接转发请求到 fanyi.youdao.com 的网页翻译接口，无需安装本地主机。 |
| `model` | `opencode/deepseek-v4.1-flash-free` | 传给 `opencode run --model` 的值，格式 `provider/model#variant`。默认已填一个可用模型（opencode 自身默认模型在本机受区域限制）。仅有道引擎时忽略。 |
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

选项页「诊断」区域的「检查当前引擎可用性」会发送 `health` 请求并展示：

- **opencode 引擎**：主机是否可用、版本、协议；opencode 是否可用、版本，不可用时显示错误码对应的中文提示。
- **youdao 引擎**：有道接口是否可用（实际请求一次 key 接口）。

失败时的提示来自 `extension/src/shared/constants.js` 的 `ERROR_MESSAGES`，例如
`HOST_UNAVAILABLE` 会提示运行 `nu justfile.nu install-host`；`YOUDAO_FAILED` /
`YOUDAO_UNAVAILABLE` 表示有道接口失败或不可达。错误码含义见
[`protocol.md`](./protocol.md)。

## 有道引擎（backend=youdao）

实现位于 `extension/src/shared/youdao.js` + `md5.js`，由 service worker 直接
`fetch`，不经过任何 native host：

1. `POST https://dict-trans.youdao.com/translate/key` 获取 `{token, secretKey}`
   （缓存 10 分钟，鉴权失败时强刷一次）。
2. 每个片段单独 `POST https://dict-trans.youdao.com/webtranslate/sse`，表单参数
   按 key 升序拼 `k=v&...&key=<secretKey>` 后取 MD5 作为 `sign`；SSE 响应中
   `transIncre` 增量拼接为译文。

行为约定：

- 并发 4、单请求超时 30s、LRU 结果缓存 2000 条（按 `语言+原文`）。
- `targetLang` 经 `normalizeLang` 映射为有道语言码（`zh-CN→zh-CHS`、`zh-TW→zh-CHT`、`ja→ja` 等，未知值原样透传）。
- 源语言固定 `auto` 自动检测。
- `[[n]]`/`{{n}}` 结构标记原样送译；有道丢弃标记时渲染层回退为纯文本（见 `docs/content-script.md`）。
- `yduuid` 为安装级随机 id，存于 `chrome.storage.local`。
- manifest 需保留 `https://dict-trans.youdao.com/*` 的 host_permissions（否则 SW 的 fetch 会被 CORS 拦截）。

注意：该接口是网页版逆向的公开接口，无 SLA，签名常量与参数可能随有道改版变化；
失效时可切回 `opencode` 引擎。单元测试：`extension/test/youdao.test.js`
（md5 向量、线上抓包验证过的签名样本、语言映射、backend 默认值）。
