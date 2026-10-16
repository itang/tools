# 安装与使用

opencode 网页翻译由两部分组成：一个 Chrome（MV3）扩展，以及一个把扩展与本地
`opencode` 命令连接起来的原生消息主机。

## 前置条件

- Chrome 或其它 Chromium 内核浏览器
- 本地 `opencode` CLI 可用（在终端中 `opencode --version` 有输出），且至少有一个可用模型
- [Bun](https://bun.sh)（用于构建主机与扩展）
- [Nushell](https://www.nushell.sh)（用于运行 `justfile.nu` 任务）

## 安装步骤

在仓库根目录依次执行：

```sh
nu justfile.nu build-host     # 编译主机为单个可执行文件到 host/dist
nu justfile.nu install-host   # 复制可执行文件到 D:/dev-env/bin、写入主机清单并在注册表中注册
nu justfile.nu build          # 构建扩展到 extension/dist
```

然后在 Chrome 中：

1. 打开 `chrome://extensions`，开启右上角「开发者模式」。
2. 点击「加载已解压的扩展程序」，选择 `extension/dist` 目录。
3. 打开扩展的「选项」页，点击「检查本地主机与 opencode」，确认主机与 opencode 均显示可用。

## 使用

| 操作 | 触发方式 |
| --- | --- |
| 整页翻译 / 撤销 | 点击工具栏图标，或按 `Alt+Shift+T`（切换） |
| 翻译选中文本 | 选中文本 → 右键「翻译选中内容」，或按 `Alt+Shift+S` |
| 翻译某个区域 | 在目标区域上右键 → 「翻译此区域」 |
| 撤销本页翻译 | 在页面空白处右键 → 「撤销本页翻译」 |
| 持续翻译 | 翻译开启后，滚动加载的新内容（如 x.com 时间线）会自动翻译 |

译文以中英对照方式内联显示：原文保留，译文紧随其后，并使用独立样式（Shadow DOM）避免被页面样式影响。

## 卸载

```sh
nu justfile.nu uninstall-host   # 移除注册表项与主机清单
```

再在 `chrome://extensions` 中移除扩展。

## 隐私说明

页面文本**只**通过本地 `opencode run` 命令发送给你在 opencode 中已经配置的 provider。
扩展本身不向任何其它服务器发送数据，也没有遥测或账号体系。缓存仅保存在主机进程内存中。

## 故障排查

| 现象 | 处理 |
| --- | --- |
| 诊断显示「本地主机不可用」 | 确认已运行 `install-host`；重新 `build-host` 后需要再运行 `install-host`，它会把最新可执行文件复制到 `D:\dev-env\bin\oc-translate-host.exe`，清单里的路径也随之更新。 |
| 诊断显示 `OPENCODE_MISSING` | 主机找不到 `opencode`。设置用户环境变量 `OPENCODE_BIN` 指向 `opencode.exe`（例如 `setx OPENCODE_BIN "D:\dev-env\bin\opencode.exe"`），然后重启 Chrome。 |
| 提示 `MODEL_UNAVAILABLE`（区域限制） | 默认模型为 `opencode/deepseek-v4.1-flash-free`（opencode 自身默认模型在本机受区域限制）。如需更换，在选项页「模型」中填写其它可用模型。 |
| 添加了站点但页面仍不翻译 | 添加站点时需在弹窗中授权该站点；也可能是该标签页是授权前打开的，刷新一次即可。 |
| 想在本地静态页面上试用 | 用任意 http 服务器托管 `extension/test/fixtures/` 下的页面（例如 `bunx serve extension/test/fixtures`），在选项页把 `localhost` 加为启用站点并授权。 |
