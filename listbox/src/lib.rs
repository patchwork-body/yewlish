use std::rc::Rc;

use presence::*;
use utils::hooks::{use_children_as_html_collection, use_controllable_state, use_keydown};
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ListboxSelected {
    Single(Option<AttrValue>),
    Multiple(Vec<AttrValue>),
}

impl Default for ListboxSelected {
    fn default() -> Self {
        ListboxSelected::Single(None)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ListboxContext {
    pub active: AttrValue,
    pub selected: ListboxSelected,
    pub on_selected_change: Callback<ListboxSelected>,
}

pub enum ListboxAction {
    ToggleSelection(AttrValue),
    SetActive(AttrValue),
}

impl Reducible for ListboxContext {
    type Action = ListboxAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ListboxAction::ToggleSelection(id) => {
                let selected = match self.selected.clone() {
                    ListboxSelected::Single(prev) => match prev {
                        Some(prev) => {
                            if prev == id {
                                ListboxSelected::Single(None)
                            } else {
                                ListboxSelected::Single(Some(id.clone()))
                            }
                        }
                        None => ListboxSelected::Single(Some(id.clone())),
                    },
                    ListboxSelected::Multiple(prev) => {
                        if prev.contains(&id) {
                            let mut next = prev.clone();
                            next.retain(|v| v != &id);
                            ListboxSelected::Multiple(next)
                        } else {
                            let mut next = prev.clone();
                            next.push(id.clone());
                            ListboxSelected::Multiple(next)
                        }
                    }
                };

                self.on_selected_change.emit(selected.clone());

                ListboxContext {
                    selected,
                    active: id,
                    ..(*self).clone()
                }
                .into()
            }
            ListboxAction::SetActive(id) => ListboxContext {
                active: id,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

pub type MutableListboxContext = UseReducerHandle<ListboxContext>;

#[derive(Clone, PartialEq, Properties)]
pub struct ListboxProps {
    pub children: ChildrenWithProps<ListboxOption>,
    #[prop_or_default]
    pub selected: Option<ListboxSelected>,
    #[prop_or_default]
    pub default_selected: Option<ListboxSelected>,
    #[prop_or_default]
    pub on_selected_change: Callback<ListboxSelected>,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(Listbox)]
pub fn listbox(props: &ListboxProps) -> Html {
    let default_selected = use_memo(
        (props.default_selected.clone(), props.multiple),
        |(default_selected, multiple)| {
            if default_selected.is_some() {
                default_selected.clone()
            } else if *multiple {
                Some(ListboxSelected::Multiple(Vec::new()))
            } else {
                Some(ListboxSelected::Single(None))
            }
        },
    );

    let selected = use_memo(
        (props.selected.clone(), default_selected.clone()),
        |(selected, default_selected)| {
            if selected.is_some() {
                selected.clone()
            } else {
                (**default_selected).clone()
            }
        },
    );

    let (selected, dispatch) = use_controllable_state(
        (*default_selected).clone(),
        (*selected).clone(),
        props.on_selected_change.clone(),
    );

    let on_selected_change =
        use_callback(dispatch.clone(), |selected: ListboxSelected, dispatch| {
            dispatch.emit(Box::new(move |_| selected.clone()));
        });

    let context_value = use_reducer(|| ListboxContext {
        active: AttrValue::from(""),
        selected: (*selected).clone().borrow().clone(),
        on_selected_change,
    });

    let node_ref = use_node_ref();
    let options = use_children_as_html_collection(node_ref.clone());

    let navigation_handler = {
        let options = options.clone();
        let context_value = context_value.clone();

        move |event: KeyboardEvent| {
            let options = options.borrow();
            let options = options.as_ref();
            let active = context_value.active.clone();

            if event.key() == " " {
                context_value.dispatch(ListboxAction::ToggleSelection(AttrValue::from(active)));
                return;
            }

            if options.is_none() {
                return;
            }

            let options = options.unwrap();

            let current_option_index = if active.is_empty() {
                None
            } else {
                let mut index = None;

                for i in 0..options.length() {
                    if let Some(option) = options.item(i) {
                        if option.id() == active {
                            index = Some(i);
                            break;
                        }
                    }
                }

                index
            };

            let next_index = {
                if let Some(index) = current_option_index {
                    match event.key().as_str() {
                        "ArrowDown" => {
                            if index + 1 >= options.length() {
                                Some(0)
                            } else {
                                Some(index + 1)
                            }
                        }
                        "ArrowUp" => {
                            if index == 0 {
                                Some(options.length() - 1)
                            } else {
                                Some(index - 1)
                            }
                        }
                        _ => None,
                    }
                } else {
                    match event.key().as_str() {
                        "ArrowDown" => Some(0),
                        "ArrowUp" => Some(options.length() - 1),
                        _ => None,
                    }
                }
            };

            if let Some(index) = next_index {
                if let Some(option) = options.item(index) {
                    context_value.dispatch(ListboxAction::SetActive(AttrValue::from(option.id())));
                }
            }
        }
    };

    let navigate_through_options = use_keydown(
        vec!["ArrowDown".into(), "ArrowUp".into(), " ".into()],
        navigation_handler,
    );

    html! {
        <ContextProvider<MutableListboxContext> context={context_value.clone()}>
            <ul ref={node_ref} role="listbox" tabindex="0" class={props.class.clone()} aria-multiselectable={if props.multiple { "true" } else { "false" }} aria-activedescendant={context_value.active.clone()} onkeydown={navigate_through_options}>
                {for props.children.iter()}
            </ul>
        </ContextProvider<MutableListboxContext>>
    }
}

#[derive(Clone, PartialEq)]
pub struct ListboxOptionContext {
    pub is_selected: Rc<bool>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListboxOptionProps {
    pub id: &'static str,
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(ListboxOption)]
pub fn listbox_option(props: &ListboxOptionProps) -> Html {
    let context =
        use_context::<MutableListboxContext>().expect("ListboxOption must be a child of Listbox");

    let is_selected = use_memo(
        (props.id, context.selected.clone()),
        |(id, selected)| match selected.clone() {
            ListboxSelected::Single(selected) => selected == AttrValue::from(*id).into(),
            ListboxSelected::Multiple(selected) => selected.contains(&AttrValue::from(*id)),
        },
    );

    let context_value = ListboxOptionContext {
        is_selected: is_selected.clone(),
    };

    let aria_selected = use_memo(is_selected.clone(), |is_selected| {
        if **is_selected {
            "true"
        } else {
            "false"
        }
    });

    let is_multiple = use_memo(context.selected.clone(), |selected| {
        matches!(selected, ListboxSelected::Multiple(_))
    });

    let select_on_click = use_callback(
        (props.id, context.clone()),
        |_event: MouseEvent, (id, context)| {
            context.dispatch(ListboxAction::ToggleSelection(AttrValue::from(*id)));
        },
    );

    use_effect_with(
        (context.clone(), is_selected.clone(), props.id),
        |(context, is_selected, id)| {
            if let ListboxSelected::Single(selected) = &context.selected {
                match selected {
                    Some(selected) => {
                        if selected == id && !**is_selected {
                            context.dispatch(ListboxAction::ToggleSelection(AttrValue::from(*id)));
                        }
                    }
                    None => {
                        if !**is_selected {
                            context.dispatch(ListboxAction::ToggleSelection(AttrValue::from(*id)));
                        }
                    }
                }
            }
        },
    );

    let data_active = use_memo(context.active.clone(), |active| {
        if *active == props.id {
            "true"
        } else {
            "false"
        }
    });

    let element = if *is_multiple {
        html! {
            <li id={props.id} role="option" class={props.class.clone()} aria-checked={*aria_selected} data-active={*data_active} onclick={select_on_click}>
                {for props.children.iter()}
            </li>
        }
    } else {
        html! {
            <li id={props.id} role="option" class={props.class.clone()} aria-selected={*aria_selected} data-active={*data_active} onclick={select_on_click}>
                {for props.children.iter()}
            </li>
        }
    };

    html! {
        <ContextProvider<ListboxOptionContext> context={context_value}>
            {element}
        </ContextProvider<ListboxOptionContext>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListboxOptionIndicatorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(ListboxOptionIndicator)]
pub fn listbox_option_indicator(props: &ListboxOptionIndicatorProps) -> Html {
    let context = use_context::<ListboxOptionContext>()
        .expect("ListboxOptionIndicator must be a child of ListboxOption");

    html! {
        <Presence name="listbox-option-indicator" present={*context.is_selected} class={props.class.clone()} render_as={
            Callback::from(|props: PresenceRenderAsProps| {
                if !props.presence {
                    return html! {
                        <span ref={props.r#ref} class={&props.class} aria-hidden="true"/>
                    };
                }

                html! {
                    <span ref={props.r#ref} class={&props.class} aria-hidden="true">
                        {props.children.clone()}
                    </span>
                }
            })
        }>
            {props.children.clone()}
        </Presence>
    }
}
