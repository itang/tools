# 端口雷达（PortRadar）

一个 nsh 桌面应用：查询端口被哪个进程占用，按进程名模糊过滤，查看父进程、启动时间、路径与监听地址，复制进程路径，并可在确认后结束进程。界面用 MoonBit 写在 nsh 的 TSX 位置，`App.tsx` 只负责把 nsh 的运行时能力注入进去。

## 环境要求

- Windows 10 1809 及以上 / Windows 11，WebView2 Runtime（Windows 11 自带）
- [nsh](https://nsh.run) 0.5.0（本机在 `D:\dev-env\nsh\bin\nsh.exe`，已加入 `PATH` 时直接用 `nsh`）
- MoonBit 工具链（`moon`）
- 生成安装包使用 Windows 自带的 IExpress，无需额外安装

## 构建与打包

在 `portfinder` 目录下按顺序执行：

```powershell
# 1. 编译 MoonBit（App.tsx 引用 _build/js/release 下的产物，必须用 --release）
cd moonbit
moon build --release
cd ..

# 2. 跑界面测试
nsh test App.test.tsx

# 3. 打 nsh 包：生成便携目录和 zip
nsh package --target windows

# 4. 把便携包封装成安装程序
powershell -ExecutionPolicy Bypass -File installer\make-installer.ps1
```

产物：

| 路径 | 内容 |
| --- | --- |
| `.nsh\out\portradar\` | 便携目录：`portradar.exe` + `nsh\` 资源 |
| `.nsh\out\portradar.zip` | 便携包，解压后直接运行 `portradar.exe` |
| `dist\PortRadarSetup-<版本>.exe` | 安装程序，版本取自 `nsh.json` 的 `version` |

安装包由 `installer\make-installer.ps1` 生成：它把便携 zip 和安装/卸载脚本放进 `IExpress` 的自解压包，输出单个 `setup.exe`。

## 安装与卸载

双击 `dist\PortRadarSetup-1.0.0.exe` 即可安装，**无需管理员权限**：

- 安装位置：`%LOCALAPPDATA%\Programs\PortRadar`
- 开始菜单：`端口雷达`（应用与「端口雷达 卸载」）
- 桌面：`端口雷达` 快捷方式
- 在「设置 → 应用 → 已安装的应用」中登记了卸载项（当前用户）
- 安装完成后自动启动应用

卸载任选其一：

- 「设置 → 应用 → 端口雷达 → 卸载」
- 开始菜单 → `端口雷达` → `端口雷达 卸载`

静默安装（不弹交互，仍按当前用户安装）：

```powershell
dist\PortRadarSetup-1.0.0.exe /Q
```

覆盖安装会先结束正在运行的实例。安装程序未做代码签名，首次运行可能出现 SmartScreen 提示，选择「更多信息 → 仍要运行」即可。

## 目录结构

```
App.tsx              入口：把 nsh:rt、process、nsh/base、nsh/desktop 注入 MoonBit app
assets/icon.png      应用图标（同时嵌入 exe 与安装包快捷方式）
nsh.json             应用名、id、版本、图标、主窗口（尺寸与隐藏标题栏）和能力清单
moonbit/             MoonBit 实现
  app.mbt            界面、查询、过滤与交互
  scan.mbt           进程/端口扫描与命令解析
  nsh.mbt            对 nsh 运行时的绑定
installer/           安装器脚本
  install.ps1        复制文件、写快捷方式与卸载项、启动应用
  uninstall.ps1      删除快捷方式、卸载项与安装目录
  make-installer.ps1 暂存文件并调用 IExpress 生成 setup.exe
dist/                安装包产物（不提交）
```

## 使用说明

- 端口：默认 8080，回车或点「查询」查看占用该端口的进程。
- 名称：按进程名做大小写不敏感的模糊匹配，与端口条件同时生效；清空名称条件后恢复全量列表。
- 清空：把端口和名称两个条件一起清空，端口占用提示栏随之隐藏，列表刷新为无条件全量数据。
- 检查器中的路径可直接拖选复制，右侧「复制」按钮一键写入剪贴板。
- 列表支持按进程、PID、端口、CPU、内存、运行时长排序；「结束」会弹出确认框，确认后才结束进程。
- 窗口：标题、1000×720 初始尺寸和 44pt 隐藏标题栏写在 `nsh.json` 的 `window` 里；应用自绘标题栏，系统的最小化/最大化/关闭按钮留在右侧。
- 权限：`nsh.json` 的 `capabilities.process.execute` 只放行了 `powershell`、`ss`、`lsof`、`ps`、`taskkill`、`kill` 这些扫描与结束进程所需的命令；`capabilities.clipboard` 仅用于复制路径；`capabilities.statusBar` 是 nsh 0.5 在声明了 `capabilities` 时应用默认状态栏样式所需，桌面上不影响界面。

## 开发调试

```powershell
nsh run
```

`nsh run` 带 HMR，但修改 MoonBit 后不会重新执行挂载逻辑，需要重启 `nsh run`。改动 MoonBit 后重新生成安装包的完整步骤见上面的「构建与打包」。
