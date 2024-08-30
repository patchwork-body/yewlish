use std::{any::Any, cell::RefCell, rc::Rc};

pub type ResultRef = Rc<RefCell<Option<Box<dyn Any>>>>;

pub struct HookTester<T> {
    inner: ResultRef,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> HookTester<T> {
    pub fn new(inner: ResultRef) -> Self {
        Self {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self) -> T {
        let x = self.inner.borrow_mut().take();

        x.and_then(|boxed| boxed.downcast::<T>().ok().map(|boxed| *boxed))
            .expect(r#"Failed to downcast to the expected type. Do you have the correct type in the render_hook! macro, or is the hook returning the wrong type?"#)
    }
}

#[cfg(test)]
mod test {
    use crate::render_hook;
    use wasm_bindgen_test::*;
    use yew::prelude::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_render_hook() {
        let (h, _) = render_hook!(UseStateHandle<bool>, {
            let a = use_state(|| true);
            a
        })
        .await;

        assert!(*h.get());
    }

    #[wasm_bindgen_test]
    async fn test_render_hook_with_effect() {
        let (h, _) = render_hook!(UseStateHandle<i32>, {
            let a = use_state(|| 0);

            {
                let a = a.clone();

                use_effect_with((), move |_| {
                    a.set(100);
                });
            }

            a
        })
        .await;

        assert_eq!(*h.get(), 100);
    }
}
