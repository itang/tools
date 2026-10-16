# 组件与消息协议

## 组件

```
content script (页面内)
      │  chrome.runtime.sendMessage / onMessage
      ▼
service worker (MV3)  ── chrome.runtime.connectNative ──▶  oc-translate-host.exe  ──▶  opencode run
      │  chrome.action / contextMenus / commands                (长度前缀 JSON 帧)
      ▼
options 页（设置 + 诊断）
```

- **content script**：收集页面文本、内联插入译文、监听动态内容、撤销。
- **service worker**：持有原生消息端口，转发请求与进度，注册右键菜单/快捷键/工具栏动作，按设置动态注册内容脚本。
- **native host**（`host/`）：批量调用 `opencode run --format json`，做缓存、排队、错误映射与分帧。
- **options 页**：设置模型/目标语言/批量大小/启用站点，并展示主机与 opencode 状态。

## 帧格式

主机与扩展之间使用 Chrome 原生消息的帧格式：**4 字节小端长度前缀 + UTF-8 JSON**。
主机保证单帧不超过 1 MB，超大的 `translate` 响应会拆成多帧，每帧 `seq` 递增，最后一帧 `last: true`。

## 扩展 ↔ service worker 消息

| `type` | 方向 | 说明 |
| --- | --- | --- |
| `toggle-page` | sw → content | 工具栏点击：已翻译则撤销，否则翻译 |
| `translate-page` | sw → content | 强制翻译整页 |
| `translate-selection` | sw → content | 翻译选中文本（无选中时退回区域翻译） |
| `translate-region` | sw → content | 翻译上次右键所在区域 |
| `revert-page` | sw → content | 撤销整页翻译 |
| `host-request` | content/options → sw | 携带 `{ id, request }`，由 sw 转发给主机 |
| `host-progress` | sw → content | 携带 `{ id, progress:{phase,completed,total} }` |

## 扩展 ↔ 主机请求

```json
{ "id": "uuid", "type": "translate",
  "segments": [{ "index": 0, "text": "Hello world" }],
  "options": { "targetLang": "zh-CN", "model": "opencode/...", "agent": "translator" } }
```

```json
{ "id": "uuid", "type": "health" }
```

## 主机 ↔ 扩展响应

```json
{ "id": "uuid", "seq": 0, "last": false, "type": "progress", "phase": "batch", "completed": 2, "total": 2 }
{ "id": "uuid", "seq": 1, "last": true,  "type": "translate", "translations": [{ "index": 0, "text": "你好，世界" }] }
{ "id": "uuid", "seq": 0, "last": true,  "type": "health", "data": { "host": {}, "opencode": {} } }
{ "id": "uuid", "seq": 0, "last": true,  "type": "error", "error": { "code": "MODEL_UNAVAILABLE", "message": "..." } }
```

扩展侧由 `createFrameAccumulator` 累积非终帧，遇到 `last: true` 得到最终结果。

## 错误码

| 错误码 | 含义 |
| --- | --- |
| `OPENCODE_MISSING` | 无法启动 `opencode` 可执行文件 |
| `AUTH_REQUIRED` | provider 认证失败 |
| `MODEL_UNAVAILABLE` | 模型不可用（如区域限制） |
| `TIMEOUT` | 单次 `opencode` 调用超时 |
| `INCOMPLETE` | 重试后模型仍未返回该片段 |
| `HOST_ERROR` | 其它 opencode 失败或主机内部错误 |
| `BAD_REQUEST` | 请求格式错误或类型未知 |
| `HOST_UNAVAILABLE` | 扩展侧无法连接本地主机（非主机返回，由扩展产生） |

## 主机环境变量

| 变量 | 默认值 | 作用 |
| --- | --- | --- |
| `OPENCODE_BIN` | `opencode` | opencode 可执行文件路径 |
| `OPENCODE_BIN_ARGS` | `[]` | 插入到 `run` 之前的 JSON 参数数组 |
| `TRANSLATE_MODEL` | `opencode/deepseek-v4.1-flash-free` | 请求未带 `model` 时使用的模型；设为空字符串则用 opencode 自身默认模型 |
| `TRANSLATE_TIMEOUT_MS` | `120000` | 单次调用超时 |
| `TRANSLATE_CONCURRENCY` | `2` | 并发上限 |
| `TRANSLATE_CACHE_MAX` | `2000` | 缓存条目上限 |

Chrome 启动主机时会继承 Chrome 进程的环境变量，因此这些变量需要设置为**用户级/系统级**环境变量（例如 `setx`）。
