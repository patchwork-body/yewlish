use attr_passer::AttrReceiver;
use hooks::use_presence::use_presence;
use yew::prelude::*;

mod hooks;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PresenceProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub present: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(Presence)]
pub fn presence(props: &PresenceProps) -> Html {
    let (presence, node_ref) = use_presence(props.present);

    html! {
        <AttrReceiver>
            <div hidden={!*presence} ref={node_ref} class={&props.class}>
                {props.children.clone()}
            </div>
        </AttrReceiver>
    }
}
