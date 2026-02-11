# Bevy 版本升级适配指南

> 基于 "Upgrade to Bevy 0.18" commit (eecd279) 的变动分析，总结适配模式与后续升级策略。

---

## 1. 本次升级 (0.17 → 0.18) 变动总结

### 1.1 核心变动：`BorderRadius` 合并进 `Node`

Bevy 0.18 最重要的 UI 变化是将 `BorderRadius` 从独立组件合并为 `Node` 结构体的一个字段（对应 Bevy PR #21781）。

**影响范围：**

| 文件                     | 变动类型    | 说明                                                                                                               |
| ------------------------ | ----------- | ------------------------------------------------------------------------------------------------------------------ |
| `Cargo.toml`             | 版本号      | bevy `0.17` → `0.18.0`                                                                                             |
| `macros/src/lib.rs`      | 删除组件    | 从 `UiComponents` 中移除 `border_radius` 字段及其 quote/path                                                       |
| `macros/src/node/mod.rs` | 新增枚举+宏 | `NodeProp` 新增 `BorderRadius` 变体，新增 `insert_node_border_radius!` 和 `insert_node_border_radius_computed!` 宏 |
| `macros/src/border.rs`   | 宏调用替换  | `insert_computed_style!` / `insert_props!` → `insert_node_border_radius_computed!` / `insert_node_border_radius!`  |
| `macros/src/picking.rs`  | 访问路径    | `border_radius` 组件 → `node, &NodeProp::BorderRadius`                                                             |
| `src/lib.rs`             | 运行时查询  | 移除 `Option<&mut BorderRadius>` 查询参数，border_radius 的 `apply_style` 移入 node 代码块                         |

### 1.2 变动统计

```
0.17 → 0.18:  471 行插入, 约 180 行删除 (较小规模)
0.16 → 0.17: 1861 行插入 (大规模重构)
```

本次升级规模较小，集中在一个组件的迁移。

---

## 2. 识别出的适配模式

### 模式一：独立组件 → Node 字段合并

**触发条件：** Bevy 将某个独立 UI 组件合并为 `Node` 的字段。

**适配步骤清单：**

```
1. [macros/src/lib.rs] 从 UiComponents 结构体中移除该组件字段
2. [macros/src/lib.rs] 从 quote 输出和 path 映射中移除该组件
3. [macros/src/node/mod.rs] 在 NodeProp 枚举中新增对应变体
4. [macros/src/node/mod.rs] 创建专用插入宏:
   - insert_node_{name}!(self, value, priority, props)        // 普通样式
   - insert_node_{name}_computed!(self, value, variant, prop) // hover/focus 计算样式
5. [macros/src/{module}.rs] 替换原有的宏调用:
   - insert_props!       → insert_node_{name}!
   - insert_computed_style!(self, {name}, ...) → insert_node_{name}_computed!(self, ...)
6. [macros/src/picking.rs] 更新 picking 样式:
   - insert_prop!(..., {name}, ...) → insert_prop!(..., node, &NodeProp::{Name}, ...)
7. [src/lib.rs] 运行时适配:
   - 从系统查询中移除 Option<&mut {Component}>
   - 将 apply_style 调用移入 node 处理块
   - 访问路径改为 node.{field_name}.{sub_field}
```

### 模式二：组件字段重命名/类型变更

**触发条件：** Bevy 修改了某个组件的字段名或类型（如 `BorderRect` 的字段从 `left/right/top/bottom` 变为 `min_inset`/`max_inset` Vec2）。

**适配步骤清单：**

```
1. 查阅 Bevy Migration Guide 确认字段映射关系
2. [macros/src/node/mod.rs] 或相关模块中更新字段访问路径
3. [src/lib.rs] 更新运行时的字段赋值
4. 如果类型变更 (如 Val → Vec2)，还需更新值构造逻辑
```

### 模式三：组件拆分 / 提取

**触发条件：** Bevy 将一个组件中的功能拆分为独立组件（如 `LineHeight` 从 `TextFont` 中提取为独立组件）。

**适配步骤清单：**

```
1. [macros/src/lib.rs] 在 UiComponents 中添加新组件字段
2. [macros/src/lib.rs] 添加 quote 输出和 path 映射
3. [macros/src/{module}.rs] 更新解析代码，改为写入新组件
4. [src/lib.rs] 在运行时查询中添加新组件
```

### 模式四：纯版本号更新

**触发条件：** Bevy API 无破坏性变化。

**适配步骤：** 仅更新 `Cargo.toml` 中的版本号。

---

## 3. 升级操作 SOP（标准流程）

### Step 1：准备阶段

```bash
# 1. 阅读 Bevy Release Notes
#    https://bevy.org/news/bevy-{version}/
# 2. 阅读 Migration Guide
#    https://bevy.org/learn/migration-guides/{old}-to-{new}/
# 3. 重点关注以下章节：
#    - UI 相关变动
#    - bevy_ui 模块变动
#    - Component 增删改
#    - Node / Style 结构变动
```

### Step 2：分类变动

将所有 UI 相关变动按上述四种模式分类：

| 变动                         | 模式   | 影响模块             |
| ---------------------------- | ------ | -------------------- |
| 例：BorderRadius → Node 字段 | 模式一 | border, picking, lib |
| 例：LineHeight 独立          | 模式三 | text, lib            |
| ...                          | ...    | ...                  |

### Step 3：逐项适配

按以下优先级处理：

1. **模式四** — 先更新 `Cargo.toml` 版本号
2. **模式二** — 字段重命名（简单查找替换）
3. **模式一** — 组件合并进 Node（需要结构性改动）
4. **模式三** — 组件拆分（需要新增代码）

### Step 4：验证

```bash
# 编译检查
cargo check

# 运行示例
cargo run --example playground
cargo run --example grid

# 运行测试
cargo test
```

---

## 4. 架构关键点说明

### 4.1 代码结构

```
macros/src/
├── lib.rs              # UiComponents 定义、组件注册表
├── node/
│   └── mod.rs          # NodeProp 枚举、Node 字段插入宏
├── border.rs           # border + border-radius 解析
├── picking.rs          # hover/focus 交互样式
├── background.rs       # bg-{color} 解析
├── text.rs             # 文字样式解析
├── outline.rs          # outline 样式解析
├── transform.rs        # transform 样式解析
├── z_index.rs          # z-index 解析
└── utils/              # 颜色、Val、quote 工具
src/
└── lib.rs              # 运行时 apply_style 系统
```

### 4.2 三层写入机制

bevy_tailwind 有三种不同的样式写入路径：

1. **独立组件写入** — 通过 `UiComponents` 的组件字段（如 `BackgroundColor`, `TextColor`）
2. **Node 字段写入** — 通过 `NodeProp` 枚举访问 `Node` 的嵌套字段（如 `node.border_radius.top_left`）
3. **Picking 交互写入** — 通过 `insert_prop!` 宏注册 hover/focus 时的运行时样式变更

**趋势：** Bevy 倾向于将更多属性合并进 `Node`，所以写入路径 2 会越来越常用。

### 4.3 `NodeProp` 枚举的作用

```rust
pub enum NodeProp {
    Display,
    PositionType,
    // ... 所有 Node 直接字段
    BorderRadius,  // 0.18 新增：原独立组件
}
```

`NodeProp` 是 bevy_tailwind 管理所有 `Node` 字段的核心枚举。每次 Bevy 将组件合并进 `Node`，就需要在此枚举中添加对应变体。

---

## 5. 未来版本预判

### 5.1 可能被合并进 Node 的组件

根据 Bevy 的发展趋势，以下组件未来可能被合并进 `Node`：

| 组件              | 可能性 | 准备方案                                       |
| ----------------- | ------ | ---------------------------------------------- |
| `Outline`         | 中     | 参考 BorderRadius 的迁移模式                   |
| `BackgroundColor` | 低     | 目前仍是独立组件，但 Bevy 已引入 Gradient 系统 |
| `BoxShadow`       | 低     | 0.18 新增为独立组件，短期内不太可能合并        |
| `ZIndex`          | 低     | 功能简单，可能保持独立                         |

### 5.2 可能的 API 重构

- **`Val` 类型变化**：Bevy 0.18 引入了 `Val2`（Vec2 化的 Val），未来 spacing/sizing 的 API 可能统一使用 `Val2`
- **Gradient 系统**：`LinearGradient`、`RadialGradient`、`ConicGradient` 是全新的组件，API 可能在后续版本调整
- **Text 系统**：0.18 引入了 `FontWeight`、`Strikethrough`、`Underline` 等独立组件，架构可能继续演化

### 5.3 建议的防御性措施

1. **保持宏抽象层**：所有属性写入都通过宏间接完成，便于批量修改
2. **关注 Bevy 的 RFC/PR**：特别是带有 `A-UI` 标签的 PR
3. **维护 NodeProp 枚举**：这是适配核心，确保所有 Node 字段变动在此集中管理
4. **每次升级前检查 `Node` struct 文档**：`https://docs.rs/bevy/{version}/bevy/ui/struct.Node.html`

---

## 6. 快速参考：升级 Checklist

- [ ] 阅读 Release Notes 和 Migration Guide
- [ ] 更新 `Cargo.toml` 中的 bevy 版本
- [ ] `cargo check` 查看编译错误
- [ ] 根据错误信息分类变动模式
- [ ] 对照本文档的模式步骤逐项修改
- [ ] 检查 `Node` struct 是否有新增/删除字段
- [ ] 检查是否有新的独立组件可以支持
- [ ] 更新 `NodeProp` 枚举（如有需要）
- [ ] 更新 `UiComponents` 结构（如有需要）
- [ ] 运行 `cargo check` 确认编译通过
- [ ] 运行示例程序验证
- [ ] 运行测试
- [ ] 更新文档
