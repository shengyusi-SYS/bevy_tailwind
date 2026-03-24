use bevy::prelude::*;
use bevy::text::LineHeight;
use bevy::window::PrimaryWindow;
pub use bevy_tailwind_macro::tw;

/// Tailwind CSS 响应式断点（mobile-first / min-width）
#[derive(Resource, Default, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum CurrentBreakpoint {
    /// < 640px
    #[default]
    Base,
    /// ≥ 640px
    Sm,
    /// ≥ 768px
    Md,
    /// ≥ 1024px
    Lg,
    /// ≥ 1280px
    Xl,
}

pub struct TailwindPlugin;

impl Plugin for TailwindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBreakpoint>()
            .add_systems(
                PostUpdate,
                (
                    update_breakpoint,
                    apply_interaction_style.after(update_breakpoint),
                    apply_responsive_on_resize
                        .after(update_breakpoint)
                        .run_if(resource_changed::<CurrentBreakpoint>),
                ),
            );
    }
}

fn update_breakpoint(
    window: Query<&Window, With<PrimaryWindow>>,
    mut breakpoint: ResMut<CurrentBreakpoint>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    let width = window.width();
    let new = if width >= 1280.0 {
        CurrentBreakpoint::Xl
    } else if width >= 1024.0 {
        CurrentBreakpoint::Lg
    } else if width >= 768.0 {
        CurrentBreakpoint::Md
    } else if width >= 640.0 {
        CurrentBreakpoint::Sm
    } else {
        CurrentBreakpoint::Base
    };
    if *breakpoint != new {
        *breakpoint = new;
    }
}

/// Apply cascade: base → responsive (sm→md→lg→xl) → interaction (hover/focus)
fn apply_cascade(
    picking_styles: &PickingStyles,
    breakpoint: &CurrentBreakpoint,
    interaction: &Interaction,
    node: Option<&mut Node>,
    background_color: Option<&mut BackgroundColor>,
    z_index: Option<&mut ZIndex>,
    border_color: Option<&mut BorderColor>,
    outline: Option<&mut Outline>,
    text_font: Option<&mut TextFont>,
    text_layout: Option<&mut TextLayout>,
    text_color: Option<&mut TextColor>,
    ui_transform: Option<&mut UiTransform>,
    box_shadow: Option<&mut BoxShadow>,
    line_height: Option<&mut LineHeight>,
) {
    // Collect active styles in cascade order
    let mut styles: Vec<&PickingStyle> = vec![&picking_styles.base];
    if *breakpoint >= CurrentBreakpoint::Sm {
        styles.push(&picking_styles.sm);
    }
    if *breakpoint >= CurrentBreakpoint::Md {
        styles.push(&picking_styles.md);
    }
    if *breakpoint >= CurrentBreakpoint::Lg {
        styles.push(&picking_styles.lg);
    }
    if *breakpoint >= CurrentBreakpoint::Xl {
        styles.push(&picking_styles.xl);
    }
    match interaction {
        Interaction::Hovered => styles.push(&picking_styles.hover),
        Interaction::Pressed => styles.push(&picking_styles.focus),
        Interaction::None => {}
    }

    // Apply all styles in order — later styles override earlier ones
    if let Some(node) = node {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(
                        if let Some(prop) = &picking.$picking_prop {
                            $lhs = prop.clone();
                        }
                    )+
                };
            }
            apply_style!(
                (aspect_ratio, node.aspect_ratio),
                (display, node.display),
                (overflow_x, node.overflow.x),
                (overflow_y, node.overflow.y),
                (position, node.position_type),
                (top, node.top),
                (right, node.right),
                (bottom, node.bottom),
                (left, node.left),
                (flex_basis, node.flex_basis),
                (flex_direction, node.flex_direction),
                (flex_wrap, node.flex_wrap),
                (flex_grow, node.flex_grow),
                (flex_shrink, node.flex_shrink),
                (grid_template_columns, node.grid_template_columns),
                (grid_template_rows, node.grid_template_rows),
                (grid_auto_flow, node.grid_auto_flow),
                (grid_auto_columns, node.grid_auto_columns),
                (grid_auto_rows, node.grid_auto_rows),
                (column_gap, node.column_gap),
                (row_gap, node.row_gap),
                (justify_content, node.justify_content),
                (justify_items, node.justify_items),
                (justify_self, node.justify_self),
                (align_content, node.align_content),
                (align_items, node.align_items),
                (align_self, node.align_self),
                (padding_top, node.padding.top),
                (padding_right, node.padding.right),
                (padding_bottom, node.padding.bottom),
                (padding_left, node.padding.left),
                (margin_top, node.margin.top),
                (margin_right, node.margin.right),
                (margin_bottom, node.margin.bottom),
                (margin_left, node.margin.left),
                (width, node.width),
                (min_width, node.min_width),
                (max_width, node.max_width),
                (height, node.height),
                (min_height, node.min_height),
                (max_height, node.max_height),
                (border_top, node.border.top),
                (border_right, node.border.right),
                (border_bottom, node.border.bottom),
                (border_left, node.border.left),
                (border_radius_tl, node.border_radius.top_left),
                (border_radius_tr, node.border_radius.top_right),
                (border_radius_br, node.border_radius.bottom_right),
                (border_radius_bl, node.border_radius.bottom_left)
            );
        }
    }

    if let Some(background_color) = background_color {
        for picking in &styles {
            if let Some(prop) = &picking.background_color {
                background_color.0 = prop.clone();
            }
        }
    }

    if let Some(z_index) = z_index {
        for picking in &styles {
            if let Some(prop) = &picking.z_index {
                z_index.0 = prop.clone();
            }
        }
    }

    if let Some(border_color) = border_color {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(if let Some(prop) = &picking.$picking_prop { $lhs = prop.clone(); })+
                };
            }
            apply_style!(
                (border_color_top, border_color.top),
                (border_color_right, border_color.right),
                (border_color_bottom, border_color.bottom),
                (border_color_left, border_color.left)
            );
        }
    }

    if let Some(outline) = outline {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(if let Some(prop) = &picking.$picking_prop { $lhs = prop.clone(); })+
                };
            }
            apply_style!(
                (outline_width, outline.width),
                (outline_color, outline.color),
                (outline_offset, outline.offset)
            );
        }
    }

    if let Some(text_font) = text_font {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(if let Some(prop) = &picking.$picking_prop { $lhs = prop.clone(); })+
                };
            }
            apply_style!(
                (font_size, text_font.font_size),
                (font_weight, text_font.weight)
            );
        }
    }

    if let Some(text_layout) = text_layout {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(if let Some(prop) = &picking.$picking_prop { $lhs = prop.clone(); })+
                };
            }
            apply_style!(
                (text_justity, text_layout.justify),
                (text_linebreak, text_layout.linebreak)
            );
        }
    }

    if let Some(text_color) = text_color {
        for picking in &styles {
            if let Some(prop) = &picking.text_color {
                text_color.0 = prop.clone();
            }
        }
    }

    if let Some(ui_transform) = ui_transform {
        for picking in &styles {
            macro_rules! apply_style {
                ($(($picking_prop:ident, $lhs:expr)),+) => {
                    $(if let Some(prop) = &picking.$picking_prop { $lhs = prop.clone(); })+
                };
            }
            apply_style!(
                (translate_x, ui_transform.translation),
                (translate_y, ui_transform.translation),
                (scale_x, ui_transform.scale),
                (scale_y, ui_transform.scale),
                (rotation, ui_transform.rotation)
            );
        }
    }

    if let Some(box_shadow) = box_shadow {
        for picking in &styles {
            if let Some(prop) = &picking.box_shadow {
                box_shadow.0 = prop.clone();
            }
        }
    }

    if let Some(line_height) = line_height {
        for picking in &styles {
            if let Some(prop) = &picking.line_height {
                *line_height = prop.clone();
            }
        }
    }
}

/// System: apply styles on interaction change (hover/press)
fn apply_interaction_style(
    breakpoint: Res<CurrentBreakpoint>,
    mut query: Query<
        (
            &PickingStyles,
            Ref<Interaction>,
            Option<&mut Node>,
            Option<&mut BackgroundColor>,
            Option<&mut ZIndex>,
            Option<&mut BorderColor>,
            Option<&mut Outline>,
            Option<&mut TextFont>,
            Option<&mut TextLayout>,
            Option<&mut TextColor>,
            Option<&mut UiTransform>,
            Option<&mut BoxShadow>,
            Option<&mut LineHeight>,
        ),
        Changed<Interaction>,
    >,
) {
    for (
        picking_styles,
        interaction,
        node,
        background_color,
        z_index,
        border_color,
        outline,
        text_font,
        text_layout,
        text_color,
        ui_transform,
        box_shadow,
        line_height,
    ) in query.iter_mut()
    {
        if interaction.is_added() {
            continue;
        }

        apply_cascade(
            picking_styles,
            &breakpoint,
            interaction.into_inner(),
            node.map(|v| v.into_inner()),
            background_color.map(|v| v.into_inner()),
            z_index.map(|v| v.into_inner()),
            border_color.map(|v| v.into_inner()),
            outline.map(|v| v.into_inner()),
            text_font.map(|v| v.into_inner()),
            text_layout.map(|v| v.into_inner()),
            text_color.map(|v| v.into_inner()),
            ui_transform.map(|v| v.into_inner()),
            box_shadow.map(|v| v.into_inner()),
            line_height.map(|v| v.into_inner()),
        );
    }
}

/// System: re-apply all PickingStyles entities when breakpoint changes
fn apply_responsive_on_resize(
    breakpoint: Res<CurrentBreakpoint>,
    mut query: Query<
        (
            &PickingStyles,
            &Interaction,
            Option<&mut Node>,
            Option<&mut BackgroundColor>,
            Option<&mut ZIndex>,
            Option<&mut BorderColor>,
            Option<&mut Outline>,
            Option<&mut TextFont>,
            Option<&mut TextLayout>,
            Option<&mut TextColor>,
            Option<&mut UiTransform>,
            Option<&mut BoxShadow>,
            Option<&mut LineHeight>,
        ),
    >,
) {
    for (
        picking_styles,
        interaction,
        node,
        background_color,
        z_index,
        border_color,
        outline,
        text_font,
        text_layout,
        text_color,
        ui_transform,
        box_shadow,
        line_height,
    ) in query.iter_mut()
    {
        apply_cascade(
            picking_styles,
            &breakpoint,
            interaction,
            node.map(|v| v.into_inner()),
            background_color.map(|v| v.into_inner()),
            z_index.map(|v| v.into_inner()),
            border_color.map(|v| v.into_inner()),
            outline.map(|v| v.into_inner()),
            text_font.map(|v| v.into_inner()),
            text_layout.map(|v| v.into_inner()),
            text_color.map(|v| v.into_inner()),
            ui_transform.map(|v| v.into_inner()),
            box_shadow.map(|v| v.into_inner()),
            line_height.map(|v| v.into_inner()),
        );
    }
}

#[derive(Default, Component)]
#[require(Interaction)]
pub struct PickingStyles {
    pub base: PickingStyle,
    pub hover: PickingStyle,
    pub focus: PickingStyle,
    pub sm: PickingStyle,
    pub md: PickingStyle,
    pub lg: PickingStyle,
    pub xl: PickingStyle,
}

#[derive(Default)]
pub struct PickingStyle {
    pub aspect_ratio: Option<Option<f32>>,
    // pub box_sizing: Option<BoxSizing>,
    pub display: Option<Display>,
    pub overflow_x: Option<OverflowAxis>,
    pub overflow_y: Option<OverflowAxis>,
    pub position: Option<PositionType>,
    pub top: Option<Val>,
    pub right: Option<Val>,
    pub bottom: Option<Val>,
    pub left: Option<Val>,
    pub z_index: Option<i32>,
    pub flex_basis: Option<Val>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // pub grid_column: Option<GridPlacement>,
    pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // pub grid_row: Option<GridPlacement>,
    pub grid_auto_flow: Option<GridAutoFlow>,
    pub grid_auto_columns: Option<Vec<GridTrack>>,
    pub grid_auto_rows: Option<Vec<GridTrack>>,
    pub column_gap: Option<Val>,
    pub row_gap: Option<Val>,
    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<JustifyItems>,
    pub justify_self: Option<JustifySelf>,
    pub align_content: Option<AlignContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub padding_top: Option<Val>,
    pub padding_right: Option<Val>,
    pub padding_bottom: Option<Val>,
    pub padding_left: Option<Val>,
    pub margin_top: Option<Val>,
    pub margin_right: Option<Val>,
    pub margin_bottom: Option<Val>,
    pub margin_left: Option<Val>,
    pub width: Option<Val>,
    pub min_width: Option<Val>,
    pub max_width: Option<Val>,
    pub height: Option<Val>,
    pub min_height: Option<Val>,
    pub max_height: Option<Val>,
    pub font_size: Option<f32>,
    pub font_weight: Option<bevy::text::FontWeight>,
    pub text_justity: Option<Justify>,
    pub text_color: Option<Color>,
    pub text_linebreak: Option<LineBreak>,
    pub line_height: Option<LineHeight>,
    pub background_color: Option<Color>,
    pub box_shadow: Option<Vec<bevy::ui::ShadowStyle>>,
    pub border_radius_tl: Option<Val>,
    pub border_radius_tr: Option<Val>,
    pub border_radius_br: Option<Val>,
    pub border_radius_bl: Option<Val>,
    pub border_top: Option<Val>,
    pub border_right: Option<Val>,
    pub border_bottom: Option<Val>,
    pub border_left: Option<Val>,
    pub border_color_top: Option<Color>,
    pub border_color_right: Option<Color>,
    pub border_color_bottom: Option<Color>,
    pub border_color_left: Option<Color>,
    pub outline_width: Option<Val>,
    pub outline_color: Option<Color>,
    pub outline_offset: Option<Val>,
    pub translate_x: Option<Val2>,
    pub translate_y: Option<Val2>,
    pub scale_x: Option<Vec2>,
    pub scale_y: Option<Vec2>,
    pub rotation: Option<Rot2>,
}
