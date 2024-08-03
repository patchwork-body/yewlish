use yew::{prelude::*, virtual_dom::VNode};

type Attributes = Vec<(&'static str, AttrValue)>;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrPasserProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub attributes: Attributes,
}

#[function_component(AttrPasser)]
pub fn attr_passer(props: &AttrPasserProps) -> Html {
    html! {
        <ContextProvider<Attributes> context={props.attributes.clone()}>
            {props.children.clone()}
        </ContextProvider<Attributes>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AttrReceiverProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AttrReceiver)]
pub fn attr_receiver(props: &AttrReceiverProps) -> Html {
    let attributes = use_context::<Attributes>().unwrap_or_default();

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

        for (key, value) in attributes.as_slice() {
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
                attributes,
            }
        }
    }};
}
