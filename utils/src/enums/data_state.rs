use yew::{html::IntoPropValue, AttrValue};

#[derive(Clone, Debug, PartialEq)]
pub enum DataState {
    On,
    Off,
}

impl Default for DataState {
    fn default() -> Self {
        Self::Off
    }
}

impl IntoPropValue<Option<AttrValue>> for DataState {
    fn into_prop_value(self) -> Option<AttrValue> {
        match self {
            Self::On => Some("on".into()),
            Self::Off => Some("off".into()),
        }
    }
}
