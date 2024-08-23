use web_sys::{wasm_bindgen::UnwrapThrowExt, HtmlElement};
use yew::prelude::*;

#[hook]
pub fn use_conditional_attr(node_ref: NodeRef, attr_name: &'static str, cond: bool) {
    use_effect_with((node_ref.clone(), cond), |(node_ref, cond)| {
        if let Some(node) = node_ref.cast::<HtmlElement>() {
            if *cond {
                node.set_attribute(attr_name, "").unwrap_throw();
            } else {
                node.remove_attribute(attr_name).unwrap_throw();
            }
        } else {
            log::warn!("use_conditional_attr received node_ref: {:?}", node_ref);
        }

        move || {}
    });
}
