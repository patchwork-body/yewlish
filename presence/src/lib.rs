use hooks::use_presence::use_presence;
use yew::prelude::*;
use yewlish_attr_passer::AttrReceiver;

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
    pub r#ref: NodeRef,
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
    #[prop_or_default]
    pub on_present: Option<Callback<()>>,
}

#[function_component(Presence)]
pub fn presence(props: &PresenceProps) -> Html {
    let presence = use_presence(props.present, props.r#ref.clone());

    let element = if let Some(render_as) = &props.render_as {
        html! {{
            render_as.emit(PresenceRenderAsProps {
                presence: *presence,
                r#ref: props.r#ref.clone(),
                children: props.children.clone(),
                class: props.class.clone(),
            })
        }}
    } else {
        if !*presence {
            return html! {};
        }

        if let Some(on_present) = &props.on_present {
            on_present.emit(());
        }

        html! {
            <div ref={props.r#ref.clone()} class={props.class.clone()}>
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
