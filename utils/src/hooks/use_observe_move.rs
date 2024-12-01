use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{wasm_bindgen::prelude::Closure, IntersectionObserverEntry};
use web_sys::{Element, IntersectionObserverInit};
use yew::prelude::*;

use crate::hooks::use_intersection_observer::use_intersection_observer_lazy;

type RefreshCallback = Callback<bool>;

#[hook]
pub fn use_observe_move(
    element_ref: &NodeRef,
    on_move: Callback<()>,
) -> (RefreshCallback, Callback<()>) {
    let (intersection_observer, create_intersection_observer) = use_intersection_observer_lazy();
    let timeout_id_ref = use_mut_ref(|| None::<i32>);

    let cleanup = use_callback(
        (intersection_observer.clone(), timeout_id_ref.clone()),
        |(), (intersection_observer, timeout_id_ref)| {
            if let Some(ref observer) = *intersection_observer.borrow() {
                observer.disconnect();
            }

            if let Some(ref timer_id) = *timeout_id_ref.borrow() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(*timer_id);
                }
            }
        },
    );

    let root = use_memo(element_ref.clone(), |element_ref| {
        element_ref.cast::<Element>().and_then(|element| {
            element
                .owner_document()
                .and_then(|doc| doc.document_element())
        })
    });

    let refresh_ref = use_mut_ref(|| None::<RefreshCallback>);
    let is_first_refresh = use_mut_ref(|| true);
    let current_threshold = use_mut_ref(|| None::<f64>);

    let handler = use_callback(
        (
            refresh_ref.clone(),
            timeout_id_ref.clone(),
            is_first_refresh.clone(),
            current_threshold.clone(),
        ),
        |entry: IntersectionObserverEntry,
         (refresh_ref, timeout_id_ref, is_first_refresh, current_threshold)| {
            let ratio = entry.intersection_ratio();

            if !*is_first_refresh.borrow() {
                if let Some(refresh) = refresh_ref.borrow().as_ref() {
                    refresh.emit(false);
                    return;
                }
            }

            let threshold = current_threshold.borrow().unwrap_or(1.0);

            if (ratio - threshold).abs() > f64::EPSILON {
                if !*is_first_refresh.borrow() {
                    if let Some(refresh) = refresh_ref.borrow().as_ref() {
                        refresh.emit(false);
                        return;
                    }
                }

                if ratio == 0.0 {
                    if let Some(window) = web_sys::window() {
                        let refresh_ref = refresh_ref.clone();
                        let current_threshold = current_threshold.clone();

                        let callback = Closure::wrap(Box::new(move || {
                            if let Some(refresh) = refresh_ref.borrow().as_ref() {
                                current_threshold.replace(1e-7.into());
                                refresh.emit(false);
                            }
                        })
                            as Box<dyn FnMut()>);

                        match window.set_timeout_with_callback(callback.as_ref().unchecked_ref()) {
                            Ok(timeout_id) => {
                                *timeout_id_ref.borrow_mut() = Some(timeout_id);
                            }
                            Err(err) => {
                                log::error!("Failed to set timeout {err:?}");
                            }
                        };

                        callback.forget();
                    }
                } else if let Some(refresh) = refresh_ref.borrow().as_ref() {
                    current_threshold.replace(ratio.into());
                    refresh.emit(false);
                }
            }

            is_first_refresh.replace(false);
        },
    );

    let refresh = {
        use_callback(
            (
                on_move.clone(),
                create_intersection_observer.clone(),
                cleanup.clone(),
                element_ref.clone(),
                root.clone(),
                is_first_refresh.clone(),
                handler.clone(),
                current_threshold.clone(),
            ),
            |skip: bool,
             (
                on_move,
                create_intersection_observer,
                cleanup,
                element_ref,
                root,
                is_first_refresh,
                handler,
                current_threshold,
            )| {
                cleanup.emit(());

                if !skip {
                    on_move.emit(());
                }

                if let Some(ref element) = element_ref.cast::<Element>() {
                    if let Some(root) = root.as_ref() {
                        let el_rect = element.get_bounding_client_rect();

                        let options = IntersectionObserverInit::new();

                        options.set_root_margin(
                            format!(
                                "-{}px -{}px -{}px -{}px",
                                el_rect.top().floor().abs(),
                                (root.client_width() as f64
                                    - (el_rect.left() + el_rect.width()).floor())
                                .abs(),
                                (root.client_height() as f64
                                    - (el_rect.top() + el_rect.height()).floor())
                                .abs(),
                                el_rect.left().floor().abs(),
                            )
                            .as_str(),
                        );

                        let threshold = current_threshold
                            .borrow()
                            .map(|t| t.clamp(0.0, 1.0))
                            .unwrap_or(1.0);

                        options.set_threshold(&JsValue::from(threshold));

                        is_first_refresh.replace(true);

                        create_intersection_observer.emit((
                            element_ref.clone(),
                            Some(options).into(),
                            handler.clone(),
                        ));
                    }
                }
            },
        )
    };

    *refresh_ref.borrow_mut() = Some(refresh.clone());

    (refresh, cleanup)
}
