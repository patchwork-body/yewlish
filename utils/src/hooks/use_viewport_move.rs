use web_sys::wasm_bindgen::JsCast;
use web_sys::{wasm_bindgen::prelude::Closure, Element, ResizeObserver};
use yew::prelude::*;

use crate::hooks::use_observe_move;

#[hook]
pub fn use_viewport_move(element_ref: &NodeRef, on_move: Callback<()>) {
    let (refresh, cleanup) = use_observe_move(element_ref, on_move);

    use_effect_with(
        (refresh.clone(), cleanup.clone(), element_ref.clone()),
        |(refresh, cleanup, element_ref)| {
            refresh.emit(true);

            let refresh = refresh.clone();

            let callback = Closure::wrap(Box::new(move || {
                refresh.emit(true);
            }) as Box<dyn FnMut()>);

            let el_resize_observer = ResizeObserver::new(callback.as_ref().unchecked_ref());

            if let Ok(ref resize_observer) = el_resize_observer {
                if let Some(element) = element_ref.cast::<Element>() {
                    resize_observer.observe(&element);
                }
            }

            let root_resize_observer = ResizeObserver::new(callback.as_ref().unchecked_ref());

            if let Ok(ref resize_observer) = root_resize_observer {
                if let Some(window) = web_sys::window() {
                    if let Some(root) = window.document().and_then(|doc| doc.document_element()) {
                        resize_observer.observe(&root);
                    }
                }
            }

            callback.forget();

            let cleanup = cleanup.clone();

            move || {
                cleanup.emit(());

                if let Ok(ref resize_observer) = el_resize_observer {
                    resize_observer.disconnect();
                }
            }
        },
    );
}
