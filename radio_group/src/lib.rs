use std::rc::Rc;

use primitives::*;
use yew::prelude::*;
use yewlish_roving_focus::*;
use yewlish_utils::enums::dir::Dir;
use yewlish_utils::enums::orientation::Orientation;

#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupContext {
    pub name: Option<AttrValue>,
    pub value: AttrValue,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
}

pub enum RadioGroupAction {
    SetValue(AttrValue),
}

impl Reducible for RadioGroupContext {
    type Action = RadioGroupAction;

    fn reduce(self: Rc<RadioGroupContext>, action: Self::Action) -> Rc<RadioGroupContext> {
        match action {
            RadioGroupAction::SetValue(value) => RadioGroupContext {
                value,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleRadioGroupContext = UseReducerHandle<RadioGroupContext>;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RadioGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub default_value: Option<AttrValue>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub dir: Option<Dir>,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or(true)]
    pub r#loop: bool,
    #[prop_or_default]
    pub on_value_change: Callback<AttrValue>,
}

#[function_component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let context_value = use_reducer(|| RadioGroupContext {
        name: props.name.clone(),
        value: props
            .value
            .clone()
            .unwrap_or(props.default_value.clone().unwrap_or_default()),
        required: props.required,
        disabled: props.disabled,
    });

    use_effect_with(
        (context_value.value.clone(), props.on_value_change.clone()),
        |(value, on_value_change)| {
            on_value_change.emit(value.clone());
        },
    );

    html! {
        <ContextProvider<ReducibleRadioGroupContext> context={context_value}>
            <RovingFocus
                role="radiogroup"
                class={&props.class}
                orientation={props.orientation.clone()}
                dir={props.dir.clone().unwrap_or(Dir::Ltr)}
                r#loop={props.r#loop}
            >
                {for props.children.iter()}
            </RovingFocus>
        </ContextProvider<ReducibleRadioGroupContext>>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadioGroupItemContext {
    pub checked: bool,
}

pub enum RadioGroupItemAction {
    Check,
    Uncheck,
}

impl Reducible for RadioGroupItemContext {
    type Action = RadioGroupItemAction;

    fn reduce(self: Rc<RadioGroupItemContext>, action: Self::Action) -> Rc<RadioGroupItemContext> {
        match action {
            RadioGroupItemAction::Check => RadioGroupItemContext { checked: true }.into(),
            RadioGroupItemAction::Uncheck => RadioGroupItemContext { checked: false }.into(),
        }
    }
}

type ReducibleRadioGroupItemContext = UseReducerHandle<RadioGroupItemContext>;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RadioGroupItemProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<RadioGroupItemIndicator>,
    #[prop_or_default]
    pub checked: Option<bool>,
}

#[function_component(RadioGroupItem)]
pub fn radio_group_item(props: &RadioGroupItemProps) -> Html {
    let group_context = use_context::<ReducibleRadioGroupContext>()
        .expect("RadioGroupItem must be a child of RadioGroup");

    let checked = use_memo(
        (group_context.clone(), props.value.clone(), props.checked),
        |(group_context, props_value, props_checked)| {
            if let Some(props_checked) = props_checked {
                return *props_checked;
            }

            group_context.value == props_value.clone().unwrap_or_default()
        },
    );

    let context_value = use_reducer(|| RadioGroupItemContext { checked: *checked });

    use_effect_with(
        (checked.clone(), context_value.clone()),
        |(checked, context_value)| {
            if **checked {
                context_value.dispatch(RadioGroupItemAction::Check);
            } else {
                context_value.dispatch(RadioGroupItemAction::Uncheck);
            }
        },
    );

    let check = use_callback(
        (group_context.clone(), props.value.clone(), props.checked),
        move |_, (group_context, value, checked)| {
            if checked.unwrap_or_default() {
                return;
            }

            group_context.dispatch(RadioGroupAction::SetValue(
                value.clone().unwrap_or_default(),
            ));
        },
    );

    let check_by_click = use_callback(check.clone(), move |_: MouseEvent, check| {
        check.emit(());
    });

    let check_by_focus = use_callback(check.clone(), move |_: FocusEvent, check| {
        check.emit(());
    });

    let aria_pressed: Option<AttrValue> = if *checked {
        Some("true".into())
    } else {
        Some("false".into())
    };

    let data_state = if *checked {
        DataState::On
    } else {
        DataState::Off
    };

    let disabled: Option<AttrValue> = if props.disabled {
        Some("true".into())
    } else {
        None
    };

    html! {
        <ContextProvider<ReducibleRadioGroupItemContext> context={context_value}>
            <button
                role="radio"
                type="button"
                id={props.id.clone()}
                class={&props.class}
                name={props.name.clone().unwrap_or_else(|| group_context.name.clone().unwrap_or_default())}
                value={props.value.clone()}
                {aria_pressed}
                {data_state}
                data_disabled={&disabled}
                aria_disabled={&disabled}
                onclick={&check_by_click}
                onfocus={&check_by_focus}
            >
                {for props.children.iter()}
            </button>
        </ContextProvider<ReducibleRadioGroupItemContext>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RadioGroupItemIndicatorProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RadioGroupItemIndicator)]
pub fn radio_group_item_indicator(props: &RadioGroupItemIndicatorProps) -> Html {
    let context = use_context::<ReducibleRadioGroupItemContext>()
        .expect("RadioGroupItemIndicator must be a child of RadioGroupItem");

    if !context.checked {
        return html! {};
    }

    html! {
        <span id={&props.id} class={&props.class}>
            {for props.children.iter()}
        </span>
    }
}
