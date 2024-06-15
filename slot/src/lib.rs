use html::{AssertAllProps, ChildrenProps};
use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use yew::html::IntoHtmlResult;
use yew::{
    prelude::*,
    virtual_dom::{Attributes, VNode},
};

#[derive(Clone, Debug, PartialEq)]
pub enum AutoCapitalizeValue {
    Off,
    None,
    On,
    Sentences,
    Words,
    Characters,
}

impl From<AutoCapitalizeValue> for AttrValue {
    fn from(value: AutoCapitalizeValue) -> Self {
        match value {
            AutoCapitalizeValue::Off => "off".into(),
            AutoCapitalizeValue::None => "none".into(),
            AutoCapitalizeValue::On => "on".into(),
            AutoCapitalizeValue::Sentences => "sentences".into(),
            AutoCapitalizeValue::Words => "words".into(),
            AutoCapitalizeValue::Characters => "characters".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct HtmlElementAttributes {
    #[prop_or_default]
    pub accesskey: Option<AttrValue>,
    #[prop_or_default]
    pub autocapitalize: Option<AutoCapitalizeValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub role: Option<AttrValue>,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub aria_hidden: Option<AttrValue>,
    #[prop_or_default]
    pub aria_live: Option<AttrValue>,
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
}

impl Iterator for HtmlElementAttributes {
    type Item = (&'static str, AttrValue);

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = vec![
            ("accesskey", self.accesskey.clone()),
            (
                "autocapitalize",
                self.autocapitalize.clone().map(Into::into),
            ),
            ("id", self.id.clone()),
            ("class", self.class.clone()),
            ("style", self.style.clone()),
            ("title", self.title.clone()),
            ("role", self.role.clone()),
            ("aria-label", self.aria_label.clone()),
            ("aria-hidden", self.aria_hidden.clone()),
            ("aria-live", self.aria_live.clone()),
            ("aria-labelledby", self.aria_labelledby.clone()),
            ("aria-describedby", self.aria_describedby.clone()),
        ]
        .into_iter();

        iter.find(|(_, value)| value.is_some())
            .map(|(key, value)| (key, value.unwrap()))
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SlotProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub arbitrary: Vec<(&'static str, AttrValue)>,
}

#[function_component(Slot)]
pub fn slot(props: &SlotProps) -> Html {
    if props.children.is_empty() {
        return html! {};
    }

    if props.children.len() > 1 {
        log::warn!("Slot component only accepts one child");
        return html! {};
    }

    let element = props.children.iter().next().unwrap();

    if let VNode::VTag(tag) = element.clone() {
        let mut tag = (*tag).clone();

        for (key, value) in props.arbitrary.as_slice() {
            tag.add_attribute(key, value);
        }

        let element = VNode::VTag(Box::new(tag));

        return html! {
            <>{element}</>
        };
    }

    html! {
        <>{element}</>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
