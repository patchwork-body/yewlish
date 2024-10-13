use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use yew::prelude::*;

type Dispatch<T> = Callback<Box<dyn Fn(T) -> T>>;

#[hook]
pub fn use_controllable_state<T>(
    initial: Option<T>,
    controlled: Option<T>,
    on_change: Callback<T>,
) -> (Rc<RefCell<T>>, Dispatch<T>)
where
    T: Debug + Default + PartialEq + Clone + 'static,
{
    let value: Rc<RefCell<T>> = use_mut_ref(|| {
        controlled
            .clone()
            .unwrap_or_else(|| initial.unwrap_or_default())
    });

    let trigger = use_force_update();

    let dispatch = use_callback((value.clone(), controlled.is_some(), on_change), {
        let trigger = trigger.clone();

        move |new_state: Box<dyn Fn(T) -> T>, (value, is_controlled, on_change)| {
            let new_state = new_state(value.borrow().clone());

            value.replace(new_state.clone());
            on_change.emit(new_state);

            if !is_controlled {
                trigger.force_update();
            }
        }
    });

    // Controlled value has been changed without dispatching
    use_effect_with(
        (controlled.clone(), value.clone(), dispatch.clone()),
        move |(controlled, value, dispatch)| {
            if let Some(controlled) = controlled.as_ref() {
                if *controlled != *value.borrow() {
                    let controlled = controlled.clone();
                    dispatch.emit(Box::new(move |_| controlled.clone()));
                    trigger.force_update();
                }
            }
        },
    );

    (value, dispatch)
}
