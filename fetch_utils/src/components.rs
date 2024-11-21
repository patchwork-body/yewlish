use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

#[derive(Default, PartialEq, Clone)]
pub struct AccordionContext {
    shown: bool,
}

pub enum AccordionAction {
    Toggle,
}

impl Reducible for AccordionContext {
    type Action = AccordionAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AccordionAction::Toggle => Rc::new(Self { shown: !self.shown }),
        }
    }
}

pub type ReducibleAccordionContext = UseReducerHandle<AccordionContext>;

#[derive(Default, PartialEq, Clone, Properties)]
pub struct AccordionProps {
    pub children: Children,
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let context = use_reducer(AccordionContext::default);

    html! {
        <ContextProvider<ReducibleAccordionContext> context={context}>
            {for props.children.iter()}
        </ContextProvider<ReducibleAccordionContext>>
    }
}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct AccordionTriggerProps {
    pub children: Children,
}

#[function_component(AccordionTrigger)]
pub fn accordion_trigger(props: &AccordionTriggerProps) -> Html {
    let context =
        use_context::<ReducibleAccordionContext>().expect_throw("No AccordionContext found");

    let toggle = use_callback(context.clone(), |event: MouseEvent, context| {
        event.prevent_default();
        context.dispatch(AccordionAction::Toggle);
    });

    html! {
        <div onclick={&toggle}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Default, PartialEq, Clone, Properties)]
pub struct AccordionContentProps {
    pub children: Children,
}

#[function_component(AccordionContent)]
pub fn accordion_content(props: &AccordionContentProps) -> Html {
    let context =
        use_context::<ReducibleAccordionContext>().expect_throw("No AccordionContext found");

    html! {
        <div style={if context.shown { "display: block" } else { "display: none" }}>
            {for props.children.iter()}
        </div>
    }
}
