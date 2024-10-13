use yew::Callback;

#[must_use]
pub fn combine_handlers<T: Clone + 'static>(
    a: Option<Callback<T>>,
    b: Option<Callback<T>>,
) -> Callback<T> {
    Callback::from(move |event: T| {
        if let Some(a) = a.clone() {
            a.emit(event.clone());
        }

        if let Some(b) = b.clone() {
            b.emit(event);
        }
    })
}
