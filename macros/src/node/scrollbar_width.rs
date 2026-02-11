use quote::quote;

use crate::{ParseCtx, ParseResult, utils::val::parse_px};

use super::NodeProp;

pub fn parse_scrollbar_width(ctx: &mut ParseCtx, class: &str) -> ParseResult {
    let val = match class {
        "scrollbar-thin" => quote! { 8.0f32 },
        "scrollbar-none" => quote! { 0.0f32 },
        "scrollbar-auto" => quote! { 16.0f32 },
        class if class.starts_with("scrollbar-[") && class.ends_with("]") => {
            let inner = &class["scrollbar-[".len()..class.len() - 1];
            let Some(px) = parse_px(inner) else {
                return Ok(false);
            };
            quote! { #px }
        }
        _ => {
            return Ok(false);
        }
    };

    ctx.insert_node_prop(NodeProp::ScrollbarWidth, val);
    Ok(true)
}
