# Tailwind CSS 类支持分析

> 分析 bevy_tailwind 当前支持的 Tailwind v4 类，以及基于 Bevy 0.18 可新增实现的类。

---

## 1. 当前已支持的 Tailwind 类

### 1.1 布局 (Layout)

| 分类         | 支持的类                                                                                                  | 映射目标             |
| ------------ | --------------------------------------------------------------------------------------------------------- | -------------------- |
| Display      | `flex`, `grid`, `block`, `hidden`                                                                         | `Node.display`       |
| Position     | `relative`, `absolute`                                                                                    | `Node.position_type` |
| Overflow     | `overflow-hidden`, `overflow-visible`, `overflow-scroll`, `overflow-clip`, `overflow-x-*`, `overflow-y-*` | `Node.overflow`      |
| Aspect Ratio | `aspect-auto`, `aspect-square`, `aspect-video`, `aspect-[n/n]`                                            | `Node.aspect_ratio`  |
| Z-Index      | `z-{n}`, `z-auto`, `z-[n]`                                                                                | `ZIndex` 组件        |
| Box Sizing   | `box-border`, `box-content`                                                                               | `Node.box_sizing`    |

### 1.2 Flexbox

| 分类      | 支持的类                                                                   | 映射目标                                        |
| --------- | -------------------------------------------------------------------------- | ----------------------------------------------- |
| Direction | `flex-row`, `flex-col`, `flex-row-reverse`, `flex-col-reverse`             | `Node.flex_direction`                           |
| Wrap      | `flex-wrap`, `flex-nowrap`, `flex-wrap-reverse`                            | `Node.flex_wrap`                                |
| Grow      | `grow`, `grow-0`, `grow-[n]`                                               | `Node.flex_grow`                                |
| Shrink    | `shrink`, `shrink-0`, `shrink-[n]`                                         | `Node.flex_shrink`                              |
| Basis     | `basis-{n}`, `basis-auto`, `basis-full`, `basis-{fraction}`, `basis-[val]` | `Node.flex_basis`                               |
| Shorthand | `flex-1`, `flex-auto`, `flex-initial`, `flex-none`                         | `Node.flex_grow` + `flex_shrink` + `flex_basis` |

### 1.3 Grid

| 分类        | 支持的类                                                                                          | 映射目标                          |
| ----------- | ------------------------------------------------------------------------------------------------- | --------------------------------- |
| Template    | `grid-cols-{n}`, `grid-cols-[...]`, `grid-rows-{n}`, `grid-rows-[...]`                            | `Node.grid_template_columns/rows` |
| Span        | `col-span-{n}`, `col-span-full`, `row-span-{n}`, `row-span-full`                                  | `Node.grid_column/row`            |
| Start/End   | `col-start-{n}`, `col-end-{n}`, `row-start-{n}`, `row-end-{n}`                                    | `Node.grid_column/row`            |
| Auto Flow   | `grid-flow-row`, `grid-flow-col`, `grid-flow-dense`, `grid-flow-row-dense`, `grid-flow-col-dense` | `Node.grid_auto_flow`             |
| Auto Sizing | `auto-cols-auto`, `auto-cols-min`, `auto-cols-max`, `auto-cols-fr`, `auto-rows-*`                 | `Node.grid_auto_columns/rows`     |

### 1.4 对齐 (Alignment)

| 分类            | 支持的类                                                                                                                                                             | 映射目标                                 |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| Justify Content | `justify-start`, `justify-end`, `justify-center`, `justify-between`, `justify-around`, `justify-evenly`, `justify-stretch`                                           | `Node.justify_content`                   |
| Justify Items   | `justify-items-start`, `justify-items-end`, `justify-items-center`, `justify-items-stretch`                                                                          | `Node.justify_items`                     |
| Justify Self    | `justify-self-start`, `justify-self-end`, `justify-self-center`, `justify-self-stretch`, `justify-self-auto`                                                         | `Node.justify_self`                      |
| Align Items     | `items-start`, `items-end`, `items-center`, `items-baseline`, `items-stretch`                                                                                        | `Node.align_items`                       |
| Align Self      | `self-start`, `self-end`, `self-center`, `self-baseline`, `self-stretch`, `self-auto`                                                                                | `Node.align_self`                        |
| Align Content   | `content-start`, `content-end`, `content-center`, `content-between`, `content-around`, `content-evenly`, `content-stretch`                                           | `Node.align_content`                     |
| Place Content   | `place-content-start`, `place-content-end`, `place-content-center`, `place-content-between`, `place-content-around`, `place-content-evenly`, `place-content-stretch` | `Node.align_content` + `justify_content` |
| Place Items     | `place-items-start`, `place-items-end`, `place-items-center`, `place-items-baseline`, `place-items-stretch`                                                          | `Node.align_items` + `justify_items`     |
| Place Self      | `place-self-start`, `place-self-end`, `place-self-center`, `place-self-auto`, `place-self-stretch`                                                                   | `Node.align_self` + `justify_self`       |

### 1.5 间距 (Spacing)

| 分类    | 支持的类                                                                      | 映射目标                      |
| ------- | ----------------------------------------------------------------------------- | ----------------------------- |
| Padding | `p-{n}`, `px-{n}`, `py-{n}`, `pt-{n}`, `pr-{n}`, `pb-{n}`, `pl-{n}`           | `Node.padding`                |
| Margin  | `m-{n}`, `mx-{n}`, `my-{n}`, `mt-{n}`, `mr-{n}`, `mb-{n}`, `ml-{n}`, `m-auto` | `Node.margin`                 |
| Gap     | `gap-{n}`, `gap-x-{n}`, `gap-y-{n}`                                           | `Node.row_gap` / `column_gap` |

### 1.6 尺寸 (Sizing)

| 分类             | 支持的类                                                                             | 映射目标                        |
| ---------------- | ------------------------------------------------------------------------------------ | ------------------------------- |
| Width            | `w-{n}`, `w-full`, `w-screen`, `w-auto`, `w-{fraction}`, `w-svw`, `w-lvw`, `w-[val]` | `Node.width`                    |
| Height           | `h-{n}`, `h-full`, `h-screen`, `h-auto`, `h-{fraction}`, `h-svh`, `h-lvh`, `h-[val]` | `Node.height`                   |
| Size             | `size-{n}`, `size-full`, `size-auto`, `size-[val]`                                   | `Node.width` + `height`         |
| Min Width/Height | `min-w-{n}`, `min-h-{n}`, `min-w-full`, `min-h-full` 等                              | `Node.min_width` / `min_height` |
| Max Width/Height | `max-w-{n}`, `max-h-{n}`, `max-w-full`, `max-h-full` 等                              | `Node.max_width` / `max_height` |

### 1.7 定位 (TRBL)

| 支持的类                                         | 映射目标                     |
| ------------------------------------------------ | ---------------------------- |
| `top-{n}`, `right-{n}`, `bottom-{n}`, `left-{n}` | `Node.top/right/bottom/left` |
| `inset-{n}`, `inset-x-{n}`, `inset-y-{n}`        | `Node.top+right+bottom+left` |

### 1.8 边框 (Border)

| 分类          | 支持的类                                                                                                                   | 映射目标                      |
| ------------- | -------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| Width         | `border`, `border-{n}`, `border-x-{n}`, `border-y-{n}`, `border-t-{n}`, `border-r-{n}`, `border-b-{n}`, `border-l-{n}`     | `Node.border`                 |
| Color         | `border-{color}`, `border-{color}/{opacity}`, `border-transparent`                                                         | `BorderColor` 组件            |
| Radius        | `rounded`, `rounded-{size}`, `rounded-full`, `rounded-none`                                                                | `Node.border_radius`          |
| Radius (方向) | `rounded-t-*`, `rounded-r-*`, `rounded-b-*`, `rounded-l-*`, `rounded-tl-*`, `rounded-tr-*`, `rounded-br-*`, `rounded-bl-*` | `Node.border_radius.{corner}` |

### 1.9 轮廓 (Outline)

| 支持的类             | 映射目标         |
| -------------------- | ---------------- |
| `outline-{n}`        | `Outline.width`  |
| `outline-offset-{n}` | `Outline.offset` |
| `outline-{color}`    | `Outline.color`  |

### 1.10 背景 (Background)

| 支持的类                                               | 映射目标               |
| ------------------------------------------------------ | ---------------------- |
| `bg-{color}`, `bg-{color}/{opacity}`, `bg-transparent` | `BackgroundColor` 组件 |

### 1.11 文字 (Typography)

| 分类       | 支持的类                                                 | 映射目标                  |
| ---------- | -------------------------------------------------------- | ------------------------- |
| Font Size  | `text-xs` ~ `text-9xl`, `text-[{size}px]`                | `TextFont.font_size`      |
| Color      | `text-{color}`, `text-{color}/{opacity}`                 | `TextColor` 组件          |
| Alignment  | `text-left`, `text-center`, `text-right`, `text-justify` | `TextLayout.justify`      |
| Smoothing  | `antialiased`                                            | `TextFont.font_smoothing` |
| Word Break | `break-words`, `break-all`                               | `TextLayout.linebreak`    |

### 1.12 变换 (Transform)

| 支持的类                                              | 映射目标                  |
| ----------------------------------------------------- | ------------------------- |
| `translate-{n}`, `translate-x-{n}`, `translate-y-{n}` | `UiTransform.translation` |
| `scale-{n}`, `scale-x-{n}`, `scale-y-{n}`             | `UiTransform.scale`       |
| `rotate-{n}`                                          | `UiTransform.rotation`    |

### 1.13 交互 (Interaction)

| 支持的类        | 说明               |
| --------------- | ------------------ |
| `hover:{class}` | 鼠标悬浮时应用样式 |
| `focus:{class}` | 获得焦点时应用样式 |

### 1.14 值系统

| 格式                    | 示例                   | 结果           |
| ----------------------- | ---------------------- | -------------- |
| 整数 `{n}`              | `p-4`                  | `16px` (n × 4) |
| 小数 `{n.m}`            | `p-0.5`                | `2px`          |
| `px`                    | `p-px`                 | `1px`          |
| `auto`                  | `m-auto`               | `Val::Auto`    |
| `full`                  | `w-full`               | `100%`         |
| `screen`                | `w-screen`             | `100vw`        |
| `svw`/`svh`/`lvw`/`lvh` | `w-svw`                | `100svw`       |
| 分数                    | `w-1/2`                | `50%`          |
| 任意值 `[val]`          | `w-[200px]`, `w-[50%]` | 精确值         |

---

## 2. 未支持但可在 Bevy 0.18 中实现的 Tailwind 类

### 2.1 🟢 容易实现（Bevy 0.18 已有直接 API 对应）

#### 阴影 (Box Shadow)

Bevy 0.18 新增了 `BoxShadow` 组件，包含 `ShadowStyle` 列表。

| Tailwind 类      | 实现方案                                                                                 | 难度 |
| ---------------- | ---------------------------------------------------------------------------------------- | ---- |
| `shadow-sm`      | `BoxShadow { shadows: vec![ShadowStyle { x: 0, y: 1px, blur: 2px, spread: 0, color }] }` | ⭐   |
| `shadow`         | 默认阴影                                                                                 | ⭐   |
| `shadow-md`      | 中等阴影                                                                                 | ⭐   |
| `shadow-lg`      | 大阴影                                                                                   | ⭐   |
| `shadow-xl`      | 超大阴影                                                                                 | ⭐   |
| `shadow-2xl`     | 最大阴影                                                                                 | ⭐   |
| `shadow-none`    | 空阴影                                                                                   | ⭐   |
| `shadow-{color}` | 自定义阴影颜色                                                                           | ⭐   |
| `shadow-inner`   | 内阴影（需检查 Bevy 是否支持 inset）                                                     | ⭐⭐ |

**实现方式：** 新增独立组件写入路径，类似 `BackgroundColor` 的模式。
**所需文件：** 新建 `macros/src/shadow.rs`，修改 `macros/src/lib.rs` 注册组件。

#### 字体粗细 (Font Weight)

Bevy 0.18 新增了 `FontWeight` 类型。

| Tailwind 类       | 值  | 难度 |
| ----------------- | --- | ---- |
| `font-thin`       | 100 | ⭐   |
| `font-extralight` | 200 | ⭐   |
| `font-light`      | 300 | ⭐   |
| `font-normal`     | 400 | ⭐   |
| `font-medium`     | 500 | ⭐   |
| `font-semibold`   | 600 | ⭐   |
| `font-bold`       | 700 | ⭐   |
| `font-extrabold`  | 800 | ⭐   |
| `font-black`      | 900 | ⭐   |

**实现方式：** 在 `macros/src/text.rs` 中新增解析分支，写入 `TextFont.font_weight`。
**注意：** 需要确认 `FontWeight` 是 `TextFont` 的字段还是独立组件。

#### 行高 (Line Height)

Bevy 0.18 将 `LineHeight` 提取为独立组件。

| Tailwind 类       | 值      | 难度 |
| ----------------- | ------- | ---- |
| `leading-none`    | 1.0     | ⭐   |
| `leading-tight`   | 1.25    | ⭐   |
| `leading-snug`    | 1.375   | ⭐   |
| `leading-normal`  | 1.5     | ⭐   |
| `leading-relaxed` | 1.625   | ⭐   |
| `leading-loose`   | 2.0     | ⭐   |
| `leading-{n}`     | n × 4px | ⭐   |
| `leading-[val]`   | 任意值  | ⭐   |

**实现方式：** 新增 `LineHeight` 独立组件路径，或在 `text.rs` 中解析并写入。
**所需文件：** 修改 `macros/src/text.rs`，`macros/src/lib.rs` 添加组件。

#### 文字装饰 (Text Decoration)

Bevy 0.18 新增了 `Strikethrough` 和 `Underline` 组件。

| Tailwind 类          | 映射                 | 难度 |
| -------------------- | -------------------- | ---- |
| `underline`          | `Underline` 组件     | ⭐   |
| `line-through`       | `Strikethrough` 组件 | ⭐   |
| `no-underline`       | 移除装饰             | ⭐   |
| `decoration-{color}` | 装饰颜色             | ⭐⭐ |
| `decoration-{width}` | 装饰粗细             | ⭐⭐ |

**实现方式：** 新增独立组件写入路径。

#### 滚动条宽度

Bevy 0.18 `Node` 新增了 `scrollbar_width` 字段。

| Tailwind 类      | 映射               | 难度 |
| ---------------- | ------------------ | ---- |
| `scrollbar-thin` | 较窄滚动条         | ⭐   |
| `scrollbar-none` | 隐藏滚动条 (`0px`) | ⭐   |
| `scrollbar-auto` | 默认宽度           | ⭐   |

**实现方式：** 在 `macros/src/node/` 中新增 `scrollbar_width` 属性映射。

#### 全局 Z-Index

Bevy 0.18 有 `GlobalZIndex` 组件。

| Tailwind 类                 | 映射           | 难度 |
| --------------------------- | -------------- | ---- |
| `global-z-{n}` (自定义前缀) | `GlobalZIndex` | ⭐   |

**实现方式：** 扩展 `z_index.rs`，区分 `z-*` (局部) 和 `global-z-*` (全局)。

#### Overflow Clip Margin

Bevy 0.18 `Node` 新增了 `overflow_clip_margin` 字段。

| Tailwind 类                             | 映射                        | 难度 |
| --------------------------------------- | --------------------------- | ---- |
| `overflow-clip-margin-{n}` (自定义前缀) | `Node.overflow_clip_margin` | ⭐⭐ |

### 2.2 🟡 中等难度（需要较多代码但 API 已就绪）

#### 渐变 (Gradients)

Bevy 0.18 新增了完整的渐变系统：`LinearGradient`、`RadialGradient`、`ConicGradient`、`BorderGradient`。

| Tailwind 类      | 映射                         | 难度   |
| ---------------- | ---------------------------- | ------ |
| `bg-linear-to-r` | `LinearGradient` 方向 right  | ⭐⭐⭐ |
| `bg-linear-to-b` | `LinearGradient` 方向 bottom | ⭐⭐⭐ |
| `from-{color}`   | 渐变起始颜色                 | ⭐⭐⭐ |
| `via-{color}`    | 渐变中间颜色                 | ⭐⭐⭐ |
| `to-{color}`     | 渐变结束颜色                 | ⭐⭐⭐ |
| `bg-radial`      | `RadialGradient`             | ⭐⭐⭐ |
| `bg-conic`       | `ConicGradient`              | ⭐⭐⭐ |

**难点：** 渐变需要组合多个类来定义完整效果（方向 + 起始色 + 结束色），解析逻辑较复杂。需要在宏层面实现"状态合并"。
**所需文件：** 新建 `macros/src/gradient.rs`。

#### 不透明度 (Opacity)

| Tailwind 类   | 实现方案                      | 难度 |
| ------------- | ----------------------------- | ---- |
| `opacity-{n}` | 需要映射到某个组件的 alpha 值 | ⭐⭐ |

**难点：** Bevy 没有直接的 opacity 组件，可能需要影响 `BackgroundColor` 和 `TextColor` 的 alpha 通道。

#### 光标 (Cursor)

| Tailwind 类                                          | 难度 |
| ---------------------------------------------------- | ---- |
| `cursor-pointer`, `cursor-default`, `cursor-move` 等 | ⭐⭐ |

**难点：** 需要与 Bevy 的窗口系统交互，不属于纯 UI 样式。

### 2.3 🔴 较难实现或 Bevy 暂不支持

| Tailwind 类                            | 原因                                  |
| -------------------------------------- | ------------------------------------- |
| `transition-*`, `duration-*`, `ease-*` | Bevy UI 没有内建 CSS 过渡系统         |
| `animate-*`                            | 需要 Bevy 动画系统对接，复杂度高      |
| `backdrop-blur-*`                      | Bevy 无后处理/背景模糊 UI 支持        |
| `filter`, `blur-*`, `brightness-*`     | Bevy UI 无滤镜支持                    |
| `ring-*`                               | 需要多层 box-shadow 模拟              |
| `divide-*`                             | 需要子元素选择器，proc macro 难以实现 |
| `space-x-*`, `space-y-*`               | 类似 divide，需要子元素间距逻辑       |
| `truncate`, `line-clamp-*`             | Bevy 文字系统暂无文本截断 API         |
| `columns-*`                            | Bevy 无多列布局                       |
| `table-*`                              | Bevy 无表格布局                       |
| `list-*`                               | Bevy 无列表样式                       |
| `float-*`, `clear-*`                   | Bevy 无浮动布局                       |
| `object-fit`, `object-position`        | Bevy Image 组件的属性，非 Node        |
| `text-wrap`, `text-nowrap`             | 需确认 Bevy 是否支持                  |
| `text-overflow`, `text-ellipsis`       | Bevy 无文本溢出处理                   |
| `whitespace-*`                         | Bevy 文字系统无白空间控制             |

---

## 3. 实现优先级建议

### 第一优先级（高价值、低成本）

| #   | 类                          | 原因                         |
| --- | --------------------------- | ---------------------------- |
| 1   | `shadow-*` 系列             | 高频使用，Bevy 0.18 API 完备 |
| 2   | `font-bold` 等字体粗细      | 极高频，实现简单             |
| 3   | `leading-*` 行高            | 常用排版类，API 已就绪       |
| 4   | `underline`, `line-through` | 实现简单，一对一映射         |

### 第二优先级（中价值、中成本）

| #   | 类                                  | 原因                              |
| --- | ----------------------------------- | --------------------------------- |
| 5   | `bg-linear-to-*` + `from/to-*` 渐变 | 视觉效果显著，但实现复杂          |
| 6   | `opacity-*`                         | 常用，但需设计 alpha 通道传播方案 |
| 7   | `scrollbar-*`                       | 使用频率一般，但实现简单          |

### 第三优先级（低价值或实验性）

| #   | 类                       | 原因                 |
| --- | ------------------------ | -------------------- |
| 8   | `global-z-*`             | 非标准 Tailwind 类名 |
| 9   | `overflow-clip-margin-*` | 使用场景较少         |
| 10  | `cursor-*`               | 需要窗口系统交互     |

---

## 4. 已支持类统计

| 分类          | 已支持数量 (约) | 备注                                                           |
| ------------- | --------------- | -------------------------------------------------------------- |
| Layout        | ~20             | display, position, overflow, aspect-ratio, z-index, box-sizing |
| Flexbox       | ~25             | 方向, wrap, grow, shrink, basis, 缩写                          |
| Grid          | ~40             | template, span, start/end, auto-flow, auto-sizing              |
| Alignment     | ~45             | justify-_, items-_, self-_, content-_, place-\*                |
| Spacing       | ~25             | padding, margin, gap                                           |
| Sizing        | ~30             | width, height, min/max, size                                   |
| Position TRBL | ~15             | top, right, bottom, left, inset                                |
| Border        | ~30             | width, color, radius (全方向)                                  |
| Outline       | ~10             | width, offset, color                                           |
| Background    | ~5              | 颜色                                                           |
| Typography    | ~15             | font-size, color, alignment, smoothing, word-break             |
| Transform     | ~10             | translate, scale, rotate                                       |
| Interaction   | 2 种前缀        | hover:, focus:                                                 |
| **总计**      | **~270+**       | 不含颜色组合和任意值变体                                       |

---

## 5. Tailwind v4 对照参考

Tailwind CSS v4 的完整功能集约有 **500+ 个工具类分类**（不含值变体）。bevy_tailwind 当前覆盖了约 **50-55%** 的核心布局和样式类。

**覆盖率分析：**

- ✅ **布局系统**：~95% 覆盖（Flex + Grid 非常完整）
- ✅ **间距和尺寸**：~90% 覆盖
- ✅ **边框和圆角**：~85% 覆盖
- ✅ **定位**：~90% 覆盖
- 🟡 **排版**：~40% 覆盖（缺少字重、行高、装饰等）
- 🟡 **背景**：~30% 覆盖（仅颜色，缺渐变）
- 🔴 **特效**：~5% 覆盖（缺阴影、滤镜、过渡）
- 🔴 **交互**：~20% 覆盖（仅 hover/focus）

实现第一优先级（shadow、font-weight、leading、underline）后，整体覆盖率可提升至 **~60-65%**。
