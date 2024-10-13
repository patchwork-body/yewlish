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

    #[allow(clippy::cast_precision_loss)]
    let total_height = (total_items as f64) * item_height;
    let first_visible = (*scroll_top / item_height).floor() as usize;
    let visible_count = (viewport_height / item_height).ceil() as usize + 1;
    let last_visible = (first_visible + visible_count).min(total_items);

    #[allow(clippy::cast_precision_loss)]
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
            style={format!("height: {viewport_height}px; overflow-y: auto; position: relative;")}
            {onscroll}
        >
            <div style={format!("height: {total_height}px; position: relative;")}>
                <div style={format!("transform: translateY({padding_top}px);")}>
                    { visible_items }
                </div>
            </div>
        </div>
    }
}
