# Spec Delta

## Purpose

定义扩展的配置与诊断能力：模型/代理覆盖、目标语言、按站点启用状态、翻译行为参数、设置的持久化，以及本地主机与 `opencode` 的状态展示。

## ADDED Requirements

### Requirement: 模型与代理配置

用户 SHALL 能够配置翻译所使用的模型与代理；未配置时扩展 SHALL 使用 `opencode` 的默认配置。

#### Scenario: 使用默认模型
- **WHEN** 用户未设置模型
- **THEN** 翻译请求使用 `opencode` 的默认模型

#### Scenario: 覆盖模型
- **WHEN** 用户设置了模型覆盖（如 `provider/model#variant`）
- **THEN** 后续翻译请求使用该模型

### Requirement: 目标语言配置

用户 SHALL 能够配置目标语言，默认目标语言为中文（zh-CN）。

#### Scenario: 默认目标语言
- **WHEN** 用户未配置目标语言
- **THEN** 翻译结果为简体中文

#### Scenario: 更改目标语言
- **WHEN** 用户将目标语言更改为其他语言
- **THEN** 后续翻译使用新的目标语言

### Requirement: 按站点启用与禁用

用户 SHALL 能够按站点启用或禁用扩展，且 x.com SHALL 默认处于启用状态。

#### Scenario: 默认对 x.com 启用
- **WHEN** 用户访问 x.com 且未做任何站点配置
- **THEN** 该站点默认可进行翻译

#### Scenario: 禁用站点
- **WHEN** 用户在配置中禁用某个站点
- **THEN** 该站点上扩展不进行翻译

#### Scenario: 启用站点
- **WHEN** 用户在配置中启用某个此前被禁用的站点
- **THEN** 该站点上扩展恢复可翻译

### Requirement: 翻译行为参数

用户 SHALL 能够调整影响翻译行为与性能的参数，例如单批处理的文本片段数量。

#### Scenario: 调整批量大小
- **WHEN** 用户修改批量大小设置
- **THEN** 后续翻译按新的批量大小组织请求

### Requirement: 设置持久化

扩展 SHALL 持久化保存用户设置，并在浏览器重启后保持有效。

#### Scenario: 重启后保留设置
- **WHEN** 用户保存设置后重启浏览器
- **THEN** 之前保存的设置仍然生效

### Requirement: 状态与诊断展示

扩展 SHALL 展示本地主机与 `opencode` 的连接状态，并在失败时给出可操作的修复指引。

#### Scenario: 查看连接状态
- **WHEN** 用户打开设置页的诊断区域
- **THEN** 显示本地主机与 `opencode` 是否可用

#### Scenario: 失败时给出指引
- **WHEN** 本地主机不可用
- **THEN** 显示失败原因并提示运行主机安装任务
