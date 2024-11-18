/// The `render!` macro is used to render Yew components for testing purposes.
/// It renders the component and returns a `Tester` instance that can be used to
/// interact with the rendered components.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
///
/// #[derive(Clone, Properties, PartialEq)]
/// struct TestProps {
///     text: AttrValue,
/// }
///
/// #[function_component(Test)]
/// fn test(props: &TestProps) -> Html {
///     html! {
///         <div>{props.text.clone()}</div>
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use crate::*;
///     use wasm_bindgen_test::*;
///     use yew::prelude::*;
///
///     wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
///
///     #[wasm_bindgen_test]
///     async fn test_render() {
///         let t = render!({
///             html! {
///                 <Test text={"Hello, World!"} />
///             }
///         })
///         .await;
///
///         assert!(t.query_by_text("Hello, World!").exists());
///     }
/// }
/// ```
///
/// # Note
///
/// The `render!` macro leverages `yew::Renderer` to wrap the component under test within a custom component.
/// While not mandatory for testing Yew components, this macro streamlines the rendering process,
/// making it easier to observe rendered results, extract data from it, and simulate user interactions.
#[macro_export]
macro_rules! render {
    ($view:expr) => {{
        use $crate::yew::prelude::*;

        #[derive(Clone, PartialEq, Properties)]
        struct TestWrapperProps {
            children: Children,
        }

        #[function_component(TestWrapper)]
        fn test_wrapper(props: &TestWrapperProps) -> Html {
            html! {
                <>
                    { for props.children.iter() }
                </>
            }
        }

        render!($view, test_wrapper)
    }};
    ($view:expr, $wrapper:expr) => {{
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::time::Duration;
        use web_sys::wasm_bindgen::JsCast;
        use $crate::tester::ResultRef;
        use $crate::yew::platform::time::sleep;
        use $crate::yew::prelude::{function_component, Html};
        use $crate::*;

        #[function_component(TestComponent)]
        fn test_component() -> Html {
            $view
        }

        #[function_component(TestRenderer)]
        fn test_renderer() -> Html {
            html! {
                <$wrapper>
                    <TestComponent />
                </$wrapper>
            }
        }

        thread_local! {
            static RESULT_REF: ResultRef = Rc::new(RefCell::new(None));
        }

        #[allow(dead_code)]
        #[hook]
        pub fn use_remember_value<T>(value: T)
        where
            T: PartialEq + Clone + 'static,
        {
            use_effect_with(value.clone(), |value| {
                RESULT_REF.with(|result_ref| {
                    *result_ref.borrow_mut() = Some(Box::new(value.clone()) as Box<dyn Any>);
                });
            });
        }

        #[allow(dead_code)]
        #[hook]
        pub fn use_mock_fetch(responses: &[(String, serde_json::Value)]) {
            let responses = responses.to_owned();

            use_effect(move || {
                let window = gloo_utils::window();

                let original_fetch = web_sys::js_sys::Reflect::get(
                    &window,
                    &wasm_bindgen::JsValue::from_str("fetch"),
                )
                .unwrap();

                let original_fetch_fn = original_fetch
                    .clone()
                    .unchecked_into::<web_sys::js_sys::Function>();

                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(
                    move |input: wasm_bindgen::JsValue, init: wasm_bindgen::JsValue| {
                        let url = {
                            if let Some(request) = input.dyn_ref::<web_sys::Request>() {
                                request.url()
                            } else {
                                input.as_string().unwrap_or_default()
                            }
                        };

                        if let Some(json_body) = responses
                            .iter()
                            .find(|(pattern, _)| url.contains(pattern))
                            .map(|(_, json_body)| json_body)
                        {
                            let json_body = match serde_json::to_string(json_body) {
                                Ok(json_body) => json_body,
                                Err(err) => {
                                    log::error!("Failed to serialize JSON body: {:?}", err);
                                    return wasm_bindgen::JsValue::NULL;
                                }
                            };

                            let response = web_sys::Response::new_with_opt_str_and_init(
                                Some(&json_body),
                                &web_sys::ResponseInit::new(),
                            )
                            .unwrap();

                            let promise = web_sys::js_sys::Promise::new(&mut |resolve, _| {
                                resolve
                                    .call1(&wasm_bindgen::JsValue::NULL, &response)
                                    .unwrap();
                            });

                            return promise.into();
                        }

                        return original_fetch_fn
                            .call2(&wasm_bindgen::JsValue::NULL, &input, &init)
                            .unwrap();
                    },
                )
                    as Box<
                        dyn FnMut(
                            wasm_bindgen::JsValue,
                            wasm_bindgen::JsValue,
                        ) -> wasm_bindgen::JsValue,
                    >);

                web_sys::js_sys::Reflect::set(
                    &window,
                    &wasm_bindgen::JsValue::from_str("fetch"),
                    closure.as_ref(),
                )
                .unwrap();

                // Keep the closure alive
                closure.forget();

                // Cleanup to restore original fetch
                move || {
                    web_sys::js_sys::Reflect::set(
                        &window,
                        &wasm_bindgen::JsValue::from_str("fetch"),
                        &original_fetch,
                    )
                    .unwrap();
                }
            });
        }

        async fn render_and_parse() -> Tester {
            $crate::yew::Renderer::<TestRenderer>::with_root(
                gloo_utils::document().get_element_by_id("output").unwrap(),
            )
            .render();

            sleep(Duration::ZERO).await;

            Tester::new(
                gloo_utils::document().get_element_by_id("output").unwrap(),
                RESULT_REF.with(|result_ref| result_ref.clone()),
            )
        }

        render_and_parse()
    }};
}
