use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};
use web_sys::{Headers, RequestInit};

pub type MiddlewareFuture = Pin<Box<dyn Future<Output = ()>>>;
pub type Middleware =
    Rc<dyn Fn(Rc<RefCell<RequestInit>>, Rc<RefCell<Headers>>) -> MiddlewareFuture>;
