# 内容脚本行为

内容脚本负责在页面内提取文本、请求翻译、内联渲染并处理动态内容。相关源码在
`extension/src/content/`。

## 翻译单元的选择

脚本用 `TreeWalker` 遍历文本节点，对每个文本节点向上找到最近的「翻译单元」元素
（`extension/src/content/units.js` 的 `chooseUnit`）：

- 优先取最近的**块级**祖先（`BLOCK_TAGS`：`P`/`DIV`/`LI`/`TD`/`TH`/`BLOCKQUOTE`/`H1..H6`/
  `ARTICLE`/`SECTION`/`FIGCAPTION`/`DD`/`DT`/`SUMMARY`）；
- 若没有任何块级祖先（裸内联文本），退回最外层内联容器（`INLINE_TAGS`）。

块内的内联容器（`SPAN`/`B`/`EM`/`A`/`CODE`…）不再各自成为单元，而是作为该块的**结构标记**
参与翻译（见下节），因此一个块只翻译一次，不会重复。

若单元标签属于 `INLINE_TAGS`，其译文以 `inline-block` 内联展示，否则以块级展示。

## 结构标记（保留原文格式）

`extension/src/content/markup.js` 在发送前把块级单元序列化成保留内联结构的字符串：

- `[[n]] … [[/n]]`：第 `n` 个可翻译内联元素（`<b>`/`<em>`/`<a>`/`<span>`…），内部文本参与翻译；
- `{{n}}`：第 `n` 个原样保留的内联节点（如 `<code>`/`<pre>`），不翻译；
- `{{br}}`：换行（`<br>`）。

嵌套的块级元素**不**纳入内联标记：它们是各自的翻译单元，因此段落/列表项/表格单元格各自成段，
父单元不会重复翻译子块文本。若文本本身已含有 `[[` 或 `{{`，该单元退回纯文本发送。

主机提示词要求模型逐字保留这些标记及其顺序；渲染时 `render.js` 的 `parseMarked` 依据标记，
用原始元素（连同 `class`/`style`/`href` 等属性）克隆出结构一致的 DOM。若模型丢弃或打乱了标记，
自动退回纯文本渲染，译文不会丢失。

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

- `insertTranslation` 给原文元素标记 `data-oc-done`，并渲染一个 `<oc-translation>` 节点：
  - **块级单元**：译文节点追加在单元**内部**（`element.append`），从而继承列表编号、单元格对齐、
    缩进等块级布局；
  - **内联单元**：作为 `inline-block` 兄弟节点插入原文**之后**。
- 译文内容位于 `attachShadow({ mode: "open" })` 内，隔离站点 CSS；影子树中按结构标记还原内联结构
  （`b`/`em`/`a`/`br`…），译文的排版与原文结构保持一致。
- 原文**不会**被替换或删除。
- `revertAll` 移除所有 `[data-oc-trans]` 节点并清除 `data-oc-done` 标记。
- 对应测试：`extension/test/render.test.js`、`extension/test/markup.test.js`。

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
