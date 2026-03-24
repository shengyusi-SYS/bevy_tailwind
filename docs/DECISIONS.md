# bevy_tailwind 技术决策记录

## D001: 响应式断点扩展

- **日期**：2026-03-23
- **背景**：PlayMusic 移动端 UI 适配需要响应式布局，bevy_tailwind 原生仅支持 `hover:` / `focus:` 前缀
- **决策**：扩展 bevy_tailwind 添加 `sm:` / `md:` / `lg:` / `xl:` 响应式断点前缀
- **断点定义**（mobile-first / min-width）：
  - `sm:` → ≥ 640px
  - `md:` → ≥ 768px
  - `lg:` → ≥ 1024px
  - `xl:` → ≥ 1280px
- **实现方式**：
  - 编译期：扩展 `parse_picking_class` + `PickingStyles` 新增 sm/md/lg/xl 字段
  - 运行时：`CurrentBreakpoint` Resource + `update_breakpoint` 系统 + `apply_cascade` 级联应用
  - 级联顺序：base → sm → md → lg → xl → hover/focus
  - 所有现有解析器（node, background, border, text 等）通过 `insert_picking_style!` 宏自动支持
- **限制**：v1 不支持混合前缀（`md:hover:bg-red`），响应式与 hover 作用于同一属性时行为未定义
- **理由**：
  - 复用已有前缀处理架构（编译期分流 + 运行时切换），改动量小（~100 行）
  - 声明式 `tw!("flex-col md:flex-row")` 优于 ECS 手动条件代码
  - 与 Tailwind CSS 标准语法一致
