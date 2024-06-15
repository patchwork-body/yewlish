use implicit_clone::unsync::IString;
use yew::html::IntoPropValue;

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

impl IntoPropValue<Option<IString>> for Orientation {
    fn into_prop_value(self) -> Option<IString> {
        match self {
            Self::Horizontal => Some("horizontal".into()),
            Self::Vertical => Some("vertical".into()),
        }
    }
}
