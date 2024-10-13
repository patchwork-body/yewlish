use std::{borrow::Borrow, rc::Rc};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PresenceState {
    pub current: AttrValue,
}

pub enum PresenceStateAction {
    Mount,
    SuspendUnmount,
    AnimationOut,
    Unmount,
}

impl Reducible for PresenceState {
    type Action = PresenceStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            PresenceStateAction::Mount => {
                let current = self.current.clone();

                if current == "unmounted" || current == "suspended" {
                    PresenceState {
                        current: AttrValue::Static("mounted"),
                    }
                    .into()
                } else {
                    self
                }
            }
            PresenceStateAction::Unmount => {
                let current = self.current.clone();

                if current == "mounted" {
                    PresenceState {
                        current: AttrValue::Static("unmounted"),
                    }
                    .into()
                } else {
                    self
                }
            }
            PresenceStateAction::SuspendUnmount => {
                let current = self.current.clone();

                if current == "mounted" {
                    PresenceState {
                        current: AttrValue::Static("suspended"),
                    }
                    .into()
                } else {
                    self
                }
            }
            PresenceStateAction::AnimationOut => {
                let current = self.current.clone();

                if current == "suspended" {
                    PresenceState {
                        current: AttrValue::Static("unmounted"),
                    }
                    .into()
                } else {
                    self
                }
            }
        }
    }
}

#[hook]
pub fn use_presence(present: bool, node_ref: NodeRef) -> Rc<bool> {
    let state = use_reducer(|| {
        if present {
            PresenceState {
                current: AttrValue::Static("mounted"),
            }
        } else {
            PresenceState {
                current: AttrValue::Static("unmounted"),
            }
        }
    });

    let styles_ref = use_mut_ref(|| None::<web_sys::CssStyleDeclaration>);

    use_effect_with(
        (node_ref.clone(), styles_ref.clone(), state.clone()),
        |(node_ref, styles_ref, _)| {
            if let Some(window) = web_sys::window() {
                if let Some(node) = node_ref.cast::<web_sys::HtmlElement>() {
                    if let Ok(styles) = window.get_computed_style(&node) {
                        styles_ref.replace(styles);
                    }
                }
            }
        },
    );

    let prev_animation_name = use_mut_ref(|| None::<String>);
    let current_animation_name = use_mut_ref(|| None::<String>);

    {
        let state = state.clone();

        use_effect_with(
            (present, current_animation_name.clone()),
            move |(present, current_animation_name)| {
                if *present {
                    state.dispatch(PresenceStateAction::Mount);
                } else if (**current_animation_name).borrow().is_some() {
                    state.dispatch(PresenceStateAction::SuspendUnmount);
                } else {
                    state.dispatch(PresenceStateAction::Unmount);
                }
            },
        );
    }

    {
        let current_animation_name = current_animation_name.clone();
        let prev_animation_name = prev_animation_name.clone();

        use_effect_with((state.clone(), node_ref.clone()), |(state, node_ref)| {
            let state = state.clone();
            let node = node_ref.cast::<web_sys::HtmlElement>();

            let animationstart_handler = Closure::wrap(Box::new(move |event: Event| {
                let Some(window) = web_sys::window() else {
                    return;
                };

                let Some(target) = event.target() else {
                    return;
                };

                let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() else {
                    return;
                };

                let Ok(Some(style)) = window.get_computed_style(element) else {
                    return;
                };

                let Ok(animation_name) = style.get_property_value("animation-name") else {
                    return;
                };

                current_animation_name.replace(animation_name.into());
            }) as Box<dyn FnMut(Event)>);

            let animationend_handler = Closure::wrap(Box::new(move |event: Event| {
                let Some(window) = web_sys::window() else {
                    return;
                };

                let Some(target) = event.target() else {
                    return;
                };

                let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() else {
                    return;
                };

                let Ok(Some(style)) = window.get_computed_style(element) else {
                    return;
                };

                let Ok(animation_name) = style.get_property_value("animation-name") else {
                    return;
                };

                prev_animation_name.replace(animation_name.into());
                state.dispatch(PresenceStateAction::AnimationOut);
            }) as Box<dyn FnMut(Event)>);

            if let Some(node) = node.clone() {
                let _ = node.add_event_listener_with_callback(
                    "animationstart",
                    animationstart_handler.as_ref().unchecked_ref(),
                );

                let _ = node.add_event_listener_with_callback(
                    "animationend",
                    animationend_handler.as_ref().unchecked_ref(),
                );
            }

            move || {
                if let Some(node) = node {
                    let _ = node.remove_event_listener_with_callback(
                        "animationstart",
                        animationstart_handler.as_ref().unchecked_ref(),
                    );

                    let _ = node.remove_event_listener_with_callback(
                        "animationend",
                        animationend_handler.as_ref().unchecked_ref(),
                    );
                }

                animationend_handler.forget();
            }
        });
    }

    let current = use_memo(state.clone(), |state| state.borrow().current != "unmounted");

    current
}
