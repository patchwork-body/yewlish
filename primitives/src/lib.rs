use html::IntoPropValue;
use implicit_clone::unsync::*;
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

impl IntoPropValue<Option<IString>> for DataState {
    fn into_prop_value(self) -> Option<IString> {
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
    pub r#type: Option<IString>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<IString>,
    #[prop_or_default]
    pub aria_pressed: Option<IString>,
    #[prop_or_default]
    pub aria_label: Option<IString>,
    #[prop_or_default]
    pub data_state: DataState,
    #[prop_or_default]
    pub data_disabled: Option<IString>,
    #[prop_or_default]
    pub aria_disabled: Option<IString>,
    #[prop_or_default]
    pub data_orientation: Option<IString>,
}

#[function_component(PrimitiveButton)]
pub fn primitive_button(props: &PrimitiveButtonProps) -> Html {
    html! {
        <button
            ref={&props.node_ref}
            type={&props.r#type}
            onclick={&props.on_click}
            disabled={props.disabled}
            class={props.class.clone()}
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
