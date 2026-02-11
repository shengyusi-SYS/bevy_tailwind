use crate::{
    ParseCtx, ParseResult,
    picking::insert_picking_style,
    utils::{deny_computed_style, insert_computed_style},
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
    pub fn parse_shadow(&mut self, class: &str) -> ParseResult {
        parse_class!(parse_box_shadow(self, class));
        Ok(false)
    }
}

fn parse_box_shadow(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if class == "shadow" {
        insert_computed_style!(ctx, box_shadow, BoxShadow, "0", 0);
    }

    let shadow = match class {
        "shadow-sm" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.05),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(1.0),
                    spread_radius: bevy::ui::Val::Px(0.0),
                    blur_radius: bevy::ui::Val::Px(2.0),
                },
            ]
        },
        "shadow" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(1.0),
                    spread_radius: bevy::ui::Val::Px(0.0),
                    blur_radius: bevy::ui::Val::Px(3.0),
                },
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(1.0),
                    spread_radius: bevy::ui::Val::Px(-1.0),
                    blur_radius: bevy::ui::Val::Px(2.0),
                },
            ]
        },
        "shadow-md" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(4.0),
                    spread_radius: bevy::ui::Val::Px(-1.0),
                    blur_radius: bevy::ui::Val::Px(6.0),
                },
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(2.0),
                    spread_radius: bevy::ui::Val::Px(-2.0),
                    blur_radius: bevy::ui::Val::Px(4.0),
                },
            ]
        },
        "shadow-lg" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(10.0),
                    spread_radius: bevy::ui::Val::Px(-3.0),
                    blur_radius: bevy::ui::Val::Px(15.0),
                },
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(4.0),
                    spread_radius: bevy::ui::Val::Px(-4.0),
                    blur_radius: bevy::ui::Val::Px(6.0),
                },
            ]
        },
        "shadow-xl" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(20.0),
                    spread_radius: bevy::ui::Val::Px(-5.0),
                    blur_radius: bevy::ui::Val::Px(25.0),
                },
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.1),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(8.0),
                    spread_radius: bevy::ui::Val::Px(-6.0),
                    blur_radius: bevy::ui::Val::Px(10.0),
                },
            ]
        },
        "shadow-2xl" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.25),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(25.0),
                    spread_radius: bevy::ui::Val::Px(-5.0),
                    blur_radius: bevy::ui::Val::Px(50.0),
                },
            ]
        },
        "shadow-inner" => quote! {
            vec![
                bevy::ui::ShadowStyle {
                    color: bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.05),
                    x_offset: bevy::ui::Val::Px(0.0),
                    y_offset: bevy::ui::Val::Px(2.0),
                    spread_radius: bevy::ui::Val::Px(0.0),
                    blur_radius: bevy::ui::Val::Px(4.0),
                },
            ]
        },
        "shadow-none" => quote! {
            vec![]
        },
        _ => {
            return Ok(false);
        }
    };

    deny_computed_style!(ctx);
    insert_picking_style!(ctx, BoxShadow, shadow);

    ctx.components
        .box_shadow
        .insert("0", shadow, &ctx.class_type, 0);

    Ok(true)
}
