# Spec Delta

## Purpose

定义扩展与本地 `opencode` 终端命令之间的翻译请求通道：通过 Chrome 原生消息与本地主机通信，主机负责批量翻译、排队、缓存、调用 `opencode`、错误返回，以及主机的安装与健康检查。

## ADDED Requirements

### Requirement: 原生消息传输通道

扩展 SHALL 通过 Chrome native messaging 与本地主机进程通信；本地主机进程 SHALL 负责调用本地 `opencode` 终端命令完成翻译。扩展 MUST NOT 依赖任何需要监听网络端口的常驻服务。

#### Scenario: 发送翻译请求
- **WHEN** 扩展需要翻译文本
- **THEN** 扩展通过原生消息将请求发送给本地主机

#### Scenario: 主机不可用
- **WHEN** 本地主机未注册或无法启动
- **THEN** 扩展收到明确的连接失败错误，而不是静默失败

### Requirement: 批量请求与结果对应

本地主机 SHALL 接受包含多个文本片段的请求，并返回与输入片段一一对应、顺序一致且带有相同标识的译文。

#### Scenario: 批量翻译
- **WHEN** 主机收到包含 N 个片段的请求
- **THEN** 返回 N 个译文，每个译文携带与输入片段相同的标识

#### Scenario: 超出消息大小限制
- **WHEN** 单个请求超过单条原生消息允许的大小
- **THEN** 主机或扩展将请求拆分为多条消息，或返回可读错误而不崩溃

### Requirement: 调用 opencode 终端命令

本地主机 SHALL 以非交互方式调用本地 `opencode` 终端命令执行翻译，并从其输出中提取纯译文返回。

#### Scenario: 非交互调用
- **WHEN** 主机处理一次翻译请求
- **THEN** 以非交互方式运行 `opencode`（如 `opencode run`）并获取模型输出

#### Scenario: 文本过大无法作为命令行参数
- **WHEN** 待翻译文本超过命令行参数长度限制
- **THEN** 主机通过标准输入或文件方式传递文本

#### Scenario: 解析结构化输出
- **WHEN** `opencode` 以结构化事件流（如 JSON）输出
- **THEN** 主机提取其中的文本内容并拼接为最终译文

### Requirement: 请求队列与并发控制

本地主机 SHALL 对翻译请求进行排队并按受限并发处理，以避免同时启动过多 `opencode` 进程。

#### Scenario: 突发大量请求
- **WHEN** 短时间内到达大量翻译请求
- **THEN** 请求进入队列按序处理

#### Scenario: 达到并发上限
- **WHEN** 正在处理的请求数达到并发上限
- **THEN** 新请求等待，而不是无限制地并行启动进程

### Requirement: 翻译结果缓存

本地主机 SHALL 缓存相同文本的翻译结果，命中缓存时不再调用模型。

#### Scenario: 命中缓存
- **WHEN** 相同文本再次被请求
- **THEN** 直接返回缓存结果，不调用模型

#### Scenario: 缓存淘汰
- **WHEN** 缓存达到容量上限
- **THEN** 按既定策略淘汰旧条目

### Requirement: 结构化错误返回

本地主机 SHALL 将 `opencode` 的失败情况（未安装、未认证、模型不可用、超时等）以结构化错误返回给扩展。

#### Scenario: opencode 不可用
- **WHEN** 本地未安装或无法执行 `opencode`
- **THEN** 主机返回明确错误信息而非崩溃

#### Scenario: 模型不可用
- **WHEN** 使用的模型不可用（例如区域限制）
- **THEN** 主机返回可读错误，并提示用户更换模型

#### Scenario: 调用超时
- **WHEN** 单次 `opencode` 调用超过设定超时时间
- **THEN** 主机终止该调用并返回超时错误

### Requirement: 主机安装与健康检查

项目 SHALL 提供将本地主机注册到 Chrome 的安装与卸载能力，并 SHALL 支持健康检查以确认主机与 `opencode` 均可用。

#### Scenario: 安装主机
- **WHEN** 用户运行主机安装任务
- **THEN** 生成主机清单文件，并在 Windows 注册表中按名称注册该主机

#### Scenario: 卸载主机
- **WHEN** 用户运行主机卸载任务
- **THEN** 移除对应的注册表项与主机清单

#### Scenario: 健康检查
- **WHEN** 扩展请求健康检查
- **THEN** 主机返回自身状态以及 `opencode` 是否可用与版本信息
