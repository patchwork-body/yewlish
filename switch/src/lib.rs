use std::rc::Rc;

use utils::{
    helpers::combine_handlers::combine_handlers,
    hooks::use_controllable_state::use_controllable_state,
};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchProps {
    pub children: ChildrenWithProps<SwitchThumb>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub default_checked: Option<bool>,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_checked_change: Callback<bool>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwitchContext {
    pub(crate) checked: bool,
    pub(crate) disabled: bool,
}

pub enum SwitchAction {
    Toggle,
}

impl Reducible for SwitchContext {
    type Action = SwitchAction;

    fn reduce(self: Rc<SwitchContext>, action: Self::Action) -> Rc<SwitchContext> {
        match action {
            SwitchAction::Toggle => SwitchContext {
                checked: !self.checked,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleSwitchContext = UseReducerHandle<SwitchContext>;

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let (checked, dispatch) = use_controllable_state(
        props.default_checked,
        props.checked,
        props.on_checked_change.clone(),
    );

    let context_value = use_reducer(|| SwitchContext {
        checked: *checked.borrow(),
        disabled: props.disabled,
    });

    use_effect_with(
        (*checked.borrow(), context_value.clone()),
        |(checked, context_value)| {
            if *checked != context_value.checked {
                context_value.dispatch(SwitchAction::Toggle);
            }
        },
    );

    let toggle = use_callback(
        context_value.clone(),
        move |_event: MouseEvent, context_value| {
            dispatch.emit(Box::new(|prev_state| !prev_state));
            context_value.dispatch(SwitchAction::Toggle);
        },
    );

    html! {
        <ContextProvider<ReducibleSwitchContext> context={context_value}>
            <button
                id={&props.id}
                class={&props.class}
                type="button"
                role="switch"
                aria-checked={checked.borrow().to_string()}
                aria-required={props.required.then_some("true")}
                data-state={if *checked.borrow() { "checked" } else { "unchecked" }}
                data-disabled={props.disabled.to_string()}
                disabled={props.disabled}
                name={&props.name}
                value={&props.value}
                onclick={&combine_handlers(props.onclick.clone(), toggle.into())}
            >
                {for props.children.iter()}
            </button>
        </ContextProvider<ReducibleSwitchContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchThumbProps {
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(SwitchThumb)]
pub fn switch_thumb(props: &SwitchThumbProps) -> Html {
    let context =
        use_context::<ReducibleSwitchContext>().expect("SwitchThumb must be a child of Switch");

    let data_state = use_memo(context.checked, |checked| {
        if *checked {
            "checked"
        } else {
            "unchecked"
        }
    });

    html! {
        <div
            class={&props.class}
            data-state={*data_state}
            data-disabled={context.disabled.to_string()}
        ></div>
    }
}
