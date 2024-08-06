use yew::prelude::*;

#[hook]
pub fn use_keydown<F>(keys: Vec<String>, callback: F) -> Callback<KeyboardEvent>
where
    F: Fn(KeyboardEvent) + 'static,
{
    use std::rc::Rc;

    let callback = Callback::from(callback);
    let callback_ref = Rc::new(callback);

    use_callback(
        (keys.clone(), callback_ref.clone()),
        |event: KeyboardEvent, (keys, callback_ref)| {
            if keys.contains(&event.key()) {
                event.prevent_default();
                event.stop_propagation();
                callback_ref.emit(event);
            }
        },
    )
}
