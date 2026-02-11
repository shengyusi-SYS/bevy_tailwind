use quote::quote;

use crate::{ParseCtx, ParseResult, utils::val::parse_px};

use super::NodeProp;

pub fn parse_overflow_clip_margin(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    if !class.starts_with("overflow-clip-margin-") {
        return Ok(false);
    }

    let class = &class["overflow-clip-margin-".len()..];

    let val = match class {
        "content" => quote! {
            bevy::ui::OverflowClipMargin::content_box()
        },
        "padding" => quote! {
            bevy::ui::OverflowClipMargin::padding_box()
        },
        "border" => quote! {
            bevy::ui::OverflowClipMargin::border_box()
        },
        class if class.starts_with("[") && class.ends_with("]") => {
            let inner = &class[1..class.len() - 1];
            let Some(px) = parse_px(inner) else {
                return Ok(false);
            };
            quote! {
                bevy::ui::OverflowClipMargin::default().with_margin(#px)
            }
        }
        _ => {
            if let Ok(n) = class.parse::<u32>() {
                let px = n as f32 * 4.0;
                quote! {
                    bevy::ui::OverflowClipMargin::default().with_margin(#px)
                }
            } else {
                return Ok(false);
            }
        }
    };

    ctx.insert_node_prop(NodeProp::OverflowClipMargin, val);
    Ok(true)
}
