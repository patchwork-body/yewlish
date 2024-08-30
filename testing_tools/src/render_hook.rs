#[macro_export]
macro_rules! render_hook {
    ($type:ty, $hook:expr, $view:expr) => {{
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::time::Duration;
        use $crate::gloo_utils;
        use $crate::hook_tester::ResultRef;
        use $crate::yew::platform::time::sleep;
        use $crate::yew::props;
        use $crate::*;

        #[derive(Properties, Clone, PartialEq)]
        struct TestRendererProps {
            get_result_ref: Callback<(), ResultRef>,
        }

        #[function_component(TestRenderer)]
        fn test_renderer(props: &TestRendererProps) -> Html {
            let result = $hook;

            use_effect({
                let result = result.clone();
                let get_result_ref = props.get_result_ref.clone();

                move || {
                    *get_result_ref.emit(()).borrow_mut() = Some(Box::new(result) as Box<dyn Any>);
                }
            });

            ($view)(result)
        }

        async fn render_and_parse() -> (HookTester<$type>, Tester) {
            let result_ref: ResultRef = Rc::new(RefCell::new(None));

            {
                let result_ref = result_ref.clone();

                $crate::yew::Renderer::<TestRenderer>::with_root_and_props(
                    gloo_utils::document().get_element_by_id("output").unwrap(),
                    props!(TestRendererProps {
                        get_result_ref: Callback::from(move |_| result_ref.clone()),
                    }),
                )
                .render();
            }

            sleep(Duration::new(0, 0)).await;

            let h = HookTester::new(result_ref);
            let t = Tester::new(gloo_utils::document().get_element_by_id("output").unwrap());

            (h, t)
        }

        render_and_parse()
    }};
    ($type:ty, $hook:expr) => {{
        use $crate::*;

        render_hook!($type, $hook, |_| html! {})
    }};
}
