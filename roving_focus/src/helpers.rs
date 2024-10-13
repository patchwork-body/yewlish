use web_sys::wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, NodeList};

pub trait NodeListIntoVec {
    fn into_vec(self) -> Vec<HtmlElement>;
}

impl NodeListIntoVec for NodeList {
    fn into_vec(self) -> Vec<HtmlElement> {
        let mut elements = Vec::new();
        for index in 0..self.length() {
            if let Some(node) = self.item(index) {
                if let Ok(html_element) = node.dyn_into::<HtmlElement>() {
                    elements.push(html_element);
                }
            }
        }

        elements
    }
}

pub static FOCUSABLE_ELEMENTS: [&str; 5] = ["button", "a", "input", "select", "textarea"];

pub static FOCUSABLE_SELECTOR: &str =
    "button, a, input, select, textarea, [tabindex]:not([tabindex='-1'])";

pub static SKIP_FOCUS_ATTRIBUTES: [&str; 3] = ["disabled", "hidden", "aria-hidden"];

#[must_use]
pub fn is_focusable(element: &HtmlElement) -> bool {
    let is_focusable_element =
        FOCUSABLE_ELEMENTS.contains(&element.tag_name().to_lowercase().as_str());

    let has_no_skip_attributes = SKIP_FOCUS_ATTRIBUTES
        .iter()
        .all(|&attr| element.get_attribute(attr).is_none());

    let has_valid_tabindex = element
        .get_attribute("tabindex")
        .map_or(false, |tabindex| tabindex != "-1");

    (is_focusable_element || has_valid_tabindex) && has_no_skip_attributes
}

#[must_use]
pub fn get_focusable_element(element: &Element) -> Option<HtmlElement> {
    if let Ok(element) = <web_sys::Element as Clone>::clone(element).dyn_into::<HtmlElement>() {
        if is_focusable(&element) {
            return Some(element);
        }

        let children = element.children();

        for index in 0..children.length() {
            let child_option = children.item(index);
            let child = match child_option {
                Some(child) => child.dyn_into::<HtmlElement>().ok(),
                None => None,
            };

            if let Some(child) = child {
                if is_focusable(&child) {
                    return Some(child);
                }
            }

            continue;
        }
    }

    None
}

pub fn focus_child(child: Option<Element>) {
    if let Some(child) = child {
        let child = child.dyn_into::<HtmlElement>();

        if let Ok(child) = child {
            let child = get_focusable_element(&child);

            if let Some(child) = child {
                if let Err(error) = child.focus() {
                    log::error!("Failed to focus the next child: {:?}", error);
                }
            } else {
                log::error!("No focusable child found");
            }
        }
    }
}

#[must_use]
pub fn get_all_focusable_elements() -> Vec<HtmlElement> {
    match window() {
        Some(window) => match window.document() {
            Some(document) => match document.query_selector_all(FOCUSABLE_SELECTOR) {
                Ok(node_list) => node_list
                    .into_vec()
                    .iter()
                    .filter_map(|element| element.dyn_ref::<HtmlElement>())
                    .filter(|element| {
                        SKIP_FOCUS_ATTRIBUTES
                            .iter()
                            .all(|attr| element.get_attribute(attr).is_none())
                    })
                    .cloned()
                    .collect::<Vec<HtmlElement>>(),
                Err(_) => Vec::new(),
            },
            None => Vec::new(),
        },
        None => Vec::new(),
    }
}

#[must_use]
pub fn get_next_focusable_element(current_element: &HtmlElement) -> HtmlElement {
    let all_focusable_elements = get_all_focusable_elements();

    let current_index = all_focusable_elements
        .iter()
        .position(|element| element == current_element)
        .unwrap_or_default();

    let next_index = if current_index == all_focusable_elements.len() - 1 {
        0
    } else {
        current_index + 1
    };

    all_focusable_elements[next_index].clone()
}

#[must_use]
pub fn get_prev_focusable_element(current_element: &HtmlElement) -> HtmlElement {
    let all_focusable_elements = get_all_focusable_elements();

    let current_index = all_focusable_elements
        .iter()
        .position(|element| element == current_element)
        .unwrap_or_default();

    let prev_index = if current_index == 0 {
        all_focusable_elements.len() - 1
    } else {
        current_index - 1
    };

    all_focusable_elements[prev_index].clone()
}
