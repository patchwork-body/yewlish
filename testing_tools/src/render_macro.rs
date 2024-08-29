#[macro_export]
macro_rules! render {
    ($($html:tt)*) => {{
        use yew::prelude::*;
        use std::time::Duration;
        use yew::platform::time::sleep;

        #[cfg(not(feature = "internal"))]
        use testing_tools::Tester;

        #[cfg(feature = "internal")]
        use $crate::Tester;

        #[function_component(TestRenderer)]
        fn test_renderer() -> Html {
            html! {
                $($html)*
            }
        }

        async fn render_and_parse() -> Tester {
            yew::Renderer::<TestRenderer>::with_root(
                gloo_utils::document()
                    .get_element_by_id("output")
                    .unwrap(),
            )
            .render();

            sleep(Duration::new(0, 0)).await;

            Tester::new(
                gloo_utils::document()
                    .get_element_by_id("output")
                    .unwrap()
            )
        }

        render_and_parse()
    }};
}
