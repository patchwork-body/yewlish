use yew::prelude::*;

#[hook]
pub fn use_keydown<F>(keys: Vec<String>, callback: F) -> Callback<KeyboardEvent>
where
    F: Fn(KeyboardEvent) + 'static,
{
    Callback::from(move |event: KeyboardEvent| {
        if keys.contains(&event.key()) {
            event.prevent_default();
            event.stop_propagation();
            callback(event);
        }
    })
}
