use std::rc::Rc;

use attr_passer::*;
use presence::*;
use web_sys::Element;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PopoverContext {
    pub host: NodeRef,
    pub is_open: bool,
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
}

#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    let node_ref = use_node_ref();

    let context_value = use_reducer(|| PopoverContext {
        host: node_ref.clone(),
        is_open: false,
    });

    html! {
        <ContextProvider<ReduciblePopoverContext> context={context_value}>
            <div ref={node_ref}>
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

        move |_event: MouseEvent, _| {
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
}

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let context = use_context::<ReduciblePopoverContext>()
        .expect("PopoverContent must be a child of Popover");

    let host = context.host.cast::<Element>();

    if host.is_none() {
        log::error!("PopoverContent must be a child of Popover");
        return html! {};
    }

    let host = host.unwrap();

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
        host,
    )
}
