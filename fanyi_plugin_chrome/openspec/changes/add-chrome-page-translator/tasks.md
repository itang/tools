# Tasks

## 1. 脚手架与构建工具链

- [x] 1.1 建立扩展与主机的目录结构（`extension/`、`host/`、`docs/`），放置 `extension/manifest.json`、主机入口与各自的 `package.json`；验证：文件与目录存在，`openspec validate` 仍通过。
- [x] 1.2 配置主机用 Bun 编译为单文件可执行（`bun build --compile` 或 Deno `compile` 作为备选），在 `justfile.nu` 添加 `main build-host` 任务；验证：运行该任务后生成 `.exe`，以空 stdin 启动能正常退出且不报错。
- [x] 1.3 将扩展与主机的 fmt/lint/test 接入 nushell runner（`main check`、`main test`）；验证：两条命令均可运行且返回成功。

## 2. 主机：翻译引擎

- [x] 2.1 实现 `opencode run --format json` 调用封装：经 stdin 传入文本、解析 JSONL 拼接 `text` 片段、支持 `--model`/`--agent` 覆盖；验证：单元测试用假 `opencode` 脚本断言拼接结果与参数传递。
- [x] 2.2 实现批量契约：发送 `[{index,text}]` 并要求返回 `index→translation` 的 JSON，校验一一对应，缺失或多余项按策略重试或标记；验证：单元测试覆盖整齐对齐、缺失项、乱序返回三种情况。
- [x] 2.3 实现请求队列与并发上限（默认 2）；验证：单元测试断言瞬时并发不超过上限且结果按请求顺序返回。
- [x] 2.4 实现结果缓存（键含目标语言、模型、归一化文本哈希）与容量淘汰；验证：单元测试断言命中时不再调用后端、超限时淘汰最旧条目。
- [x] 2.5 实现单次调用超时与子进程终止；验证：单元测试用挂起的假进程断言返回 `TIMEOUT`。
- [x] 2.6 实现错误映射（`OPENCODE_MISSING`/`AUTH_REQUIRED`/`MODEL_UNAVAILABLE`/`TIMEOUT`/`HOST_ERROR`）；验证：单元测试分别针对可执行文件缺失、区域受限错误输出、非零退出断言对应错误码。
- [x] 2.7 在 `host/README.md` 文档化翻译引擎与错误码；验证：文档中的示例命令按原文可运行并得到预期结果。

## 3. 主机：协议与安装

- [x] 3.1 实现 stdio 长度前缀 JSON 帧收发，遵守 host→extension ≤ 1 MB 限制并对超限响应分帧；验证：单元测试覆盖正常收发与超限拆分。
- [x] 3.2 实现 `health` 请求，返回主机状态与 `opencode` 可用性及版本；验证：手动向主机发送 health 帧，收到包含 opencode 版本的结果。
- [x] 3.3 实现主机清单生成与 Windows 注册表注册/注销的 nushell 任务（`main install-host`、`main uninstall-host`）；验证：安装后注册表项与清单文件存在，卸载后两者被移除。
- [x] 3.4 在扩展 `manifest.json` 写入固定 `key` 以获得稳定扩展 ID，并据此生成主机清单的 `allowed_origins`；验证：Chrome 加载扩展后的 ID 与清单 `allowed_origins` 一致。
- [x] 3.5 在 `docs/setup.md` 文档化从零安装步骤与隐私说明（页面文本仅经用户配置的 `opencode` provider）；验证：按文档从零安装可成功连上主机。

## 4. 扩展：骨架与服务工作者

- [x] 4.1 编写 MV3 `manifest.json`（权限：`nativeMessaging`、`storage`、`scripting`、`contextMenus`、`activeTab`，并配置 x.com 的 host 匹配与 `commands`）；验证：Chrome「加载已解压的扩展程序」无报错。
- [ ] 4.2 实现 service worker 的原生端口连接、断线重连与会话保持；验证：发起一次翻译可拉起主机进程，手动结束主机后 SW 能重连成功。
- [x] 4.3 实现 content ↔ service worker ↔ 主机的批量请求与进度事件转发；验证：单元测试或手动断言多批请求按批返回并将进度事件转发到内容脚本。
- [ ] 4.4 注册上下文菜单与键盘快捷键，向内容脚本派发整页/区域翻译动作；验证：手动验证右键菜单与快捷键均触发对应的翻译消息。
- [x] 4.5 在 `docs/protocol.md` 文档化扩展各组件与消息协议；验证：协议字段与实现逐项核对一致。

## 5. 扩展：内容脚本与页面翻译

- [x] 5.1 实现文本提取与跳过规则（`code`/`pre`/`script`/`style`、可编辑区域、扩展自身节点、纯 URL/数字/符号、已是中文的文本）；验证：单元测试逐类覆盖跳过规则。
- [x] 5.2 实现基于 Shadow DOM 隔离的中英双语内联插入，使用 `data-oc-trans`/`data-oc-done` 标记；验证：在带强样式的测试页插入译文不破坏原有样式与布局。
- [x] 5.3 实现撤销：移除全部插入节点并清理标记；验证：单元测试与手动断言撤销后 DOM 与翻译前一致。
- [x] 5.4 实现 MutationObserver 动态翻译，按归一化文本哈希去重；验证：在模拟无限滚动页面上断言新内容被自动翻译且同一文本不重复翻译。
- [ ] 5.5 实现进行中/错误状态 UI（进度、局部失败提示、本地服务不可用提示）；验证：手动断言状态可见且单批失败不影响其余内容继续翻译。
- [x] 5.6 在 `docs/content-script.md` 文档化内容脚本行为与跳过规则；验证：文档与其对应的 5.1 测试用例一致。

## 6. 扩展：设置与诊断

- [x] 6.1 实现选项页：模型/代理覆盖、目标语言（默认 zh-CN）、批量大小、按站点启用（x.com 默认启用）；验证：单元测试或手动断言各字段可正确读写。
- [ ] 6.2 通过 `chrome.storage` 持久化设置；验证：手动修改设置后重启浏览器，设置仍生效。
- [x] 6.3 实现诊断面板，调用主机 `health` 展示主机与 `opencode` 状态并在失败时给出修复指引；验证：主机不可用时展示失败原因与安装任务提示。
- [x] 6.4 在 `docs/settings.md` 文档化各设置项与诊断含义；验证：文档条目与选项页字段逐一对应。

## 7. 端到端集成验证

- [ ] 7.1 在 x.com 验证整页翻译：可见时间线内容以内联中文展示，滚动加载的新内容被自动翻译且不重复；验证：手动观察并记录结果。
- [ ] 7.2 验证区域翻译：选中文本与选中区域两种方式均只影响所选范围；验证：手动断言范围外内容未变化。
- [ ] 7.3 验证错误路径：使 `opencode` 不可用或选用区域受限模型，界面显示 `OPENCODE_MISSING`/`MODEL_UNAVAILABLE` 提示；验证：手动复现两类错误。
- [ ] 7.4 验证撤销与站点开关：撤销后页面恢复原状，禁用 x.com 后不再翻译；验证：手动断言两者行为。
- [x] 7.5 运行 `openspec validate --strict` 与 `main check`；验证：两条命令均通过。

## Workflow follow-up

- Archive the change after the project's review requirements are satisfied.
- Verify the archived result.
