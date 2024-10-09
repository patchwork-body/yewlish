mod event;
mod extractor;
mod query;
pub use event::TesterEvent;
pub use extractor::Extractor;
pub use query::Query;

use std::fmt::Debug;
use std::{any::Any, cell::RefCell, rc::Rc};
use std::{fmt::Formatter, future::Future, pin::Pin, time::Duration};
use web_sys::wasm_bindgen::UnwrapThrowExt;
use web_sys::{js_sys::Date, wasm_bindgen::JsCast};
use yew::platform::time::sleep;

pub type ResultRef = Rc<RefCell<Option<Box<dyn Any>>>>;

/// The `HookTester` struct is designed to facilitate testing of hooks in a Yew application.
///
/// This struct provides methods to create a new `HookTester` instance and retrieve the inner value
/// of a specified type. It leverages Rust's reference counting and interior mutability to manage
/// the inner value safely across multiple references.
///
/// # Examples
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// use yewlish_testing_tools::tester::{HookTester, ResultRef};
///
/// let result_ref: ResultRef = Rc::new(RefCell::new(Some(Box::new(42))));
/// let tester = HookTester::new(result_ref.clone());
///
/// let value: i32 = tester.get();
/// assert_eq!(value, 42);
/// ```
///
/// # Fields
///
/// - `inner`: A reference-counted cell containing an optional boxed value of any type.
#[derive(Debug, Clone)]
pub struct HookTester {
    inner: ResultRef,
}

impl HookTester {
    /// Creates a new `HookTester` instance.
    ///
    /// # Arguments
    ///
    /// * `inner` - A reference-counted cell containing an optional boxed value of any type.
    ///
    /// # Returns
    ///
    /// A new instance of `HookTester`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use yewlish_testing_tools::tester::{HookTester, ResultRef};
    ///
    /// let result_ref: ResultRef = Rc::new(RefCell::new(Some(Box::new(42))));
    /// let tester = HookTester::new(result_ref.clone());
    /// ```
    pub fn new(inner: ResultRef) -> Self {
        Self { inner }
    }

    #[must_use]
    /// Retrieves the inner value of the specified type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to which the inner value should be downcast.
    ///
    /// # Returns
    ///
    /// The inner value of type `T`.
    ///
    /// # Panics
    ///
    /// This function will panic if the inner value cannot be downcast to the specified type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use yewlish_testing_tools::tester::{HookTester, ResultRef};
    ///
    /// let result_ref: ResultRef = Rc::new(RefCell::new(Some(Box::new(42))));
    /// let tester = HookTester::new(result_ref.clone());
    ///
    /// let value: i32 = tester.get();
    /// assert_eq!(value, 42);
    /// ```
    pub fn get<T: 'static + Clone>(&self) -> T {
        self.inner
            .borrow()
            .as_ref()
            .and_then(|v| v.downcast_ref::<T>())
            .unwrap_or_else(|| {
                panic!(
                    "HookTester: type mismatch. Expected type is: {:?}",
                    std::any::type_name::<T>()
                )
            })
            .clone()
    }

    /// Executes an asynchronous action and waits for it to complete.
    ///
    /// The `act` method is designed to facilitate testing by allowing you to perform an action
    /// and then wait for it to complete. This is particularly useful in scenarios where you need
    /// to ensure that certain asynchronous operations have finished before proceeding with your tests.
    ///
    /// # Arguments
    ///
    /// * `action` - A closure representing the action to be performed. The closure should not take any arguments and should not return any value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use yewlish_testing_tools::tester::{HookTester, ResultRef};
    ///
    /// let result_ref: ResultRef = Rc::new(RefCell::new(Some(Box::new(42))));
    /// let tester = HookTester::new(result_ref.clone());
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result_ref: ResultRef = Rc::new(RefCell::new(Some(Box::new(42))));
    ///     let tester = HookTester::new(result_ref.clone());
    ///
    ///     tester.act(|| {
    ///         // Perform some action here
    ///     }).await;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances. However, if the provided closure panics,
    /// the panic will propagate.
    ///
    /// # Type Parameters
    ///
    /// - `F`: The type of the closure to be executed. It must implement the `FnOnce` trait.
    pub async fn act<F>(&self, action: F)
    where
        F: FnOnce(),
    {
        action();
        sleep(Duration::ZERO).await;
    }
}

#[must_use]
pub fn node_list_to_vec(node_list: &web_sys::NodeList) -> Vec<web_sys::Element> {
    (0..node_list.length())
        .filter_map(|i| {
            node_list
                .item(i)
                .map(|node| node.dyn_into::<web_sys::Element>().unwrap_throw())
        })
        .collect()
}

pub struct Tester {
    root: Option<web_sys::Element>,
    state: HookTester,
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
    pub fn new(root: web_sys::Element, result_ref: ResultRef) -> Self {
        Self {
            root: Some(root),
            state: HookTester::new(result_ref),
        }
    }

    #[must_use]
    pub fn exists(&self) -> bool {
        self.root.is_some()
    }

    pub async fn wait_for<F>(&self, timeout: f64, check_fn: F) -> bool
    where
        F: Fn() -> bool,
    {
        let start = Date::now();

        while Date::now() - start < timeout {
            if check_fn() {
                return true;
            }

            sleep(Duration::from_millis(100)).await;
        }

        false
    }

    pub async fn act<F>(&self, action: F)
    where
        F: FnOnce(),
    {
        self.state.act(action).await;
    }

    #[must_use]
    /// Queries the root element by the given CSS selector.
    ///
    /// # Panics
    ///
    /// This function will panic if the root element is `None` or if the query selector fails.
    pub fn query_by_selector(&self, selector: &str) -> Self {
        match &self.root {
            Some(root) => match root.query_selector(selector) {
                Ok(element) => Self {
                    root: element,
                    state: self.state.clone(),
                },
                Err(error) => {
                    panic!("Failed to query by selector: {error:?}");
                }
            },
            None => {
                panic!("Root element is None");
            }
        }
    }

    #[must_use]
    /// Queries all elements by the given CSS selector.
    ///
    /// # Panics
    ///
    /// This function will panic if the root element is `None` or if the query selector fails.
    pub fn query_all_by_selector(&self, selector: &str) -> Vec<Self> {
        match &self.root {
            Some(root) => match root.query_selector_all(selector) {
                Ok(node_list) => node_list_to_vec(&node_list)
                    .iter()
                    .map(|node| Self {
                        root: node.clone().into(),
                        state: self.state.clone(),
                    })
                    .collect(),
                Err(error) => {
                    panic!("Failed to query all by selector: {error:?}");
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
        let implicit_selector = self.build_role_query(role);
        let explicit_selector = format!("[role='{role}']");

        if implicit_selector.is_empty() {
            return self.query_by_selector(&explicit_selector);
        }

        self.query_by_selector(&format!("{implicit_selector}, {explicit_selector}"))
    }

    fn query_by_text(&self, text: &str) -> Self {
        self.query_all_by_selector("*")
            .into_iter()
            .find(|element| element.text().contains(text))
            .unwrap_or_else(|| Tester {
                root: None,
                state: self.state.clone(),
            })
    }

    fn query_by_testid(&self, testid: &str) -> Self {
        self.query_by_selector(&format!("[data-testid='{testid}']"))
    }

    fn query_all_by_role(&self, role: &str) -> Vec<Self> {
        self.query_all_by_selector(&format!("[role='{role}']"))
    }

    fn query_all_by_text(&self, text: &str) -> Vec<Self> {
        self.query_all_by_selector("*")
            .into_iter()
            .filter(|element| element.text().contains(text))
            .collect()
    }

    fn query_all_by_testid(&self, testid: &str) -> Vec<Self> {
        self.query_all_by_selector(&format!("[data-testid='{testid}']"))
    }
}

impl TesterEvent for Tester {
    fn click(self) -> Pin<Box<dyn Future<Output = Self>>> {
        match &self.root {
            Some(root) => {
                if root.has_attribute("disabled") {
                    return Box::pin(async move {
                        sleep(Duration::ZERO).await;
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
                    sleep(Duration::ZERO).await;
                    self
                })
            }
            None => Box::pin(async move {
                sleep(Duration::ZERO).await;
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
                    sleep(Duration::ZERO).await;
                    self
                })
            }
            None => Box::pin(async move {
                sleep(Duration::ZERO).await;
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
            None => String::new(),
        }
    }

    fn get_state<T: Clone + 'static>(&self) -> T {
        self.state.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::{render, Extractor, Query, TesterEvent};
    use std::rc::Rc;
    use wasm_bindgen_test::*;
    use web_sys::wasm_bindgen::JsCast;
    use yew::prelude::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_query_by_selector() {
        let t = render!({
            html! {
                <div id="test"></div>
            }
        })
        .await;

        let test_div = t.query_by_selector("#test");
        assert!(test_div.exists());
    }

    #[wasm_bindgen_test]
    async fn test_extract_text() {
        let t = render!({
            html! {
                <div id="test">{"Hello"}</div>
            }
        })
        .await;

        assert_eq!(t.query_by_selector("#test").text(), "Hello");
    }

    #[wasm_bindgen_test]
    async fn test_extract_attribute() {
        let t = render!({
            html! {
                <div id="test" data-test="test"></div>
            }
        })
        .await;

        assert_eq!(
            t.query_by_selector("#test").attribute("data-test"),
            "test".to_string().into()
        );
    }

    #[wasm_bindgen_test]
    async fn test_extract_not_existing_attribute() {
        let t = render!({
            html! {
                <div id="test"></div>
            }
        })
        .await;

        assert_eq!(t.query_by_selector("#test").attribute("data-test"), None);
    }

    #[wasm_bindgen_test]
    async fn test_click() {
        let t = render!({
            html! {
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
        })
        .await;

        let button = t.query_by_selector("#test");
        assert_eq!(button.text(), "Click me");

        let button = button.click().await;
        assert_eq!(button.text(), "Clicked");
    }

    #[wasm_bindgen_test]
    async fn test_click_when_disabled() {
        let t = render!({
            html! {
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
        })
        .await;

        let button = t.query_by_selector("#test");
        assert_eq!(button.text(), "Click me");

        let button = button.click().await;
        assert_eq!(button.text(), "Click me");
    }

    #[wasm_bindgen_test]
    async fn test_query_by_text() {
        let t = render!({
            html! {
                <div id="test">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_text("Hello").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_text_not_found() {
        let t = render!({
            html! {
                <div id="test">{"Hello"}</div>
            }
        })
        .await;

        assert!(!t.query_by_text("World").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_link() {
        let t = render!({
            html! {
                <span role="link">{"Hello"}</span>
            }
        })
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_a_with_href_has_implicit_role_link() {
        let t = render!({
            html! {
                <a href="http://example.com">{"Hello"}</a>
            }
        })
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_area_with_href_has_implicit_role_link() {
        let t = render!({
            html! {
                <map name="map">
                    <area href="http://example.com" shape="rect" coords="34,44,270,350" />
                </map>
            }
        })
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_a_without_href_does_not_have_implicit_role_link() {
        let t = render!({
            html! {
                <a>{"Hello"}</a>
            }
        })
        .await;

        assert!(!t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_area_without_href_does_not_have_implicit_role_link() {
        let t = render!({
            html! {
                <map name="map">
                    <area shape="rect" coords="34,44,270,350" />
                </map>
            }
        })
        .await;

        assert!(!t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_generic() {
        let t = render!({
            html! {
                <span role="generic">{"Hello"}</span>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_b_has_implicit_role_generic() {
        let t = render!({
            html! {
                <b>{"Hello"}</b>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_bdi_has_implicit_role_generic() {
        let t = render!({
            html! {
                <bdi>{"Hello"}</bdi>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_bdo_has_implicit_role_generic() {
        let t = render!({
            html! {
                <bdo>{"Hello"}</bdo>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_body_has_implicit_role_generic() {
        let t = render!({
            html! {
                <body>{"Hello"}</body>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_data_has_implicit_role_generic() {
        let t = render!({
            html! {
                <data>{"Hello"}</data>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_div_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div>{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_article_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <article>
                    <footer>{"Hello"}</footer>
                </article>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_aside_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <aside>
                    <footer>{"Hello"}</footer>
                </aside>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_main_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <main>
                    <footer>{"Hello"}</footer>
                </main>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <nav>
                    <footer>{"Hello"}</footer>
                </nav>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <section>
                    <footer>{"Hello"}</footer>
                </section>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_article_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="article">
                    <footer>{"Hello"}</footer>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_complementary_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="complementary">
                    <footer>{"Hello"}</footer>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_main_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="main">
                    <footer>{"Hello"}</footer>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_navigation_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="navigation">
                    <footer>{"Hello"}</footer>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_region_footer_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="region">
                    <footer>{"Hello"}</footer>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_article_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <article>
                    <header>{"Hello"}</header>
                </article>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_aside_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <aside>
                    <header>{"Hello"}</header>
                </aside>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_main_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <main>
                    <header>{"Hello"}</header>
                </main>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <nav>
                    <header>{"Hello"}</header>
                </nav>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <section>
                    <header>{"Hello"}</header>
                </section>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_article_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="article">
                    <header>{"Hello"}</header>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_complementary_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="complementary">
                    <header>{"Hello"}</header>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_main_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="main">
                    <header>{"Hello"}</header>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_navigation_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="navigation">
                    <header>{"Hello"}</header>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_region_header_has_implicit_role_generic() {
        let t = render!({
            html! {
                <div role="region">
                    <header>{"Hello"}</header>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_i_has_implicit_role_generic() {
        let t = render!({
            html! {
                <i>{"Hello"}</i>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_li_has_implicit_role_generic() {
        let t = render!({
            html! {
                <li>{"Hello"}</li>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_pre_has_implicit_role_generic() {
        let t = render!({
            html! {
                <pre>{"Hello"}</pre>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_q_has_implicit_role_generic() {
        let t = render!({
            html! {
                <q>{"Hello"}</q>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_samp_has_implicit_role_generic() {
        let t = render!({
            html! {
                <samp>{"Hello"}</samp>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_has_implicit_role_generic() {
        let t = render!({
            html! {
                <section>{"Hello"}</section>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_does_not_have_implicit_role_generic() {
        let t = render!({
            html! {
                <section aria-label="Hello">{"Hello"}</section>
            }
        })
        .await;

        assert!(!t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_small_has_implicit_role_generic() {
        let t = render!({
            html! {
                <small>{"Hello"}</small>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_span_has_implicit_role_generic() {
        let t = render!({
            html! {
                <span>{"Hello"}</span>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_u_has_implicit_role_generic() {
        let t = render!({
            html! {
                <u>{"Hello"}</u>
            }
        })
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_group() {
        let t = render!({
            html! {
                <span role="group">{"Hello"}</span>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_address_has_implicit_role_group() {
        let t = render!({
            html! {
                <address>{"Hello"}</address>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_details_has_implicit_role_group() {
        let t = render!({
            html! {
                <details>{"Hello"}</details>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_fieldset_has_implicit_role_group() {
        let t = render!({
            html! {
                <fieldset>{"Hello"}</fieldset>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_hgroup_has_implicit_role_group() {
        let t = render!({
            html! {
                <hgroup>{"Hello"}</hgroup>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_optgroup_has_implicit_role_group() {
        let t = render!({
            html! {
                <optgroup>{"Hello"}</optgroup>
            }
        })
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_article() {
        let t = render!({
            html! {
                <div role="article">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("article").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_article() {
        let t = render!({
            html! {
                <article>{"Hello"}</article>
            }
        })
        .await;

        assert!(t.query_by_role("article").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_complementary() {
        let t = render!({
            html! {
                <div role="complementary">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("complementary").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_aside() {
        let t = render!({
            html! {
                <aside>{"Hello"}</aside>
            }
        })
        .await;

        assert!(t.query_by_role("complementary").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_blockquote() {
        let t = render!({
            html! {
                <div role="blockquote">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("blockquote").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_blockquote() {
        let t = render!({
            html! {
                <blockquote>{"Hello"}</blockquote>
            }
        })
        .await;

        assert!(t.query_by_role("blockquote").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_button() {
        let t = render!({
            html! {
                <div role="button">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_button_has_implicit_role_button() {
        let t = render!({
            html! {
                <button>{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_button_has_implicit_role_button() {
        let t = render!({
            html! {
                <input type="button" value="Hello" />
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_image_has_implicit_role_button() {
        let t = render!({
            html! {
                <input type="image" src="http://example.com/image.png" />
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_reset_has_implicit_role_button() {
        let t = render!({
            html! {
                <input type="reset" value="Hello" />
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_submit_has_implicit_role_button() {
        let t = render!({
            html! {
                <input type="submit" value="Hello" />
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_summary_has_implicit_role_button() {
        let t = render!({
            html! {
                <summary>{"Hello"}</summary>
            }
        })
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_caption() {
        let t = render!({
            html! {
                <div role="caption">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("caption").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_caption() {
        let t = render!({
            html! {
                <caption>{"Hello"}</caption>
            }
        })
        .await;

        assert!(t.query_by_role("caption").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_code() {
        let t = render!({
            html! {
                <div role="code">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("code").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_code() {
        let t = render!({
            html! {
                <code>{"Hello"}</code>
            }
        })
        .await;

        assert!(t.query_by_role("code").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_listbox() {
        let t = render!({
            html! {
                <div role="listbox">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_datalist_has_implicit_role_datalist() {
        let t = render!({
            html! {
                <datalist>{"Hello"}</datalist>
            }
        })
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_multiple_has_implicit_role_listbox() {
        let t = render!({
            html! {
                <select multiple=true>{"Hello"}</select>
            }
        })
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_greater_than_2_has_implicit_role_listbox() {
        let t = render!({
            html! {
                <select size="2">
                    <option>{"Hello"}</option>
                    <option>{"World"}</option>
                </select>
            }
        })
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_deletion() {
        let t = render!({
            html! {
                <div role="deletion">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_del_with_implicit_role_deletion() {
        let t = render!({
            html! {
                <del>{"Hello"}</del>
            }
        })
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_s_with_implicit_role_deletion() {
        let t = render!({
            html! {
                <s>{"Hello"}</s>
            }
        })
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_term() {
        let t = render!({
            html! {
                <div role="term">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("term").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_dfn() {
        let t = render!({
            html! {
                <dfn>{"Hello"}</dfn>
            }
        })
        .await;

        assert!(t.query_by_role("term").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_dialog() {
        let t = render!({
            html! {
                <div role="dialog">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("dialog").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_dialog() {
        let t = render!({
            html! {
                <dialog>{"Hello"}</dialog>
            }
        })
        .await;

        assert!(t.query_by_role("dialog").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_emphasis() {
        let t = render!({
            html! {
                <div role="emphasis">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("emphasis").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_em() {
        let t = render!({
            html! {
                <em>{"Hello"}</em>
            }
        })
        .await;

        assert!(t.query_by_role("emphasis").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_figure() {
        let t = render!({
            html! {
                <div role="figure">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("figure").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_figure() {
        let t = render!({
            html! {
                <figure>{"Hello"}</figure>
            }
        })
        .await;

        assert!(t.query_by_role("figure").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_contentinfo() {
        let t = render!({
            html! {
                <div role="contentinfo">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("contentinfo").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_footer_has_implicit_role_contentinfo() {
        let t = render!({
            html! {
                <footer>{"Hello"}</footer>
            }
        })
        .await;

        assert!(t.query_by_role("contentinfo").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_form() {
        let t = render!({
            html! {
                <div role="form">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("form").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_form_has_implicit_role_form() {
        let t = render!({
            html! {
                <form>{"Hello"}</form>
            }
        })
        .await;

        assert!(t.query_by_role("form").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_heading() {
        let t = render!({
            html! {
                <div role="heading">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h1_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h1>{"Hello"}</h1>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h2_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h2>{"Hello"}</h2>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h3_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h3>{"Hello"}</h3>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h4_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h4>{"Hello"}</h4>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h5_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h5>{"Hello"}</h5>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h6_has_implicit_role_heading() {
        let t = render!({
            html! {
                <h6>{"Hello"}</h6>
            }
        })
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_header_has_implicit_role_banner() {
        let t = render!({
            html! {
                <header>{"Hello"}</header>
            }
        })
        .await;

        assert!(t.query_by_role("banner").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_separator() {
        let t = render!({
            html! {
                <div role="separator">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("separator").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_hr_has_implicit_role_separator() {
        let t = render!({
            html! {
                <hr />
            }
        })
        .await;

        assert!(t.query_by_role("separator").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_document() {
        let t = render!({
            html! {
                <div role="document">{"Hello"}</div>
            }
        })
        .await;

        assert!(t.query_by_role("document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_html_has_implicit_role_document() {
        let t = render!({
            html! {
                <html>{"Hello"}</html>
            }
        })
        .await;

        assert!(t.query_by_role("document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_checkbox() {
        let t = render!({
            html! {
                <button role="checkbox">{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("checkbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_checkbox_has_implicit_role_checkbox() {
        let t = render!({
            html! {
                <input type="checkbox" />
            }
        })
        .await;

        assert!(t.query_by_role("checkbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_textbox() {
        let t = render!({
            html! {
                <button role="textbox">{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_with_no_type_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <input />
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_text_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <input type="text" />
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <input type="email" />
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_with_list_does_not_have_implicit_role_textbox() {
        let t = render!({
            html! {
                <>
                    <input type="email" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <input type="url" />
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_with_list_does_not_have_implicit_role_textbox() {
        let t = render!({
            html! {
                <>
                    <input type="url" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <input type="tel" />
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_with_list_does_not_have_implicit_role_textbox() {
        let t = render!({
            html! {
                <>
                    <input type="tel" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_textarea_has_implicit_role_textbox() {
        let t = render!({
            html! {
                <textarea>{"Hello"}</textarea>
            }
        })
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_spinbutton() {
        let t = render!({
            html! {
                <button role="spinbutton">{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("spinbutton").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_number_has_implicit_role_spinbutton() {
        let t = render!({
            html! {
                <input type="number" />
            }
        })
        .await;

        assert!(t.query_by_role("spinbutton").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_radio() {
        let t = render!({
            html! {
                <button role="radio">{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("radio").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_radio_has_implicit_role_radio() {
        let t = render!({
            html! {
                <input type="radio" />
            }
        })
        .await;

        assert!(t.query_by_role("radio").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_range_has_implicit_role_slider() {
        let t = render!({
            html! {
                <input type="range" />
            }
        })
        .await;

        assert!(t.query_by_role("slider").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_has_implicit_role_searchbox() {
        let t = render!({
            html! {
                <input type="search" />
            }
        })
        .await;

        assert!(t.query_by_role("searchbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_with_list_does_not_have_role_searchbox() {
        let t = render!({
            html! {
                <>
                    <input type="search" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(!t.query_by_role("searchbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_combobox() {
        let t = render!({
            html! {
                <button role="combobox">{"Hello"}</button>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_text_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input type="text" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input type="email" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input type="tel" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input type="url" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_with_list_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <>
                    <input type="search" list="datalist" />

                    <datalist id="datalist">
                        <option value="Hello" />
                    </datalist>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <select>
                    <option value="Hello">{"Hello"}</option>
                </select>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_1_has_implicit_role_combobox() {
        let t = render!({
            html! {
                <select size="1">
                    <option value="Hello">{"Hello"}</option>
                </select>
            }
        })
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_multiple_does_not_have_implicit_role_combobox() {
        let t = render!({
            html! {
                <select multiple=true>
                    <option value="Hello">{"Hello"}</option>
                </select>
            }
        })
        .await;

        assert!(!t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_greater_than_1_does_not_have_implicit_role_combobox(
    ) {
        let t = render!({
            html! {
                <select size="2">
                    <option value="Hello">{"Hello"}</option>
                    <option value="Hello">{"Hello"}</option>
                </select>
            }
        })
        .await;

        assert!(!t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_time_does_not_have_implicit_role() {
        let t = render!({
            html! {
                <input type="time" />
            }
        })
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_week_does_not_have_implicit_role() {
        let t = render!({
            html! {
                <input type="week" />
            }
        })
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ins_has_implicit_role_insertion() {
        let t = render!({
            html! {
                <ins>{"Hello"}</ins>
            }
        })
        .await;

        assert!(t.query_by_role("insertion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ul_li_has_implicit_role_listitem() {
        let t = render!({
            html! {
                <ul>
                    <li>{"Hello"}</li>
                </ul>
            }
        })
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ol_li_has_implicit_role_listitem() {
        let t = render!({
            html! {
                <ol>
                    <li>{"Hello"}</li>
                </ol>
            }
        })
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_menu_li_has_implicit_role_listitem() {
        let t = render!({
            html! {
                <menu>
                    <li>{"Hello"}</li>
                </menu>
            }
        })
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_menu_has_implicit_role_menu() {
        let t = render!({
            html! {
                <menu>{"Hello"}</menu>
            }
        })
        .await;

        assert!(t.query_by_role("menu").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ul_has_implicit_role_list() {
        let t = render!({
            html! {
                <ul>{"Hello"}</ul>
            }
        })
        .await;

        assert!(t.query_by_role("list").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ol_has_implicit_role_list() {
        let t = render!({
            html! {
                <ol>{"Hello"}</ol>
            }
        })
        .await;

        assert!(t.query_by_role("list").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_math_has_implicit_role_math() {
        let t = render!({
            html! {
                <math>{"Hello"}</math>
            }
        })
        .await;

        assert!(t.query_by_role("math").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_meter_has_implicit_role_meter() {
        let t = render!({
            html! {
                <meter>{"Hello"}</meter>
            }
        })
        .await;

        assert!(t.query_by_role("meter").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_has_implicit_role_navigation() {
        let t = render!({
            html! {
                <nav>{"Hello"}</nav>
            }
        })
        .await;

        assert!(t.query_by_role("navigation").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_option_has_implicit_role_option() {
        let t = render!({
            html! {
                <select>
                    <option>{"Hello"}</option>
                </select>
            }
        })
        .await;

        assert!(t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_datalist_option_has_implicit_role_option() {
        let t = render!({
            html! {
                <datalist>
                    <option>{"Hello"}</option>
                </datalist>
            }
        })
        .await;

        assert!(t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_option_does_not_have_implicit_role_option() {
        let t = render!({
            html! {
                <option>{"Hello"}</option>
            }
        })
        .await;

        assert!(!t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_output_has_implicit_role_status() {
        let t = render!({
            html! {
                <output>{"Hello"}</output>
            }
        })
        .await;

        assert!(t.query_by_role("status").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_p_has_implicit_role_paragraph() {
        let t = render!({
            html! {
                <p>{"Hello"}</p>
            }
        })
        .await;

        assert!(t.query_by_role("paragraph").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_progress_has_implicit_role_progressbar() {
        let t = render!({
            html! {
                <progress>{"Hello"}</progress>
            }
        })
        .await;

        assert!(t.query_by_role("progressbar").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_search_has_implicit_role_search() {
        let t = render!({
            html! {
                <search>{"Hello"}</search>
            }
        })
        .await;

        assert!(t.query_by_role("search").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_aria_label_has_implicit_role_region() {
        let t = render!({
            html! {
                <section aria-label="Hello">{"Hello"}</section>
            }
        })
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_aria_labelledby_has_implicit_role_region() {
        let t = render!({
            html! {
                <>
                    <label id="label">{"Hello"}</label>
                    <section aria-labelledby="label">{"Hello"}</section>
                </>
            }
        })
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_title_has_implicit_role_region() {
        let t = render!({
            html! {
                <section title="Hello">{"Hello"}</section>
            }
        })
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_strong_has_implicit_role_strong() {
        let t = render!({
            html! {
                <strong>{"Hello"}</strong>
            }
        })
        .await;

        assert!(t.query_by_role("strong").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_sub_has_implicit_role_subscript() {
        let t = render!({
            html! {
                <sub>{"Hello"}</sub>
            }
        })
        .await;

        assert!(t.query_by_role("subscript").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_svg_has_implicit_role_graphics_document() {
        let t = render!({
            html! {
                <svg>{"Hello"}</svg>
            }
        })
        .await;

        assert!(t.query_by_role("graphics-document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_has_implicit_role_table() {
        let t = render!({
            html! {
                <table>{"Hello"}</table>
            }
        })
        .await;

        assert!(t.query_by_role("table").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tbody_has_implicit_role_rowgroup() {
        let t = render!({
            html! {
                <tbody>{"Hello"}</tbody>
            }
        })
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_thead_has_implicit_role_rowgroup() {
        let t = render!({
            html! {
                <thead>{"Hello"}</thead>
            }
        })
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tfoot_has_implicit_role_rowgroup() {
        let t = render!({
            html! {
                <tfoot>{"Hello"}</tfoot>
            }
        })
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_td_has_implicit_role_cell() {
        let t = render!({
            html! {
                <table>
                    <td>{"Hello"}</td>
                </table>
            }
        })
        .await;

        assert!(t.query_by_role("cell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_table_role_td_has_implicit_role_cell() {
        let t = render!({
            html! {
                <div role="table">
                    <td>{"Hello"}</td>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("cell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_th_has_implicit_role_cell_or_columnheader_or_rowheader() {
        let t = render!({
            html! {
                <table>
                    <th>{"Hello"}</th>
                </table>
            }
        })
        .await;

        assert!(t.query_by_role("cell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_table_role_th_has_implicit_role_cell_or_columnheader_or_rowheader(
    ) {
        let t = render!({
            html! {
                <div role="table">
                    <th>{"Hello"}</th>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("cell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_grid_td_has_implicit_role_gridcell() {
        let t = render!({
            html! {
                <div role="grid">
                    <td>{"Hello"}</td>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("gridcell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_treegrid_td_has_implicit_role_gridcell() {
        let t = render!({
            html! {
                <div role="treegrid">
                    <td>{"Hello"}</td>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("gridcell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_grid_th_has_implicit_role_gridcell_or_columnheader_or_rowheader() {
        let t = render!({
            html! {
                <div role="grid">
                    <th>{"Hello"}</th>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("gridcell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_treegrid_th_has_implicit_role_gridcell_or_columnheader_or_rowheader(
    ) {
        let t = render!({
            html! {
                <div role="treegrid">
                    <th>{"Hello"}</th>
                </div>
            }
        })
        .await;

        assert!(t.query_by_role("gridcell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_time_has_implicit_role_time() {
        let t = render!({
            html! {
                <time>{"Hello"}</time>
            }
        })
        .await;

        assert!(t.query_by_role("time").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tr_has_implicit_role_row() {
        let t = render!({
            html! {
                <tr>{"Hello"}</tr>
            }
        })
        .await;

        assert!(t.query_by_role("row").exists());
    }

    #[wasm_bindgen_test]
    async fn test_render_with_state() {
        let t = render!({
            let state = use_state(|| true);
            use_remember_value(state);

            html! {}
        })
        .await;

        assert!(*t.get_state::<UseStateHandle<bool>>());
    }

    #[wasm_bindgen_test]
    async fn test_render_with_effect() {
        let t = render!({
            let state = use_state(|| 0);

            {
                let state = state.clone();

                use_effect_with((), move |()| {
                    state.set(100);
                });
            }

            use_remember_value(state.clone());

            html! {}
        })
        .await;

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 100);
    }

    #[wasm_bindgen_test]
    async fn test_render_with_reducer() {
        #[derive(Clone, PartialEq)]
        struct Counter {
            count: i32,
        }

        enum CounterAction {
            Increment,
            Decrement,
        }

        impl Reducible for Counter {
            type Action = CounterAction;

            fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
                match action {
                    CounterAction::Increment => Self {
                        count: self.count + 1,
                    }
                    .into(),
                    CounterAction::Decrement => Self {
                        count: self.count - 1,
                    }
                    .into(),
                }
            }
        }

        let t = render!({
            let state = use_reducer(|| Counter { count: 0 });
            use_remember_value(state.clone());

            html! {}
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 0);

        t.act(|| {
            t.get_state::<UseReducerHandle<Counter>>()
                .dispatch(CounterAction::Increment);
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 1);

        t.act(|| {
            t.get_state::<UseReducerHandle<Counter>>()
                .dispatch(CounterAction::Decrement);
        })
        .await;

        assert_eq!(t.get_state::<UseReducerHandle<Counter>>().count, 0);
    }

    #[wasm_bindgen_test]
    async fn test_render() {
        let t = render!({
            let counter = use_state(|| 0);

            let increment = use_callback(counter.clone(), |_event: MouseEvent, counter| {
                counter.set(**counter + 1);
            });

            use_remember_value(counter.clone());

            html! {
                <button onclick={&increment}>{"Click me "}{*counter}</button>
            }
        })
        .await;

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 0);

        let button = t.query_by_role("button");
        assert!(button.exists());
        assert!(button.text().contains("Click me 0"));

        let button = button.click().await;
        assert!(button.text().contains("Click me 1"));

        assert_eq!(*t.get_state::<UseStateHandle<i32>>(), 1);
    }
}
