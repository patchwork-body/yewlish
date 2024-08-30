#[macro_export]
macro_rules! render {
    ($($html:tt)*) => {{
        use $crate::*;

        #[function_component(TestRenderer)]
        fn test_renderer() -> Html {
            html! {
                $($html)*
            }
        }

        async fn render_and_parse() -> Tester {
            Renderer::<TestRenderer>::with_root(
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
