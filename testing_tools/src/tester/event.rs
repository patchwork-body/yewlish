use std::{future::Future, pin::Pin};

pub trait Event {
    fn click(self) -> Pin<Box<dyn Future<Output = Self>>>;
    fn keydown(self, key: &str) -> Pin<Box<dyn Future<Output = Self>>>;
}
