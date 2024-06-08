use slot::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let slot_ref = use_node_ref();
    let as_child = true;

    let props: Vec<(&str, AttrValue)> = vec![
        ("id", "button".into()),
        ("class", "bg-[#333] text-[#fff]".into()),
        ("style", "padding: 1rem;".into()),
    ];

    html! {
        <div class="bg-[#333] min-h-screen">
            <Slot arbitrary={props}>
                <div>{"Hello, world!"}</div>
            </Slot>
        </div>
    }
}
