use html::IntoPropValue;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum DataState {
    On,
    Off,
}

impl Default for DataState {
    fn default() -> Self {
        Self::Off
    }
}

impl IntoPropValue<Option<AttrValue>> for DataState {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            Self::On => Some("on".into()),
            Self::Off => Some("off".into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct PrimitiveButtonProps {
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub r#type: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub aria_pressed: Option<AttrValue>,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub data_state: DataState,
    #[prop_or_default]
    pub data_disabled: Option<AttrValue>,
    #[prop_or_default]
    pub aria_disabled: Option<AttrValue>,
    #[prop_or_default]
    pub data_orientation: Option<AttrValue>,
}

#[function_component(PrimitiveButton)]
pub fn primitive_button(props: &PrimitiveButtonProps) -> Html {
    html! {
        <button
            ref={&props.node_ref}
            type={&props.r#type}
            onclick={&props.on_click}
            disabled={props.disabled}
            class={&props.class}
            aria-pressed={&props.aria_pressed}
            aria-label={&props.aria_label}
            data-state={props.data_state.clone()}
            data-orientation={&props.data_orientation}
            data-disabled={&props.data_disabled}
            aria-disabled={&props.aria_disabled}
        >
            {props.children.clone()}
        </button>
    }
}
