use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Debug, PartialEq)]
pub enum Dir {
    Ltr,
    Rtl,
}

impl IntoPropValue<Option<AttrValue>> for Dir {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            Dir::Ltr => Some("ltr".into()),
            Dir::Rtl => Some("rtl".into()),
        }
    }
}
