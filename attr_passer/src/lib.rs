use yew::{prelude::*, virtual_dom::VNode};

type Attributes = Vec<(&'static str, AttrValue)>;

#[derive(Debug, Clone, PartialEq)]
pub struct AttrPasserContext {
    pub name: AttrValue,
    pub attributes: Attributes,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrPasserProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub attributes: Attributes,
}

#[function_component(AttrPasser)]
pub fn attr_passer(props: &AttrPasserProps) -> Html {
    html! {
        <ContextProvider<AttrPasserContext> context={AttrPasserContext {
            name: props.name.clone(),
            attributes: props.attributes.clone(),
        }}>
            {props.children.clone()}
        </ContextProvider<AttrPasserContext>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrReceiverProps {
    #[prop_or_default]
    pub name: &'static str,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AttrReceiver)]
pub fn attr_receiver(props: &AttrReceiverProps) -> Html {
    let context = use_context::<AttrPasserContext>();

    if context.is_none() {
        return html! {
            <>{props.children.clone()}</>
        };
    }

    let context = context.unwrap();

    if props.name != context.name.as_ref() {
        return html! {
            <>{props.children.clone()}</>
        };
    }

    if props.children.is_empty() {
        return html! {};
    }

    if props.children.len() > 1 {
        log::warn!("AttrReceiver component only accepts one child");
        return html! {};
    }

    let element = props.children.iter().next().unwrap();

    if let VNode::VTag(tag) = element.clone() {
        let mut tag = (*tag).clone();

        for (key, value) in context.attributes.as_slice() {
            tag.add_attribute(key, value);
        }

        let element = VNode::VTag(Box::new(tag));

        return html! {
            <>{element}</>
        };
    } else {
        log::warn!("AttrReceiver component only accepts a tag element");
    }

    html! {}
}

#[macro_export]
macro_rules! attributify {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        use yew::props;

        let mut attributes = vec![];

        $(
            attributes.push(($key, $value.into()));
        )*

        props! {
            AttrPasserProps {
                name: "",
                attributes,
            }
        }
    }};
}
