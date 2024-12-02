use web_sys::{wasm_bindgen::UnwrapThrowExt, HtmlElement};
use yew::prelude::*;

#[hook]
pub fn use_conditional_attr(
    node_ref: NodeRef,
    attr_name: &'static str,
    attr_value: Option<&'static str>,
    cond: bool,
) {
    use_effect_with((node_ref.clone(), cond), move |(node_ref, cond)| {
        if let Some(node) = node_ref.cast::<HtmlElement>() {
            if *cond {
                node.set_attribute(attr_name, attr_value.unwrap_or(""))
                    .unwrap_throw();
            } else {
                node.remove_attribute(attr_name).unwrap_throw();
            }
        }

        move || {}
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yewlish_testing_tools::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_set_when_true() {
        let t = render!({
            let node_ref = use_node_ref();
            use_conditional_attr(node_ref.clone(), "disabled", None, true);
            use_remember_value(node_ref.clone());

            html! {
                <button ref={node_ref}>{ "TEXT" }</button>
            }
        })
        .await;

        let button = t
            .get_remembered_value::<NodeRef>()
            .cast::<HtmlElement>()
            .unwrap_throw();

        assert_eq!(button.get_attribute("disabled"), Some("".to_string()));
    }

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_unset_when_false() {
        let t = render!({
            let node_ref = use_node_ref();
            use_conditional_attr(node_ref.clone(), "disabled", None, false);
            use_remember_value(node_ref.clone());

            html! {
                <button disabled={true} ref={node_ref}>{ "TEXT" }</button>
            }
        })
        .await;

        let button = t
            .get_remembered_value::<NodeRef>()
            .cast::<HtmlElement>()
            .unwrap_throw();

        assert_eq!(button.get_attribute("disabled"), None);
    }

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_set_with_value_when_true() {
        let t = render!({
            let node_ref = use_node_ref();
            use_conditional_attr(node_ref.clone(), "title", Some("Button"), true);
            use_remember_value(node_ref.clone());

            html! {
                <button ref={node_ref}>{ "TEXT" }</button>
            }
        })
        .await;

        let button = t
            .get_remembered_value::<NodeRef>()
            .cast::<HtmlElement>()
            .unwrap_throw();

        assert_eq!(button.get_attribute("title"), Some("Button".to_string()));
    }

    #[wasm_bindgen_test]
    async fn test_use_conditional_attr_unset_with_value_when_false() {
        let t = render!({
            let node_ref = use_node_ref();
            use_conditional_attr(node_ref.clone(), "title", Some("Button"), false);
            use_remember_value(node_ref.clone());

            html! {
                <button title="Button" ref={node_ref}>{ "TEXT" }</button>
            }
        })
        .await;

        let button = t
            .get_remembered_value::<NodeRef>()
            .cast::<HtmlElement>()
            .unwrap_throw();

        assert_eq!(button.get_attribute("title"), None);
    }
}
