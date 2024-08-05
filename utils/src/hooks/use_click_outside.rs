use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

pub enum Node {
    Element(web_sys::Element),
    Ref(NodeRef),
}

impl From<&web_sys::Element> for Node {
    fn from(element: &web_sys::Element) -> Self {
        Node::Element(element.clone())
    }
}

impl From<&NodeRef> for Node {
    fn from(node_ref: &NodeRef) -> Self {
        Node::Ref(node_ref.clone())
    }
}

#[hook]
pub fn use_click_outside<T>(nodes: Vec<Node>, callback: T)
where
    T: Fn(Event) + 'static,
{
    use std::rc::Rc;

    use web_sys::wasm_bindgen::prelude::Closure;

    let callback = Callback::from(callback);
    let callback_ref = Rc::new(callback);

    use_effect_with(callback_ref, move |callback_ref| {
        let callback = callback_ref.clone();
        let callback_ref = Rc::downgrade(callback_ref);

        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let target = event.target_unchecked_into::<web_sys::Element>();

            if !nodes.iter().any(|node| match &node {
                Node::Element(element) => element.contains(Some(&target)),
                Node::Ref(node_ref) => {
                    if let Some(node) = node_ref.cast::<web_sys::Element>() {
                        node.contains(Some(&target))
                    } else {
                        false
                    }
                }
            }) {
                callback.emit(event);
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = web_sys::window() {
            let _ = window
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref());

            let _ = window
                .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref());
        }

        move || {
            if callback_ref.upgrade().is_some() {
                if let Some(window) = web_sys::window() {
                    let _ = window.remove_event_listener_with_callback(
                        "mousedown",
                        closure.as_ref().unchecked_ref(),
                    );

                    let _ = window.remove_event_listener_with_callback(
                        "touchstart",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
        }
    });
}
