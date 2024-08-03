use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or_default]
    pub color: Option<AttrValue>,
    #[prop_or_default]
    pub width: Option<AttrValue>,
    #[prop_or_default]
    pub height: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
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
            class={&props.class}
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

#[function_component(CheckIcon)]
pub fn check_icon(props: &IconProps) -> Html {
    html! {
        <svg
            width={props.width.clone().unwrap_or("15".into())}
            height={props.height.clone().unwrap_or("15".into())}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class={&props.class}
        >
            <path
                d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
                fill={props.color.clone().unwrap_or("currentColor".into())}
                fill-rule="evenodd"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[function_component(CircleIcon)]
pub fn circle_icon(props: &IconProps) -> Html {
    html! {
        <svg
            width={props.width.clone().unwrap_or("15".into())}
            height={props.height.clone().unwrap_or("15".into())}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class={&props.class}
        >
            <path
                d="M0.877075 7.49991C0.877075 3.84222 3.84222 0.877075 7.49991 0.877075C11.1576 0.877075 14.1227 3.84222 14.1227 7.49991C14.1227 11.1576 11.1576 14.1227 7.49991 14.1227C3.84222 14.1227 0.877075 11.1576 0.877075 7.49991ZM7.49991 1.82708C4.36689 1.82708 1.82708 4.36689 1.82708 7.49991C1.82708 10.6329 4.36689 13.1727 7.49991 13.1727C10.6329 13.1727 13.1727 10.6329 13.1727 7.49991C13.1727 4.36689 10.6329 1.82708 7.49991 1.82708Z"
                fill={props.color.clone().unwrap_or("currentColor".into())}
                fill-rule="evenodd"
                clip-rule="evenodd"
            />
        </svg>
    }
}
