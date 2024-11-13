use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Callback<T>>>>,
}

impl<T: 'static + Clone> Signal<T> {
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
