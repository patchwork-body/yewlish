use std::rc::Rc;

use yew::prelude::*;
use yewlish_attr_passer::{attributify, AttrPasser, AttrReceiver};
use yewlish_utils::{
    helpers::combine_handlers::combine_handlers,
    hooks::{use_conditional_attr, use_controllable_state},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchRenderAsProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub children: ChildrenWithProps<SwitchThumb>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub toggle: Callback<()>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
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
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub render_as: Option<Callback<SwitchRenderAsProps, Html>>,
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
        (dispatch.clone(), context_value.clone(), props.readonly),
        move |(), (dispatch, context_value, readonly)| {
            if *readonly {
                return;
            }

            dispatch.emit(Box::new(|prev_state| !prev_state));
            context_value.dispatch(SwitchAction::Toggle);
        },
    );

    let toggle_on_click = use_callback(toggle.clone(), move |_: MouseEvent, toggle| {
        toggle.emit(());
    });

    use_conditional_attr(props.r#ref.clone(), "data-disabled", props.disabled);

    let element = if let Some(render_as) = &props.render_as {
        render_as.emit(SwitchRenderAsProps {
            r#ref: props.r#ref.clone(),
            children: props.children.clone(),
            id: props.id.clone(),
            class: props.class.clone(),
            checked: *checked.borrow(),
            disabled: props.disabled,
            required: props.required,
            name: props.name.clone(),
            value: props.value.clone(),
            readonly: props.readonly,
            toggle: toggle.clone(),
        })
    } else {
        html! {
            <AttrReceiver name="switch">
                <button
                    id={&props.id}
                    class={&props.class}
                    type="button"
                    role="switch"
                    disabled={props.disabled}
                    name={&props.name}
                    value={&props.value}
                    onclick={&combine_handlers(props.onclick.clone(), toggle_on_click.into())}
                >
                    {for props.children.iter()}
                </button>
            </AttrReceiver>
        }
    };

    html! {
        <ContextProvider<ReducibleSwitchContext> context={context_value}>
            <AttrPasser name="switch" ..attributify! {
                "aria-checked" => checked.borrow().to_string(),
                "aria-required" => props.required.then_some("true").unwrap_or_default(),
                "data-state" => if *checked.borrow() { "checked" } else { "unchecked" },
                "data-disabled" => props.disabled.to_string(),
            }>
                {element}
            </AttrPasser>
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
