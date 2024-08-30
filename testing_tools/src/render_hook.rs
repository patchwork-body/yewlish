#[macro_export]
macro_rules! render_hook {
    ($type:ty, $hook:expr, $view:expr) => {{
        use $crate::*;

        type ResultRef = Rc<RefCell<Option<Box<dyn Any>>>>;

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

        async fn render_and_parse() -> HookTester<$type> {
            let result_ref: ResultRef = Rc::new(RefCell::new(None));

            {
                let result_ref = result_ref.clone();

                yew::Renderer::<TestRenderer>::with_root_and_props(
                    gloo_utils::document().get_element_by_id("output").unwrap(),
                    props!(TestRendererProps {
                        get_result_ref: Callback::from(move |_| result_ref.clone()),
                    }),
                )
                .render();
            }

            sleep(Duration::new(0, 0)).await;

            let x = result_ref.borrow_mut().take();

            HookTester::new(x.and_then(|boxed| boxed.downcast::<$type>().ok().map(|boxed| *boxed))
                .expect(r#"Failed to downcast to the expected type. Do you have the correct type in the render_hook! macro, or is the hook returning the wrong type?"#))
        }

        render_and_parse()
    }};
    ($type:ty, $hook:expr) => {{
        use $crate::*;

        render_hook!($type, $hook, |_| html! {})
    }};
}
