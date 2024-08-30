use std::ops::Deref;

pub struct HookTester<T> {
    inner: T,
}

impl<T> Deref for HookTester<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> HookTester<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
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
        let h = render_hook!(UseStateHandle<bool>, {
            let a = use_state(|| true);
            a
        })
        .await;

        assert!(**h);
    }

    #[wasm_bindgen_test]
    async fn test_render_hook_with_effect() {
        let h = render_hook!(UseStateHandle<i32>, {
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

        assert_eq!(**h, 100);
    }
}
