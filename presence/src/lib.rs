use attr_passer::AttrReceiver;
use hooks::use_presence::use_presence;
use yew::prelude::*;

mod hooks;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PresenceRenderAsProps {
    pub presence: bool,
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PresenceProps {
    #[prop_or_default]
    pub name: &'static str,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub present: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub render_as: Option<Callback<PresenceRenderAsProps, Html>>,
}

#[function_component(Presence)]
pub fn presence(props: &PresenceProps) -> Html {
    let node_ref = use_node_ref();
    let presence = use_presence(props.present, node_ref.clone());

    let element = if let Some(render_as) = &props.render_as {
        html! {{
            render_as.emit(PresenceRenderAsProps {
                presence: *presence,
                r#ref: node_ref.clone(),
                children: props.children.clone(),
                class: props.class.clone(),
            })
        }}
    } else {
        if !*presence {
            return html! {};
        }

        html! {
            <div ref={node_ref} class={props.class.clone()}>
                {props.children.clone()}
            </div>
        }
    };

    html! {
        <AttrReceiver name={props.name}>
            {element}
        </AttrReceiver>
    }
}
