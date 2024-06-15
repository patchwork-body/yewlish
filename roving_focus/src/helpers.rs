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

pub fn focus_child(child: Option<Element>) {
    if let Some(child) = child {
        let child = child.dyn_into::<HtmlElement>();

        if let Ok(child) = child {
            if let Err(error) = child.focus() {
                log::error!("Failed to focus the next child: {:?}", error);
            }
        }
    }
}

pub static FOCUSABLE_SELECTOR: &str =
    "button, a, input, select, textarea, [tabindex]:not([tabindex='-1'])";

pub static SKIP_FOCUS_ATTRIBUTES: [&str; 3] = ["disabled", "hidden", "aria-hidden"];

pub fn get_next_focusable_element(current_element: HtmlElement) -> HtmlElement {
    let all_focusable_elements = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector_all(FOCUSABLE_SELECTOR)
        .unwrap()
        .into_vec()
        .iter()
        .filter(|element| {
            SKIP_FOCUS_ATTRIBUTES
                .iter()
                .all(|attr| element.get_attribute(attr).is_none())
        })
        .cloned()
        .collect::<Vec<HtmlElement>>();

    let current_index = all_focusable_elements
        .iter()
        .position(|element| element == &current_element)
        .unwrap_or_default();

    let next_index = if current_index == all_focusable_elements.len() - 1 {
        0
    } else {
        current_index + 1
    };

    all_focusable_elements[next_index].clone()
}

pub fn get_prev_focusable_element(current_element: HtmlElement) -> HtmlElement {
    let all_focusable_elements = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector_all(FOCUSABLE_SELECTOR)
        .unwrap()
        .into_vec()
        .iter()
        .filter(|element| {
            SKIP_FOCUS_ATTRIBUTES
                .iter()
                .all(|attr| element.get_attribute(attr).is_none())
        })
        .cloned()
        .collect::<Vec<HtmlElement>>();

    let current_index = all_focusable_elements
        .iter()
        .position(|element| element == &current_element)
        .unwrap_or_default();

    let prev_index = if current_index == 0 {
        all_focusable_elements.len() - 1
    } else {
        current_index - 1
    };

    all_focusable_elements[prev_index].clone()
}
