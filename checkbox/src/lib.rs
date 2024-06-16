use std::default::Default;
use std::rc::Rc;

use html::IntoPropValue;
use implicit_clone::unsync::*;
use utils::hooks::use_controllable_state::use_controllable_state;
use yew::prelude::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum CheckedState {
    Checked,
    #[default]
    Unchecked,
}

impl IntoPropValue<Option<IString>> for CheckedState {
    fn into_prop_value(self) -> Option<IString> {
        match self {
            CheckedState::Checked => Some("checked".into()),
            CheckedState::Unchecked => Some("unchecked".into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxContext {
    pub(crate) checked: CheckedState,
    pub(crate) disabled: bool,
}

pub enum CheckboxAction {
    Toggle,
}

impl Reducible for CheckboxContext {
    type Action = CheckboxAction;

    fn reduce(self: Rc<CheckboxContext>, action: Self::Action) -> Rc<CheckboxContext> {
        match action {
            CheckboxAction::Toggle => CheckboxContext {
                checked: match self.checked {
                    CheckedState::Checked => CheckedState::Unchecked,
                    CheckedState::Unchecked => CheckedState::Checked,
                },
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleCheckboxContext = UseReducerHandle<CheckboxContext>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxProps {
    pub children: ChildrenWithProps<CheckboxIndicator>,
    #[prop_or_default]
    pub id: Option<IString>,
    #[prop_or_default]
    pub class: Option<IString>,
    #[prop_or_default]
    pub default_checked: Option<CheckedState>,
    #[prop_or_default]
    pub checked: Option<CheckedState>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<CheckedState>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<IString>,
    #[prop_or_default]
    pub value: Option<IString>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let (checked, dispatch) = use_controllable_state(
        props.default_checked.clone(),
        props.checked.clone(),
        props.on_checked_change.clone(),
    );

    let context_value = use_reducer(|| CheckboxContext {
        checked: checked.borrow().clone(),
        disabled: props.disabled,
    });

    let toggle = use_callback(
        (dispatch.clone(), context_value.clone()),
        move |_: MouseEvent, (dispatch, context_value)| {
            dispatch.emit(Box::new(|prev_state| match prev_state {
                CheckedState::Checked => CheckedState::Unchecked,
                CheckedState::Unchecked => CheckedState::Checked,
            }));

            context_value.dispatch(CheckboxAction::Toggle);
        },
    );

    let prevent_checked_by_enter = use_callback((), |event: KeyboardEvent, _| {
        if event.key() == "Enter" {
            event.prevent_default();
        }
    });

    html! {
        <ContextProvider<ReducibleCheckboxContext> context={context_value}>
            <button
                id={props.id.clone()}
                class={props.class.clone()}
                type="button"
                role="checkbox"
                aria-checked={if *checked.borrow() == CheckedState::Checked { "true" } else { "false" }}
                aria-required={if props.required { "true" } else { "false" }}
                data-state={checked.borrow().clone()}
                data-disabled={if props.disabled { Some(String::new()) } else { None::<String> }}
                disabled={props.disabled}
                name={props.name.clone()}
                value={props.value.clone()}
                onkeydown={prevent_checked_by_enter}
                onclick={toggle}
            >
                {for props.children.iter()}
            </button>
        </ContextProvider<ReducibleCheckboxContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckboxIndicatorProps {
    #[prop_or_default]
    pub class: Option<IString>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub show_when: CheckedState,
}

#[function_component(CheckboxIndicator)]
pub fn checkbox_indicator(props: &CheckboxIndicatorProps) -> Html {
    let context = use_context::<ReducibleCheckboxContext>()
        .expect("CheckboxIndicator must be a child of Checkbox");

    if context.checked != props.show_when {
        return html! {};
    }

    html! {
        <span
            class={props.class.clone()}
            data-state={if context.checked == CheckedState::Checked { "checked" } else { "unchecked" }}
            data-disabled={if context.disabled { Some(String::new()) } else { None::<String> }}
        >
            {for props.children.iter()}
        </span>
    }
}
