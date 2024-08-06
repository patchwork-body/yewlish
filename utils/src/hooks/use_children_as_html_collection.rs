use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlCollection;
use yew::prelude::*;

#[hook]
pub fn use_children_as_html_collection(
    parent_node_ref: NodeRef,
) -> Rc<RefCell<Option<HtmlCollection>>> {
    let children = use_mut_ref(|| None::<HtmlCollection>);

    {
        let parent_node_ref = parent_node_ref.clone();
        let children = children.clone();

        use_effect_with(parent_node_ref, move |parent_node_ref| {
            let parent_node = parent_node_ref.cast::<web_sys::Element>();

            if parent_node.is_none() {
                return;
            }

            let parent_node = parent_node.unwrap();

            if parent_node.child_element_count() == 0 {
                return;
            }

            children.replace(Some(parent_node.children()));
        });
    };

    children
}
