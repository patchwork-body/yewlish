use std::{fmt::Debug, time::Instant};
use std::{fmt::Formatter, future::Future, pin::Pin, time::Duration};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::platform::time::sleep;

pub fn node_list_to_vec(node_list: web_sys::NodeList) -> Vec<web_sys::Element> {
    (0..node_list.length())
        .filter_map(|i| {
            node_list
                .item(i)
                .map(|node| node.dyn_into::<web_sys::Element>().unwrap_throw())
        })
        .collect()
}

pub trait Query {
    fn query_by_role(&self, role: &str) -> Self;
    fn query_by_text(&self, text: &str) -> Self;
    fn query_by_testid(&self, testid: &str) -> Self;

    fn query_all_by_role(&self, role: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    fn query_all_by_text(&self, text: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;

    fn query_all_by_testid(&self, testid: &str) -> Vec<Self>
    where
        Self: std::marker::Sized;
}

pub trait Event {
    fn click(self) -> Pin<Box<dyn Future<Output = Self>>>;
    fn keydown(self, key: &str) -> Pin<Box<dyn Future<Output = Self>>>;
}

pub trait Extractor {
    fn attribute(&self, name: &str) -> Option<String>;
    fn text(&self) -> String;
}

pub struct Tester {
    root: Option<web_sys::Element>,
}

impl Debug for Tester {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.root {
            Some(root) => write!(
                f,
                "Tester {{ root: Some({}), outer_html: {} }}",
                root.tag_name(),
                root.outer_html()
            ),
            None => write!(f, "Tester {{ root: None }}"),
        }
    }
}

impl Tester {
    pub fn new(root: web_sys::Element) -> Self {
        Self { root: Some(root) }
    }

    pub fn exists(&self) -> bool {
        self.root.is_some()
    }

    pub async fn wait_for<F>(&self, timeout: u64, check_fn: F) -> bool
    where
        F: Fn() -> bool,
    {
        let start = Instant::now();

        while start.elapsed() < Duration::from_millis(timeout) {
            if check_fn() {
                return true;
            }

            sleep(Duration::from_millis(100)).await;
        }

        false
    }

    pub fn query_by_selector(&self, selector: &str) -> Self {
        match &self.root {
            Some(root) => match root.query_selector(selector) {
                Ok(element) => Self { root: element },
                Err(error) => {
                    panic!("Failed to query by selector: {:?}", error);
                }
            },
            None => {
                panic!("Root element is None");
            }
        }
    }

    pub fn query_all_by_selector(&self, selector: &str) -> Vec<Self> {
        match &self.root {
            Some(root) => match root.query_selector_all(selector) {
                Ok(node_list) => node_list_to_vec(node_list)
                    .iter()
                    .map(|node| Self {
                        root: node.clone().into(),
                    })
                    .collect(),
                Err(error) => {
                    panic!("Failed to query all by selector: {:?}", error);
                }
            },
            None => {
                panic!("Root element is None");
            }
        }
    }
}

impl Query for Tester {
    fn query_by_role(&self, role: &str) -> Self {
        self.query_by_selector(&format!("[role='{}']", role))
    }

    fn query_by_text(&self, text: &str) -> Self {
        self.query_all_by_selector("*")
            .into_iter()
            .find(|element| element.text().contains(text))
            .unwrap_or_else(|| Tester { root: None })
    }

    fn query_by_testid(&self, testid: &str) -> Self {
        self.query_by_selector(&format!("[data-testid='{}']", testid))
    }

    fn query_all_by_role(&self, role: &str) -> Vec<Self> {
        self.query_all_by_selector(&format!("[role='{}']", role))
    }

    fn query_all_by_text(&self, text: &str) -> Vec<Self> {
        self.query_all_by_selector("*")
            .into_iter()
            .filter(|element| element.text().contains(text))
            .collect()
    }

    fn query_all_by_testid(&self, testid: &str) -> Vec<Self> {
        self.query_all_by_selector(&format!("[data-testid='{}']", testid))
    }
}

impl Event for Tester {
    fn click(self) -> Pin<Box<dyn Future<Output = Self>>> {
        match &self.root {
            Some(root) => {
                if root.has_attribute("disabled") {
                    return Box::pin(async move {
                        sleep(Duration::new(0, 0)).await;
                        self
                    });
                }

                let click_event_init_dict = web_sys::MouseEventInit::new();
                click_event_init_dict.set_bubbles(true);
                click_event_init_dict.set_cancelable(true);
                click_event_init_dict.set_composed(true);
                click_event_init_dict.set_button(0);

                let click_event = web_sys::MouseEvent::new_with_mouse_event_init_dict(
                    "click",
                    &click_event_init_dict,
                )
                .unwrap_throw();

                let _ = root
                    .dyn_ref::<web_sys::EventTarget>()
                    .unwrap_throw()
                    .dispatch_event(&click_event);

                Box::pin(async move {
                    sleep(Duration::new(0, 0)).await;
                    self
                })
            }
            None => Box::pin(async move {
                sleep(Duration::new(0, 0)).await;
                self
            }),
        }
    }

    fn keydown(self, key: &str) -> Pin<Box<dyn Future<Output = Self>>> {
        match &self.root {
            Some(root) => {
                let keydown_event_init_dict = web_sys::KeyboardEventInit::new();
                keydown_event_init_dict.set_bubbles(true);
                keydown_event_init_dict.set_cancelable(true);
                keydown_event_init_dict.set_composed(true);
                keydown_event_init_dict.set_key(key);

                let keydown_event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
                    "keydown",
                    &keydown_event_init_dict,
                )
                .expect("Failed to create keydown event");

                let _ = root
                    .dyn_ref::<web_sys::EventTarget>()
                    .expect("Failed to cast element to EventTarget")
                    .dispatch_event(&keydown_event);

                Box::pin(async move {
                    sleep(Duration::new(0, 0)).await;
                    self
                })
            }
            None => Box::pin(async move {
                sleep(Duration::new(0, 0)).await;
                self
            }),
        }
    }
}

impl Extractor for Tester {
    fn attribute(&self, name: &str) -> Option<String> {
        match &self.root {
            Some(root) => root.get_attribute(name),
            None => None,
        }
    }

    fn text(&self) -> String {
        match &self.root {
            Some(root) => root.text_content().unwrap_or_default(),
            None => "".to_string(),
        }
    }
}

#[cfg(feature = "internal")]
#[cfg(test)]
mod tests {
    use crate::{render, Event, Extractor, Query};
    use wasm_bindgen_test::*;
    use web_sys::wasm_bindgen::JsCast;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_query_by_selector() {
        let t = render! {
            <div id="test"></div>
        }
        .await;

        t.query_by_selector("#test");
    }

    #[wasm_bindgen_test]
    async fn test_extract_text() {
        let t = render! {
            <div id="test">{"Hello"}</div>
        }
        .await;

        assert_eq!(t.query_by_selector("#test").text(), "Hello");
    }

    #[wasm_bindgen_test]
    async fn test_extract_attribute() {
        let t = render! {
            <div id="test" data-test="test"></div>
        }
        .await;

        assert_eq!(
            t.query_by_selector("#test").attribute("data-test"),
            "test".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_extract_not_existing_attribute() {
        let t = render! {
            <div id="test"></div>
        }
        .await;

        assert_eq!(t.query_by_selector("#test").attribute("data-test"), None);
    }

    #[wasm_bindgen_test]
    async fn test_click() {
        let t = render! {
            <button id="test" onclick={
                Callback::from(|event: MouseEvent| {
                    let target = event.target().unwrap();
                    let element = target.dyn_into::<web_sys::HtmlElement>().unwrap();
                    element.set_inner_text("Clicked");
                })
            }>
                {"Click me"}
            </button>
        }
        .await;

        let button = t.query_by_selector("#test");
        assert_eq!(button.text(), "Click me");

        let button = button.click().await;
        assert_eq!(button.text(), "Clicked");
    }

    #[wasm_bindgen_test]
    async fn test_click_when_disabled() {
        let t = render! {
            <button id="test" disabled=true onclick={
                Callback::from(|event: MouseEvent| {
                    let target = event.target().unwrap();
                    let element = target.dyn_into::<web_sys::HtmlElement>().unwrap();
                    element.set_inner_text("Clicked");
                })
            }>
                {"Click me"}
            </button>
        }
        .await;

        let button = t.query_by_selector("#test");
        assert_eq!(button.text(), "Click me");

        let button = button.click().await;
        assert_eq!(button.text(), "Click me");
    }

    #[wasm_bindgen_test]
    async fn test_query_by_text() {
        let t = render! {
            <div id="test">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_text("Hello").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_text_not_found() {
        let t = render! {
            <div id="test">{"Hello"}</div>
        }
        .await;

        assert!(!t.query_by_text("World").exists());
    }
}
