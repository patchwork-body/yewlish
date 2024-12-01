use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{
    js_sys,
    wasm_bindgen::{prelude::Closure, JsCast},
    Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::prelude::*;

type HandlerParams = (
    NodeRef,
    Rc<Option<IntersectionObserverInit>>,
    Callback<IntersectionObserverEntry>,
);

#[hook]
pub fn use_intersection_observer_lazy() -> (
    Rc<RefCell<Option<IntersectionObserver>>>,
    Callback<HandlerParams>,
) {
    let intersection_observer = use_mut_ref(|| None::<IntersectionObserver>);

    let create_intersection_observer = use_callback(
        intersection_observer.clone(),
        |(element_ref, options, handler): HandlerParams, intersection_observer| {
            let callback = Closure::wrap(Box::new(
                move |entries: js_sys::Array, _: IntersectionObserver| {
                    for entry in entries.iter() {
                        if let Some(entry) = entry.dyn_ref::<IntersectionObserverEntry>() {
                            handler.emit(entry.clone());
                        }
                    }
                },
            )
                as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

            let observer = if let Some(opts) = options.as_ref() {
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), opts)
            } else {
                IntersectionObserver::new(callback.as_ref().unchecked_ref())
            };

            match observer {
                Ok(ref observer) => {
                    if let Some(element) = element_ref.cast::<Element>() {
                        observer.observe(&element);
                        intersection_observer.replace(Some(observer.clone()));
                    } else {
                        log::error!("Failed to cast element to Element");
                    }
                }
                Err(err) => {
                    log::error!("Failed to create IntersectionObserver {err:?}");
                }
            }

            callback.forget();
        },
    );

    {
        let intersection_observer = intersection_observer.clone();

        use_effect_with((), move |()| {
            move || {
                if let Some(ref observer) = *intersection_observer.borrow() {
                    observer.disconnect();
                }
            }
        });
    }

    (intersection_observer, create_intersection_observer)
}

#[hook]
pub fn use_intersection_observer(
    element_ref: &NodeRef,
    options: Rc<Option<IntersectionObserverInit>>,
    handler: Callback<IntersectionObserverEntry>,
) -> Rc<RefCell<Option<IntersectionObserver>>> {
    let (intersection_observer, create_intersection_observer) = use_intersection_observer_lazy();

    {
        let element = element_ref.clone();

        use_effect_with((), move |()| {
            create_intersection_observer.emit((element.clone(), options.clone(), handler.clone()));
        });
    }

    intersection_observer
}
