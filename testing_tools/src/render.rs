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
/// struct Props {
///     text: &'static str,
/// }
///
/// #[function_component(TestComponent)]
/// fn test_component(props: &Props) -> Html {
///     html! {
///         <div>{props.text}</div>
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
///                 <TestComponent text="Hello, World!" />
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
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::time::Duration;
        use $crate::tester::ResultRef;
        use $crate::yew::platform::time::sleep;
        use $crate::yew::prelude::{function_component, Html};
        use $crate::*;

        #[function_component(TestRenderer)]
        fn test_renderer() -> Html {
            $view
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
