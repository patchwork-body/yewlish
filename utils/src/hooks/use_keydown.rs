use yew::prelude::*;

#[hook]
pub fn use_keydown(
    keys: Vec<String>,
    callback: Callback<KeyboardEvent>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |event: KeyboardEvent| {
        if keys.contains(&event.key()) {
            event.prevent_default();
            event.stop_propagation();
            callback.emit(event);
        }
    })
}
