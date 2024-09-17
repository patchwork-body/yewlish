use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VirtualListProps {
    pub total_items: usize,
    pub item_height: f64,
    pub viewport_height: f64,
    pub render_item: Callback<usize, Html>,
}

#[function_component(VirtualList)]
pub fn virtual_list(props: &VirtualListProps) -> Html {
    let scroll_top = use_state(|| 0.0);

    let total_items = props.total_items;
    let item_height = props.item_height;
    let viewport_height = props.viewport_height;
    let render_item = &props.render_item;

    let total_height = total_items as f64 * item_height;
    let first_visible = (*scroll_top / item_height).floor() as usize;
    let visible_count = (viewport_height / item_height).ceil() as usize + 1;
    let last_visible = (first_visible + visible_count).min(total_items);

    let padding_top = first_visible as f64 * item_height;

    let visible_items = (first_visible..last_visible)
        .map(|index| render_item.emit(index))
        .collect::<Html>();

    let onscroll = use_callback((), {
        let scroll_top = scroll_top.clone();

        move |event: Event, (): &()| {
            let target = event.target().unwrap();
            let scroll_top_value = target.unchecked_into::<HtmlElement>().scroll_top() as f64;
            scroll_top.set(scroll_top_value);
        }
    });

    html! {
        <div
            style={format!("height: {}px; overflow-y: auto; position: relative;", viewport_height)}
            {onscroll}
        >
            <div style={format!("height: {}px; position: relative;", total_height)}>
                <div style={format!("transform: translateY({}px);", padding_top)}>
                    { visible_items }
                </div>
            </div>
        </div>
    }
}
