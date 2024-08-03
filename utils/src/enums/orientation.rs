use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl IntoPropValue<Option<AttrValue>> for Orientation {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            Self::Horizontal => Some("horizontal".into()),
            Self::Vertical => Some("vertical".into()),
        }
    }
}
