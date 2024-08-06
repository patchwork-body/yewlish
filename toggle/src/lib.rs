use primitives::*;
use utils::{enums::orientation::Orientation, hooks::*};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ToggleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_pressed: Option<bool>,
    #[prop_or_default]
    pub pressed: Option<bool>,
    #[prop_or_default]
    pub on_pressed_change: Callback<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub r#type: Option<AttrValue>,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub orientation: Orientation,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    let (value, dispatch) = use_controllable_state(
        props.default_pressed,
        props.pressed,
        props.on_pressed_change.clone(),
    );

    let toggle = use_callback((), move |_event: MouseEvent, _| {
        dispatch.emit(Box::new(move |prev_state| !prev_state));
    });

    let aria_pressed: Option<AttrValue> = if *value.borrow() {
        Some("true".into())
    } else {
        Some("false".into())
    };

    let data_state = if *value.borrow() {
        DataState::On
    } else {
        DataState::Off
    };

    let disabled: Option<AttrValue> = if props.disabled {
        Some("true".into())
    } else {
        None
    };

    // TODO: combine on_click handlers

    let rest_props = PrimitiveButtonProps {
        node_ref: props.node_ref.clone(),
        r#type: props.r#type.clone(),
        class: props.class.clone(),
        disabled: props.disabled,
        on_click: props.on_click.clone(),
        ..Default::default()
    };

    html! {
        <PrimitiveButton
            r#type="button"
            {aria_pressed}
            {data_state}
            data_disabled={&disabled}
            aria_disabled={&disabled}
            data_orientation={props.orientation.clone()}
            on_click={toggle}
            ..rest_props
        >
            {props.children.clone()}
        </PrimitiveButton>
    }
}
