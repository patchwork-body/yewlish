use std::rc::Rc;

use utils::hooks::use_controllable_state::use_controllable_state;
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
pub enum ListboxSelectedAction {
    Select(AttrValue),
    Deselect(AttrValue),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ListboxContext {
    pub selected: ListboxSelected,
    pub on_selected_change: Callback<ListboxSelectedAction>,
}

pub enum ListboxAction {
    Select(AttrValue),
    Deselect(AttrValue),
    SelectAll,
}

impl Reducible for ListboxContext {
    type Action = ListboxAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ListboxAction::Select(id) => {
                let selected = match self.selected.clone() {
                    ListboxSelected::Single(_) => ListboxSelected::Single(Some(id.clone())),
                    ListboxSelected::Multiple(prev) => {
                        let mut next = prev.clone();
                        next.push(id.clone());
                        ListboxSelected::Multiple(next)
                    }
                };

                self.on_selected_change
                    .emit(ListboxSelectedAction::Select(id));

                ListboxContext {
                    selected,
                    ..(*self).clone()
                }
                .into()
            }
            ListboxAction::Deselect(id) => {
                let selected = match self.selected.clone() {
                    ListboxSelected::Single(_) => ListboxSelected::Single(None),
                    ListboxSelected::Multiple(prev) => {
                        let mut next = prev.clone();
                        next.retain(|v| v != &id);
                        ListboxSelected::Multiple(next)
                    }
                };

                self.on_selected_change
                    .emit(ListboxSelectedAction::Deselect(id));

                ListboxContext {
                    selected,
                    ..(*self).clone()
                }
                .into()
            }
            ListboxAction::SelectAll => self,
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
}

#[function_component(Listbox)]
pub fn listbox(props: &ListboxProps) -> Html {
    let (selected, dispatch) = use_controllable_state(
        props.default_selected.clone(),
        props.selected.clone(),
        props.on_selected_change.clone(),
    );

    let on_selected_change = use_callback(
        dispatch.clone(),
        |selected: ListboxSelectedAction, dispatch| {
            dispatch.emit(Box::new(move |state| match state {
                ListboxSelected::Single(_) => match selected.clone() {
                    ListboxSelectedAction::Select(value) => ListboxSelected::Single(Some(value)),
                    ListboxSelectedAction::Deselect(_) => ListboxSelected::Single(None),
                },
                ListboxSelected::Multiple(prev) => match selected.clone() {
                    ListboxSelectedAction::Select(id) => {
                        let mut next = prev.clone();
                        next.push(id);
                        ListboxSelected::Multiple(next)
                    }
                    ListboxSelectedAction::Deselect(id) => {
                        let mut next = prev.clone();
                        next.retain(|v| v != &id);
                        ListboxSelected::Multiple(next)
                    }
                },
            }));
        },
    );

    let context_value = use_reducer(|| ListboxContext {
        selected: (*selected).clone().borrow().clone(),
        on_selected_change,
    });

    html! {
        <ContextProvider<MutableListboxContext> context={context_value}>
            <ul role="listbox" aria-multiselectable={if props.multiple { "true" } else { "false" }}>
                {for props.children.iter()}
            </ul>
        </ContextProvider<MutableListboxContext>>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListboxOptionProps {
    pub id: &'static str,
    pub children: Children,
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
        (props.id, is_selected.clone(), context.clone()),
        |_event: MouseEvent, (id, is_selected, context)| {
            if **is_selected {
                context.dispatch(ListboxAction::Deselect(AttrValue::from(*id)));
            } else {
                context.dispatch(ListboxAction::Select(AttrValue::from(*id)));
            }
        },
    );

    let select_on_focus = use_callback(
        (props.id, context.clone()),
        |_event: FocusEvent, (id, context)| {
            context.dispatch(ListboxAction::Select(AttrValue::from(*id)));
        },
    );

    if *is_multiple {
        html! {
            <li id={props.id} role="option" aria-checked={*aria_selected} onclick={select_on_click} onfocus={select_on_focus}>
                {for props.children.iter()}
            </li>
        }
    } else {
        html! {
            <li id={props.id} role="option" aria-selected={*aria_selected} onclick={select_on_click}>
                {for props.children.iter()}
            </li>
        }
    }
}
