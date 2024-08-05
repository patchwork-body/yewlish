pub mod helpers;
pub mod hooks;

use dir::Dir;
use helpers::*;
use hooks::{use_children_as_html_collection::*, use_roving_iterator::*};
use orientation::Orientation;
use utils::{enums::*, hooks::use_keydown};
use web_sys::{wasm_bindgen::JsCast, HtmlElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RovingFocusProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or(Dir::Ltr)]
    pub dir: Dir,
    #[prop_or(true)]
    pub r#loop: bool,
    #[prop_or_default]
    pub role: Option<AttrValue>,
}

#[function_component(RovingFocus)]
pub fn roving_focus(props: &RovingFocusProps) -> Html {
    let roving_iterator =
        use_roving_iterator(props.children.len() as u32, props.r#loop, &props.dir);
    let node_ref = use_node_ref();
    let children_as_html_collection = use_children_as_html_collection(node_ref.clone());
    let is_focus_entered = use_mut_ref(|| false);

    let navigation_handler = {
        let is_focus_entered = is_focus_entered.clone();
        let children_as_html_collection = children_as_html_collection.clone();
        let roving_iterator = roving_iterator.clone();
        let orientation = props.orientation.clone();
        let dir = props.dir.clone();

        move |event: KeyboardEvent| {
            let children_as_html_collection = children_as_html_collection.borrow();
            let children = children_as_html_collection.as_ref();

            if children.is_none() {
                return;
            }

            let children = children.unwrap();

            let next_index = match event.key().as_str() {
                "ArrowDown" => match orientation {
                    Orientation::Vertical => roving_iterator.borrow_mut().next(&dir),
                    Orientation::Horizontal => roving_iterator.borrow_mut().prev(&dir),
                },
                "ArrowUp" => match orientation {
                    Orientation::Vertical => roving_iterator.borrow_mut().prev(&dir),
                    Orientation::Horizontal => roving_iterator.borrow_mut().next(&dir),
                },
                "ArrowLeft" => roving_iterator.borrow_mut().prev(&dir),
                "ArrowRight" => roving_iterator.borrow_mut().next(&dir),
                "Home" => roving_iterator.borrow_mut().first(&dir),
                "End" => roving_iterator.borrow_mut().last(&dir),
                "Tab" => {
                    let last_focusable_element_index = if event.shift_key() {
                        0
                    } else {
                        roving_iterator.borrow().length - 1
                    };

                    children
                        .item(last_focusable_element_index)
                        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
                        .and_then(|html_element| get_focusable_element(&html_element))
                        .map(|html_element| {
                            if event.shift_key() {
                                get_prev_focusable_element(html_element)
                            } else {
                                get_next_focusable_element(html_element)
                            }
                        })
                        .and_then(|next_outside_focusable_element| {
                            next_outside_focusable_element.focus().err()
                        })
                        .map_or_else(
                            || {},
                            |error| log::error!("Failed to focus the element: {:?}", error),
                        );

                    *is_focus_entered.borrow_mut() = false;
                    None
                }
                _ => None,
            };

            if let Some(next_index) = next_index {
                focus_child(children.item(next_index));
            }
        }
    };

    let navigate_through_children = use_keydown(
        vec![
            "ArrowDown".to_string(),
            "ArrowUp".to_string(),
            "ArrowLeft".to_string(),
            "ArrowRight".to_string(),
            "Home".to_string(),
            "End".to_string(),
            "Tab".to_string(),
        ],
        navigation_handler,
    );

    let focus_last_focused_child = use_callback(
        (
            roving_iterator.clone(),
            children_as_html_collection.clone(),
            is_focus_entered.clone(),
        ),
        move |_event: FocusEvent,
              (roving_iterator, children_as_html_collection, is_focus_entered)| {
            if *is_focus_entered.borrow() {
                return;
            }

            let children_as_html_collection = children_as_html_collection.borrow();
            let children = children_as_html_collection.as_ref();

            if children.is_none() {
                return;
            }

            let children = children.unwrap();
            focus_child(children.item(roving_iterator.borrow().current));
            *is_focus_entered.borrow_mut() = true;
        },
    );

    html! {
        <div role={props.role.clone()} class={&props.class} data-orientation={props.orientation.clone()} ref={node_ref} onfocusin={&focus_last_focused_child} onkeydown={&navigate_through_children}>
            {for props.children.iter()}
        </div>
    }
}
