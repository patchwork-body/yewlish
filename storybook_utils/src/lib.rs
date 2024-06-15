#[macro_export]
macro_rules! story {
    ($component:ty, { title: $title:expr }) => {
        use yew::prelude::*;

        #[function_component(Story)]
        pub fn story() -> Html {
            html! {
                <div>
                    {$title}
                    <$component />
                </div>
            }
        }

        Story
    };
}
