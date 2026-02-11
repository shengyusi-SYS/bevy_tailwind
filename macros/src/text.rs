use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{color::Color, deny_computed_style, insert_computed_style, val::parse_px},
};
use quote::quote;

macro_rules! parse_class {
    ($($expr:expr),*) => {
        $(
            match $expr {
                Ok(true) => {
                    return Ok(true)
                }
                Err(e) => {
                  return Err(e);
                }
                _ => {}
            }
        )*
    };
}

impl ParseCtx {
    pub fn parse_text(&mut self, class: &str) -> ParseResult {
        parse_class!(
            parse_font_size(self, class),
            parse_font_weight(self, class),
            parse_font_smoothing(self, class),
            parse_text_align(self, class),
            parse_line_break(self, class),
            parse_line_height(self, class),
            parse_text_decoration(self, class),
            parse_text_color(self, class)
        );
        Ok(false)
    }
}

fn parse_font_size(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "text" {
        insert_computed_style!(ctx, text_font, FontSize, "font_size", 0);
    }

    if !class.starts_with("text-") {
        return Ok(false);
    }

    let class = &class["text-".len()..];

    let font_size: f32 = match class {
        "xs" => 12.,
        "sm" => 14.,
        "base" => 16.,
        "lg" => 18.,
        "xl" => 20.,
        "2xl" => 24.,
        "3xl" => 30.,
        "4xl" => 36.,
        "5xl" => 48.,
        "6xl" => 64.,
        "7xl" => 80.,
        "8xl" => 96.,
        "9xl" => 128.,
        class if class.starts_with("[") && class.ends_with("]") => {
            let class = &class[1..class.len() - 1];
            let Some(px) = parse_px(class) else {
                return Ok(false);
            };
            px
        }
        _ => {
            return Ok(false);
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, FontSize, quote! { #font_size });

    ctx.components
        .text_font
        .insert("font_size", quote! {#font_size}, &ctx.class_type, 0);

    Ok(true)
}

fn parse_font_smoothing(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class != "antialiased" {
        return Ok(false);
    }

    deny_computed_style!(ctx);
    ctx.components.text_font.insert(
        "font_smoothing",
        quote! {
            bevy::text::FontSmoothing::AntiAliased
        },
        &ctx.class_type,
        0,
    );

    Ok(true)
}

fn parse_text_align(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "text-align" {
        insert_computed_style!(ctx, text_layout, TextJustify, "justify", 0);
    }

    if !class.starts_with("text-") {
        return Ok(false);
    }

    let class = &class["text-".len()..];

    let justify = match class {
        "left" => quote! {bevy::text::Justify::Left},
        "center" => quote! {bevy::text::Justify::Center},
        "right" => quote! {bevy::text::Justify::Right},
        "justify" => quote! {bevy::text::Justify::Justified},
        _ => {
            return Ok(false);
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, TextJustify, justify);

    ctx.components
        .text_layout
        .insert("justify", justify, &ctx.class_type, 0);

    Ok(true)
}

fn parse_line_break(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "break" {
        insert_computed_style!(ctx, text_layout, TextLinebreak, "linebreak", 0);
    }

    let line_break = match class {
        "break-words" => quote! {bevy::text::LineBreak::WordBoundary},
        "break-all" => quote! {bevy::text::LineBreak::AnyCharacter},
        _ => {
            return Ok(false);
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, TextLinebreak, line_break);

    ctx.components
        .text_layout
        .insert("linebreak", line_break, &ctx.class_type, 0);

    Ok(true)
}

fn parse_text_color(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "text-color" {
        insert_computed_style!(ctx, text_color, TextColor, "0", 0);
    }

    if !class.starts_with("text-") {
        return Ok(false);
    }

    let Some(color) = Color::parse(&class["text-".len()..]) else {
        return Ok(false);
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, TextColor, color);

    ctx.components
        .text_color
        .insert("0", color, &ctx.class_type, 0);

    Ok(true)
}

fn parse_font_weight(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let weight = match class {
        "font-thin" => quote! { bevy::text::FontWeight::THIN },
        "font-extralight" => quote! { bevy::text::FontWeight::EXTRA_LIGHT },
        "font-light" => quote! { bevy::text::FontWeight::LIGHT },
        "font-normal" => quote! { bevy::text::FontWeight::NORMAL },
        "font-medium" => quote! { bevy::text::FontWeight::MEDIUM },
        "font-semibold" => quote! { bevy::text::FontWeight::SEMIBOLD },
        "font-bold" => quote! { bevy::text::FontWeight::BOLD },
        "font-extrabold" => quote! { bevy::text::FontWeight::EXTRA_BOLD },
        "font-black" => quote! { bevy::text::FontWeight::BLACK },
        _ => {
            return Ok(false);
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, FontWeight, weight);

    ctx.components
        .text_font
        .insert("weight", weight, &ctx.class_type, 0);

    Ok(true)
}

fn parse_line_height(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("leading-") {
        return Ok(false);
    }

    let class = &class["leading-".len()..];

    let val = match class {
        "none" => quote! { bevy::text::LineHeight::RelativeToFont(1.0) },
        "tight" => quote! { bevy::text::LineHeight::RelativeToFont(1.25) },
        "snug" => quote! { bevy::text::LineHeight::RelativeToFont(1.375) },
        "normal" => quote! { bevy::text::LineHeight::RelativeToFont(1.5) },
        "relaxed" => quote! { bevy::text::LineHeight::RelativeToFont(1.625) },
        "loose" => quote! { bevy::text::LineHeight::RelativeToFont(2.0) },
        class if class.starts_with("[") && class.ends_with("]") => {
            let inner = &class[1..class.len() - 1];
            if let Some(px) = parse_px(inner) {
                quote! { bevy::text::LineHeight::Px(#px) }
            } else if let Ok(val) = inner.parse::<f32>() {
                quote! { bevy::text::LineHeight::RelativeToFont(#val) }
            } else {
                return Ok(false);
            }
        }
        _ => {
            if let Ok(n) = class.parse::<u32>() {
                let px = n as f32 * 4.0;
                quote! { bevy::text::LineHeight::Px(#px) }
            } else if let Ok(n) = class.parse::<f32>() {
                let px = n * 4.0;
                quote! { bevy::text::LineHeight::Px(#px) }
            } else {
                return Ok(false);
            }
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, LineHeight, val);

    ctx.components.line_height = Some(val);

    Ok(true)
}

fn parse_text_decoration(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    match class {
        "underline" => {
            deny_computed_style!(ctx);
            ctx.components.underline = true;
            Ok(true)
        }
        "line-through" => {
            deny_computed_style!(ctx);
            ctx.components.strikethrough = true;
            Ok(true)
        }
        "no-underline" => {
            deny_computed_style!(ctx);
            ctx.components.underline = false;
            ctx.components.strikethrough = false;
            Ok(true)
        }
        _ => Ok(false),
    }
}
