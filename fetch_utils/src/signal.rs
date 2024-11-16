use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Callback<T>>>>,
}

impl<T: 'static + Clone + std::fmt::Debug> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(initial)),
            subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[must_use]
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn set(&self, new_value: T) {
        *self.value.borrow_mut() = new_value.clone();

        web_sys::console::log_1(&format!("Signal set: {new_value:?}").into());
        web_sys::console::log_1(
            &format!("Signal subscribers: {}", self.subscribers.borrow().len()).into(),
        );

        for callback in self.subscribers.borrow().iter() {
            callback.emit(new_value.clone());
        }
    }

    pub fn subscribe(&self, callback: Callback<T>) {
        callback.emit(self.get());
        self.subscribers.borrow_mut().push(callback);
    }

    pub fn subscribe_once(&self, callback: Callback<T>) {
        if !self.subscribers.borrow().contains(&callback) {
            self.subscribe(callback);
        }
    }
}

#[hook]
pub fn use_signal_state<T>(signal: Rc<RefCell<Signal<T>>>) -> UseStateHandle<T>
where
    T: Clone + std::fmt::Debug + 'static,
{
    let state = use_state(|| signal.borrow().get());

    {
        let state = state.clone();

        use_effect_with((), move |()| {
            signal
                .borrow()
                .subscribe_once(Callback::from(move |value: T| {
                    state.set(value);
                }));
        });
    }

    state
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SignalStateProps<T: PartialEq> {
    pub signal: Rc<RefCell<Signal<T>>>,
}

#[function_component(SignalState)]
pub fn signal_state<T>(props: &SignalStateProps<T>) -> Html
where
    T: Clone + std::fmt::Debug + PartialEq + Serialize + 'static,
{
    let state = use_signal_state(props.signal.clone());

    html! {
        <>{
            serde_json::to_string_pretty(&*state).map(|json| {
                html! { <pre>{ json }</pre> }
            }).unwrap_or(html! { "Failed to serialize state" })
        }</>
    }
}
