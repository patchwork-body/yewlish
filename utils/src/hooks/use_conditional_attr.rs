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

#[cfg(test)]
mod tests {
    use super::*;
    use testing_tools::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_set_when_true() {
        let (h, _) = render_hook!(
            NodeRef,
            {
                let node_ref = use_node_ref();
                use_conditional_attr(node_ref.clone(), "disabled", true);
                node_ref
            },
            |node_ref: NodeRef| {
                html! {
                    <button ref={node_ref.clone()}>{ "TEXT" }</button>
                }
            }
        )
        .await;

        let button = h.get().cast::<HtmlElement>().unwrap_throw();
        assert_eq!(button.get_attribute("disabled"), Some("".to_string()));
    }

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_unset_when_false() {
        let (h, _) = render_hook!(
            NodeRef,
            {
                let node_ref = use_node_ref();
                use_conditional_attr(node_ref.clone(), "disabled", false);
                node_ref
            },
            |node_ref: NodeRef| {
                html! {
                    <button disabled={true} ref={node_ref.clone()}>{ "TEXT" }</button>
                }
            }
        )
        .await;

        let button = h.get().cast::<HtmlElement>().unwrap_throw();
        assert_eq!(button.get_attribute("disabled"), None);
    }
}
