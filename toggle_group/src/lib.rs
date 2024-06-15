use std::{cell::RefCell, rc::Rc};

use implicit_clone::unsync::*;
use roving_focus::{utils::*, RovingFocus};
use toggle::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ToggleGroupType {
    Radio,
    Checkbox,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToggleGroupContext {
    pub(crate) r#type: ToggleGroupType,
    pub(crate) value: Rc<RefCell<Vec<IString>>>,
    pub(crate) disabled: bool,
    pub(crate) orientation: Orientation,
}

pub enum ToggleGroupAction {
    Activate(IString),
    Deactivate(IString),
    Replace(IString),
}

impl Reducible for ToggleGroupContext {
    type Action = ToggleGroupAction;

    fn reduce(self: Rc<ToggleGroupContext>, action: Self::Action) -> Rc<ToggleGroupContext> {
        match action {
            ToggleGroupAction::Activate(value) => ToggleGroupContext {
                value: Rc::new(RefCell::new(
                    self.value
                        .borrow()
                        .iter()
                        .cloned()
                        .chain(std::iter::once(value))
                        .collect(),
                )),
                ..(*self).clone()
            }
            .into(),

            ToggleGroupAction::Deactivate(value) => ToggleGroupContext {
                value: Rc::new(RefCell::new(
                    self.value
                        .borrow()
                        .iter()
                        .filter(|v| *v != &value)
                        .cloned()
                        .collect(),
                )),
                ..(*self).clone()
            }
            .into(),

            ToggleGroupAction::Replace(value) => ToggleGroupContext {
                value: Rc::new(RefCell::new(vec![value])),
                ..(*self).clone()
            }
            .into(),
        }
    }
}

type ReducibleToggleGroupContext = UseReducerHandle<ToggleGroupContext>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ToggleGroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<ToggleGroupItem>,
    #[prop_or(ToggleGroupType::Radio)]
    pub r#type: ToggleGroupType,
    #[prop_or_default]
    pub default_value: Option<Vec<IString>>,
    #[prop_or_default]
    pub value: Option<Vec<IString>>,
    #[prop_or_default]
    pub on_value_change: Callback<Vec<IString>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<IString>,
    #[prop_or(true)]
    pub roving_focus: bool,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or_default]
    pub dir: Option<Dir>,
    #[prop_or(true)]
    pub r#loop: bool,
}

#[function_component(ToggleGroup)]
pub fn toggle_group(props: &ToggleGroupProps) -> Html {
    let context_value = use_reducer(|| ToggleGroupContext {
        r#type: props.r#type.clone(),
        value: Rc::new(RefCell::new(props.value.clone().unwrap_or(
            props.default_value.clone().unwrap_or_else(|| {
                if props.r#type == ToggleGroupType::Radio {
                    log::warn!("ToggleGroup must have a default value for radio type");
                }

                vec![]
            }),
        ))),
        disabled: props.disabled,
        orientation: props.orientation.clone(),
    });

    if props.roving_focus {
        return html! {
            <ContextProvider<ReducibleToggleGroupContext> context={context_value}>
                <RovingFocus
                    class={&props.class}
                    orientation={props.orientation.clone()}
                    dir={props.dir.clone().unwrap_or(Dir::Ltr)}
                    r#loop={props.r#loop}
                >
                    {for props.children.iter()}
                </RovingFocus>
            </ContextProvider<ReducibleToggleGroupContext>>
        };
    }

    html! {
        <ContextProvider<ReducibleToggleGroupContext> context={context_value}>
            <div class={&props.class} data-orientation={props.orientation.clone()}>
                {for props.children.iter()}
            </div>
        </ContextProvider<ReducibleToggleGroupContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ToggleGroupItemProps {
    pub value: IString,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<IString>,
}

#[function_component(ToggleGroupItem)]
pub fn toggle_group_item(props: &ToggleGroupItemProps) -> Html {
    let context = use_context::<ReducibleToggleGroupContext>()
        .expect("ToggleGroupItem must be a child of ToggleGroup");

    let pressed = use_memo(
        (context.value.clone(), props.value.clone()),
        |(context_value, props_value)| context_value.borrow().contains(props_value),
    );

    let on_pressed_change = use_callback(
        (context.clone(), props.value.clone()),
        move |next_state: bool, (context, value)| {
            if context.r#type == ToggleGroupType::Radio && next_state {
                context.dispatch(ToggleGroupAction::Replace(value.clone()));
                return;
            }

            if next_state {
                context.dispatch(ToggleGroupAction::Activate(value.clone()));
            } else {
                context.dispatch(ToggleGroupAction::Deactivate(value.clone()));
            }
        },
    );

    html! {
        <Toggle
            class={&props.class}
            pressed={*pressed}
            on_pressed_change={&on_pressed_change}
            orientation={context.orientation.clone()}
            disabled={props.disabled || context.disabled}
        >
            {props.children.clone()}
        </Toggle>
    }
}
