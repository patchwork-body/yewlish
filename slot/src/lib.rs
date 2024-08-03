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

// #[derive(Clone, Debug, PartialEq)]
// pub enum AutoCapitalizeValue {
//     Off,
//     None,
//     On,
//     Sentences,
//     Words,
//     Characters,
// }

// impl From<AutoCapitalizeValue> for AttrValue {
//     fn from(value: AutoCapitalizeValue) -> Self {
//         match value {
//             AutoCapitalizeValue::Off => "off".into(),
//             AutoCapitalizeValue::None => "none".into(),
//             AutoCapitalizeValue::On => "on".into(),
//             AutoCapitalizeValue::Sentences => "sentences".into(),
//             AutoCapitalizeValue::Words => "words".into(),
//             AutoCapitalizeValue::Characters => "characters".into(),
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq, Default, Properties)]
// pub struct HtmlElementAttributes {
//     #[prop_or_default]
//     pub accesskey: Option<AttrValue>,
//     #[prop_or_default]
//     pub autocapitalize: Option<AutoCapitalizeValue>,
//     #[prop_or_default]
//     pub id: Option<AttrValue>,
//     #[prop_or_default]
//     pub class: Option<AttrValue>,
//     #[prop_or_default]
//     pub style: Option<AttrValue>,
//     #[prop_or_default]
//     pub title: Option<AttrValue>,
//     #[prop_or_default]
//     pub role: Option<AttrValue>,
//     #[prop_or_default]
//     pub aria_label: Option<AttrValue>,
//     #[prop_or_default]
//     pub aria_hidden: Option<AttrValue>,
//     #[prop_or_default]
//     pub aria_live: Option<AttrValue>,
//     #[prop_or_default]
//     pub aria_labelledby: Option<AttrValue>,
//     #[prop_or_default]
//     pub aria_describedby: Option<AttrValue>,
// }

// impl Iterator for HtmlElementAttributes {
//     type Item = (&'static str, AttrValue);

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut iter = vec![
//             ("accesskey", self.accesskey.clone()),
//             (
//                 "autocapitalize",
//                 self.autocapitalize.clone().map(Into::into),
//             ),
//             ("id", self.id.clone()),
//             ("class", self.class.clone()),
//             ("style", self.style.clone()),
//             ("title", self.title.clone()),
//             ("role", self.role.clone()),
//             ("aria-label", self.aria_label.clone()),
//             ("aria-hidden", self.aria_hidden.clone()),
//             ("aria-live", self.aria_live.clone()),
//             ("aria-labelledby", self.aria_labelledby.clone()),
//             ("aria-describedby", self.aria_describedby.clone()),
//         ]
//         .into_iter();

//         iter.find(|(_, value)| value.is_some())
//             .map(|(key, value)| (key, value.unwrap()))
//     }
// }

pub type Arbitrary = Vec<(&'static str, AttrValue)>;

#[macro_export]
macro_rules! arbitrify {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        use yew::props;

        let mut arbitrary = vec![];

        $(
            arbitrary.push(($key, $value.into()));
        )*

        props! {
            SlotProps {
                arbitrary,
            }
        }
    }};
}
