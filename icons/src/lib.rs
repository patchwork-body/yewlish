use implicit_clone::unsync::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or_default]
    pub color: Option<IString>,
    #[prop_or_default]
    pub width: Option<IString>,
    #[prop_or_default]
    pub height: Option<IString>,
}

#[function_component(FontItalicIcon)]
pub fn font_italic_icon(props: &IconProps) -> Html {
    html! {
        <svg
            width={props.width.clone().unwrap_or("15".into())}
            height={props.height.clone().unwrap_or("15".into())}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                d="M5.67494 3.50017C5.67494 3.25164 5.87641 3.05017 6.12494 3.05017H10.6249C10.8735 3.05017 11.0749 3.25164 11.0749 3.50017C11.0749 3.7487 10.8735 3.95017 10.6249 3.95017H9.00587L7.2309 11.05H8.87493C9.12345 11.05 9.32493 11.2515 9.32493 11.5C9.32493 11.7486 9.12345 11.95 8.87493 11.95H4.37493C4.1264 11.95 3.92493 11.7486 3.92493 11.5C3.92493 11.2515 4.1264 11.05 4.37493 11.05H5.99397L7.76894 3.95017H6.12494C5.87641 3.95017 5.67494 3.7487 5.67494 3.50017Z"
                fill={props.color.clone().unwrap_or("currentColor".into())}
                fillRule="evenodd"
                clipRule="evenodd"
            />
        </svg>
    }
}
