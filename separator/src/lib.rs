use utils::enums::orientation::Orientation;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SeparatorProps {
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or_default]
    pub decorative: bool,
}

#[function_component(Separator)]
pub fn separator(props: &SeparatorProps) -> Html {
    let role = (!props.decorative).then_some("separator");
    let aria_orientation: Option<&str> = (!props.decorative).then_some(match props.orientation {
        Orientation::Horizontal => "vertical",
        Orientation::Vertical => "horizontal",
    });

    html! {
        <div
            role={role}
            class={&props.class}
            aria-orientation={aria_orientation}
            data-orientation={props.orientation.clone()}
        />
    }
}
