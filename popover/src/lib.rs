use attr_passer::*;
use presence::*;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};
use utils::hooks::{use_controllable_state, use_interaction_outside};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{wasm_bindgen::prelude::Closure, Element};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PopoverContext {
    pub host: NodeRef,
    pub is_open: bool,
    pub on_toggle: Callback<Event>,
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
        move |_event: Event, dispatch| {
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
    pub toggle: Callback<MouseEvent>,
    pub is_open: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub render_as: Option<Callback<PopoverTriggerRenderAsProps, Html>>,
}

#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &PopoverTriggerProps) -> Html {
    let context = use_context::<ReduciblePopoverContext>()
        .expect("PopoverTrigger must be a child of Popover");

    let toggle = use_callback(context.is_open, {
        let context = context.clone();

        move |event: MouseEvent, _| {
            context.on_toggle.emit(event.into());

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

    let element = if let Some(render_as) = &props.render_as {
        html! {{
            render_as.emit(PopoverTriggerRenderAsProps {
                children: props.children.clone(),
                class: props.class.clone(),
                toggle,
                is_open: context.is_open,
            })
        }}
    } else {
        html! {
            <button class={&props.class} onclick={&toggle}>
                {props.children.clone()}
            </button>
        }
    };

    html! {
        <AttrPasser ..attributify! {
            "data-state" => *data_state,
            "role" => "button",
        }>
            { element }
        </AttrPasser>
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PopoverSide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl Display for PopoverSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PopoverSide::Top => write!(f, "top"),
            PopoverSide::Right => write!(f, "right"),
            PopoverSide::Bottom => write!(f, "bottom"),
            PopoverSide::Left => write!(f, "left"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

impl Display for PopoverAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PopoverAlign::Start => write!(f, "start"),
            PopoverAlign::Center => write!(f, "center"),
            PopoverAlign::End => write!(f, "end"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverContentRenderAsProps {
    pub is_open: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub container: Option<Element>,
    #[prop_or_default]
    pub render_as: Option<Callback<PopoverContentRenderAsProps, Html>>,
    #[prop_or_default]
    pub side: PopoverSide,
    #[prop_or_default]
    pub align: PopoverAlign,
    #[prop_or_default]
    pub on_esc_key_down: Callback<KeyboardEvent>,
    #[prop_or_default]
    pub on_interaction_outside: Callback<Event>,
}

#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let context = use_context::<ReduciblePopoverContext>()
        .expect("PopoverContent must be a child of Popover");

    let host = props.container.clone().unwrap_or_else(|| {
        context
            .host
            .cast::<Element>()
            .expect("PopoverContent must be a child of Popover")
    });

    use_interaction_outside(
        {
            let mut nodes = vec![];
            nodes.push((&host).into());

            if props.container.is_some() {
                nodes.push((&context.host.clone()).into());
            }

            nodes
        },
        {
            let context = context.clone();
            let on_interaction_outside = props.on_interaction_outside.clone();

            move |event: Event| {
                on_interaction_outside.emit(event.clone());

                if event.default_prevented() {
                    return;
                }

                context.on_toggle.emit(event.clone());
                context.dispatch(PopoverAction::Close);
            }
        },
    );

    {
        let context = context.clone();

        use_effect_with(
            (host.clone(), props.on_esc_key_down.clone()),
            |(host, on_esc_key_down)| {
                let on_esc_key_down = on_esc_key_down.clone();

                let listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    if event.key() != "Escape" {
                        return;
                    }

                    on_esc_key_down.emit(event.clone());

                    if event.default_prevented() {
                        return;
                    }

                    context.on_toggle.emit(event.into());
                    context.dispatch(PopoverAction::Close);
                }) as Box<dyn FnMut(_)>);

                let _ = host
                    .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref());

                let host = host.clone();

                move || {
                    let _ = host.remove_event_listener_with_callback(
                        "keydown",
                        listener.as_ref().unchecked_ref(),
                    );
                }
            },
        );
    }

    let dom_rect = host.get_bounding_client_rect();

    let style = stringify!(
        position: fixed;
        top: 0;
        left: 0;
        will-change: transform;
    )
    .to_string();

    let transform = format!(
        "transform: translate({}, {});",
        match props.side {
            PopoverSide::Right => format!("calc({}px + {}px)", dom_rect.x(), dom_rect.width()),
            PopoverSide::Top | PopoverSide::Bottom => match props.align {
                PopoverAlign::Start => format!("calc({}px - 100%)", dom_rect.x()),
                PopoverAlign::Center => format!(
                    "calc({}px - (100% - {}px) / 2)",
                    dom_rect.x(),
                    dom_rect.width(),
                ),
                PopoverAlign::End => format!("calc({}px + {}px)", dom_rect.x(), dom_rect.width()),
            },
            PopoverSide::Left => format!("calc({}px - 100%)", dom_rect.x()),
        },
        match props.side {
            PopoverSide::Top => format!("calc({}px - 100%)", dom_rect.y()),
            PopoverSide::Bottom => format!("calc({}px + {}px)", dom_rect.y(), dom_rect.height()),
            PopoverSide::Right | PopoverSide::Left => match props.align {
                PopoverAlign::Start => format!("calc({}px - 100%)", dom_rect.y()),
                PopoverAlign::Center =>
                    format!("calc({}px - {}px)", dom_rect.y(), dom_rect.height()),
                PopoverAlign::End => format!("calc({}px + {}px)", dom_rect.y(), dom_rect.height()),
            },
        },
    );

    let style = format!("{} {}", style, transform);

    let element = if let Some(render_as) = &props.render_as {
        html! {{
            render_as.emit(PopoverContentRenderAsProps {
                children: props.children.clone(),
                class: props.class.clone(),
                is_open: context.is_open,
            })
        }}
    } else {
        html! {
            <Presence name="popover" present={context.is_open} class={&props.class}>
                {props.children.clone()}
            </Presence>
        }
    };

    create_portal(
        html! {
            <AttrPasser name="popover" ..attributify! {
                "data-state" => if context.is_open { "open" } else { "closed" },
                "data-side" => props.side.to_string(),
                "data-align" => props.align.to_string(),
                "style" => style,
                "role" => "dialog",
            }>
                { element }
            </AttrPasser>
        },
        host,
    )
}
