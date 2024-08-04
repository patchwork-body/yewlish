use checkbox::CheckboxPage;
use popover::PopoverPage;
use radio_group::RadioGroupPage;
use switch::SwitchPage;
use toggle::TogglePage;
use toggle_group::ToggleGroupPage;
use yew::prelude::*;

use crate::Router;

mod checkbox;
mod common;
mod popover;
mod radio_group;
mod switch;
mod toggle;
mod toggle_group;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct StorybookProps {}

#[function_component(StorybookPage)]
pub fn storybook_page(_props: &StorybookProps) -> Html {
    let location = use_context::<Router>().expect("Router Context not found!");

    match location.path.as_str() {
        "toggle" | "/toggle" => html! { <TogglePage /> },
        "checkbox" | "/checkbox" => html! { <CheckboxPage /> },
        "switch" | "/switch" => html! { <SwitchPage /> },
        "radio-group" | "/radio-group" => html! { <RadioGroupPage /> },
        "toggle-group" | "/toggle-group" => html! { <ToggleGroupPage /> },
        "popover" | "/popover" => html! { <PopoverPage /> },
        _ => html! {{ "Not Found!" }},
    }
}
