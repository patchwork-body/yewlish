use attr_passer::*;
use presence::*;
use std::rc::Rc;
use utils::hooks::use_controllable_state::use_controllable_state;
use web_sys::{wasm_bindgen::UnwrapThrowExt, Element};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PopoverContext {
    pub host: NodeRef,
    pub is_open: bool,
    pub on_toggle: Callback<MouseEvent>,
}

pub enum PopoverAction {
    Open,
    Close,
}

impl Reducible for PopoverContext {
    type Action = PopoverAction;

    fn reduce(self: Rc<PopoverContext>, action: Self::Action) -> Rc<PopoverContext> {
        match action {
            PopoverAction::Open => PopoverContext {
                is_open: true,
                ..(*self).clone()
            }
            .into(),
            PopoverAction::Close => PopoverContext {
                is_open: false,
                ..(*self).clone()
            }
            .into(),
        }
    }
}

pub type ReduciblePopoverContext = UseReducerHandle<PopoverContext>;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProps {
    pub children: Children,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub default_open: bool,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let node_ref = use_node_ref();

    let (is_open, dispatch) = use_controllable_state(
        props.default_open.into(),
        props.open.into(),
        props.on_open_change.clone(),
    );

    let on_toggle = use_callback(dispatch.clone(), {
        move |_event: MouseEvent, dispatch| {
            dispatch.emit(Box::new(|prev_state| !prev_state));
        }
    });

    let context_value = use_reducer(|| PopoverContext {
        host: node_ref.clone(),
        is_open: *is_open.borrow(),
        on_toggle,
    });

    html! {
        <ContextProvider<ReduciblePopoverContext> context={context_value}>
            <div ref={node_ref} class={&props.class}>
                {props.children.clone()}
            </div>
        </ContextProvider<ReduciblePopoverContext>>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverTriggerRenderAsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub toggle: Callback<MouseEvent>,
    #[prop_or_default]
    pub data_state: &'static str,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub render_as: Option<Callback<PopoverTriggerRenderAsProps, Html>>,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &PopoverTriggerProps) -> Html {
    let context = use_context::<ReduciblePopoverContext>()
        .expect("PopoverTrigger must be a child of Popover");

    let toggle = use_callback(context.is_open, {
        let context = context.clone();

        move |event: MouseEvent, _| {
            context.on_toggle.emit(event);

            if context.is_open {
                context.dispatch(PopoverAction::Close);
            } else {
                context.dispatch(PopoverAction::Open);
            }
        }
    });

    let data_state = use_memo(
        context.is_open,
        |is_open| {
            if *is_open {
                "open"
            } else {
                "closed"
            }
        },
    );

    if let Some(render_as) = &props.render_as {
        return render_as.emit(PopoverTriggerRenderAsProps {
            children: props.children.clone(),
            class: props.class.clone(),
            toggle,
            data_state: *data_state,
        });
    }

    html! {
        <button data-state={*data_state} class={&props.class} onclick={&toggle}>
            {props.children.clone()}
        </button>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub container: Option<Element>,
}

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let context = use_context::<ReduciblePopoverContext>()
        .expect("PopoverContent must be a child of Popover");

    let host = use_memo(
        (props.container.clone(), context.host.clone()),
        |(container, context_host)| {
            container
                .clone()
                .unwrap_or_else(|| context_host.cast::<Element>().unwrap_throw())
        },
    );

    create_portal(
        html! {
            <AttrPasser ..attributify! {
                "data-state" => if context.is_open { "open" } else { "closed" }
            }>
                <Presence present={context.is_open} class={&props.class}>
                    {props.children.clone()}
                </Presence>
            </AttrPasser>
        },
        (*host).clone(),
    )
}
