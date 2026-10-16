# 内容脚本行为

内容脚本负责在页面内提取文本、请求翻译、内联渲染并处理动态内容。相关源码在
`extension/src/content/`。

## 翻译单元的选择

脚本用 `TreeWalker` 遍历所有文本节点，对每个文本节点向上找到最近的「翻译单元」元素
（`extension/src/content/units.js` 的 `chooseUnit`）：从 `parentElement` 起，向上寻找第一个
`BLOCK_TAGS` 中的标签。`BLOCK_TAGS` 既包含块级标签（`P`/`DIV`/`LI`/`H1..H6`/`BLOCKQUOTE`/
`ARTICLE`/`SECTION`/`TD`…），也包含常见内联容器（`SPAN`/`A`/`EM`/`STRONG`…），因此会尽量选到最内层的容器。

若元素标签属于 `INLINE_TAGS`，其译文以 `inline-block` 内联展示，否则以块级展示。

## 跳过规则

`extension/src/content/extract.js` 决定哪些内容不翻译，对应单元测试
`extension/test/extract.test.js`：

- **元素级跳过**（`shouldSkipElement`）：`SCRIPT`/`STYLE`/`NOSCRIPT`/`CODE`/`PRE`/`KBD`/
  `TEXTAREA`/`INPUT`/`SELECT`/`BUTTON`/`SVG`/`CANVAS`/`IFRAME`/`VIDEO`… 等；可编辑元素
  （`isContentEditable` 或表单控件）；扩展自身注入的节点（`[data-oc-trans]`、`[data-oc-trans-ui]`）。
- **文本级跳过**（`isTranslatableText`）：长度小于 2、已经是中文（CJK 占比 ≥ 30%）、
  不含拉丁字母（纯数字/符号）、整段就是一个 URL 的文本。

## 双语内联渲染与撤销

`extension/src/content/render.js`：

- `insertTranslation` 在原文元素**之后**插入一个 `<oc-translation>` 兄弟节点，使用
  `attachShadow({ mode: "open" })` 隔离样式（站点 CSS 与译文字体/颜色互不影响），并给原文元素
  标记 `data-oc-done`。
- 原文**不会**被替换或删除。
- `revertAll` 移除所有 `[data-oc-trans]` 节点并清除 `data-oc-done` 标记。
- 对应测试：`extension/test/render.test.js`。

## 去重与动态内容

`extension/src/content/units.js` 的 `collectUnits` 用一个 `WeakMap<Element, string>` 记录每个元素
**上一次处理过的文本哈希**（djb2）：

- 元素已处理且文本哈希未变 → 跳过（不重复翻译）。
- 元素文本发生变化（SPA 复用节点）→ 重新翻译。
- 新出现的元素 → 作为新单元翻译。

内容脚本在翻译开始后启动 `MutationObserver`（`childList` + `subtree`，400ms 去抖），每次
DOM 变化后重新收集新单元并翻译，因此滚动加载的 x.com 时间线会被持续翻译。译文节点没有
light DOM 文本，不会再次被采集，从而避免自触发循环。

对应测试：`extension/test/units.test.js`。

## 触发与反馈

- 工具栏图标 / `Alt+Shift+T`：`toggle-page`，整页翻译或撤销。
- 右键「翻译选中内容」/ `Alt+Shift+S`：翻译选区；无选区时退回区域翻译。
- 右键「翻译此区域」：翻译上次右键所在区域的子树。
- 右键「撤销本页翻译」：撤销。
- 进行中与错误通过右下角状态浮标显示；`badge` 显示主机报告的进度。
- 站点未启用时会提示在设置中启用。
