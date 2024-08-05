use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl IntoPropValue<Option<AttrValue>> for Orientation {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            Self::Horizontal => Some("horizontal".into()),
            Self::Vertical => Some("vertical".into()),
        }
    }
}
