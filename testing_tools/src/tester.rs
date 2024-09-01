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
    fn build_role_query(&self, role: &str) -> String {
        match role {
            "link" => "a[href], area[href]",
            "generic" => {
                r#"
                a:not([href]), area:not([href]), b, bdi, bdo, body, data, div,
                article footer, aside footer, main footer, nav footer, section footer,
                [role='article'] footer, [role='complementary'] footer, [role='main'] footer,
                [role='navigation'] footer, [role='region'] footer, article header,
                aside header, main header, nav header, section header, [role='article'] header,
                [role='complementary'] header, [role='main'] header, [role='navigation'] header,
                [role='region'] header, i, li:not(ul li):not(ol li):not(menu li), pre, q, samp,
                section:not([aria-label]):not([aria-labelledby]):not([title]), small, span, u   
            "#
            }
            "group" => "address, details, fieldset, hgroup, optgroup",
            "article" => "article",
            "complementary" => "aside",
            "blockquote" => "blockquote",
            "button" => "button, input[type='button'], input[type='image'], input[type='reset'], input[type='submit'], summary",
            "caption" => "caption",
            "code" => "code",
            "listbox" => "datalist, select[multiple], select[size]:not([size='1'])",
            "deletion" => "del, s",
            "term" => "dfn",
            "dialog" => "dialog",
            "emphasis" => "em",
            "figure" => "figure",
            "contentinfo" => "footer:not(article footer):not(aside footer):not(main footer):not(nav footer):not(section footer):not([role='article'] footer):not([role='complementary'] footer):not([role='main'] footer):not([role='navigation'] footer):not([role='region'] footer)",
            "form" => "form",
            "heading" => "h1, h2, h3, h4, h5, h6",
            "banner" => "header:not(article header):not(aside header):not(main header):not(nav header):not(section header):not([role='article'] header):not([role='complementary'] header):not([role='main'] header):not([role='navigation'] header):not([role='region'] header)",
            "separator" => "hr",
            "document" => "html",
            "checkbox" => "input[type='checkbox']",
            "textbox" => r#"
                input:not([list]):not([type]), input[type='email']:not([list]), input[type='text']:not([list]),
                input[type='tel']:not([list]), input[type='url']:not([list]), textarea
            "#,
            "combobox" => r#"
                input[list]:not([type]), input[list][type='text'], input[list][type='search'],
                input[list][type='tel'], input[list][type='url'], input[list][type='email'],
                select:not([multiple]):not([size]), select[size='1']:not([multiple])
            "#,
            "spinbutton" => "input[type='number']",
            "radio" => "input[type='radio']",
            "slider" => "input[type='range']",
            "searchbox" => "input[type='search']:not([list])",
            "insertion" => "ins",
            "listitem" => "ul li, ol li, menu li",
            "menu" => "menu",
            "math" => "math",
            "list" => "ol, ul",
            "meter" => "meter",
            "navigation" => "nav",
            "option" => "select option, datalist option",
            "status" => "output",
            "paragraph" => "p",
            "progressbar" => "progress",
            "search" => "search",
            "region" => "section[aria-label], section[aria-labelledby], section[title]",
            "strong" => "strong",
            "subscript" => "sub",
            "graphics-document" => "svg",
            "table" => "table",
            "rowgroup" => "tbody, thead, tfoot",
            "cell" => "table td, [role='table'] td, table th, [role='table'] th",
            "gridcell" => "[role='grid'] td, [role='treegrid'] td, [role='grid'] th, [role='treegrid'] th",
            "columnheader" => "table th, [role='table'] th, [role='grid'] th, [role='treegrid'] th", 
            "rowheader" => "table th, [role='table'] th, [role='grid'] th, [role='treegrid'] th", 
            "time" => "time",
            "row" => "tr",
            _ => "",
        }
        .to_string()
    }

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
        let implicit_selector = self.build_role_query(role);
        let explicit_selector = format!("[role='{}']", role);

        self.query_by_selector(&format!("{implicit_selector}, {explicit_selector}"))
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

#[cfg(test)]
mod tests {
    use crate::{render, Event, Extractor, Query};
    use wasm_bindgen_test::*;
    use web_sys::wasm_bindgen::JsCast;
    use yew::prelude::*;

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

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_link() {
        let t = render! {
            <span role="link">{"Hello"}</span>
        }
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_a_with_href_has_implicit_role_link() {
        let t = render! {
            <a href="http://example.com">{"Hello"}</a>
        }
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_area_with_href_has_implicit_role_link() {
        let t = render! {
            <map name="map">
                <area href="http://example.com" shape="rect" coords="34,44,270,350" />
            </map>
        }
        .await;

        assert!(t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_a_without_href_does_not_have_implicit_role_link() {
        let t = render! {
            <a>{"Hello"}</a>
        }
        .await;

        assert!(!t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_area_without_href_does_not_have_implicit_role_link() {
        let t = render! {
            <map name="map">
                <area shape="rect" coords="34,44,270,350" />
            </map>
        }
        .await;

        assert!(!t.query_by_role("link").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_generic() {
        let t = render! {
            <span role="generic">{"Hello"}</span>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_b_has_implicit_role_generic() {
        let t = render! {
            <b>{"Hello"}</b>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_bdi_has_implicit_role_generic() {
        let t = render! {
            <bdi>{"Hello"}</bdi>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_bdo_has_implicit_role_generic() {
        let t = render! {
            <bdo>{"Hello"}</bdo>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_body_has_implicit_role_generic() {
        let t = render! {
            <body>{"Hello"}</body>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_data_has_implicit_role_generic() {
        let t = render! {
            <data>{"Hello"}</data>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_div_has_implicit_role_generic() {
        let t = render! {
            <div>{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_article_footer_has_implicit_role_generic() {
        let t = render! {
            <article>
                <footer>{"Hello"}</footer>
            </article>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_aside_footer_has_implicit_role_generic() {
        let t = render! {
            <aside>
                <footer>{"Hello"}</footer>
            </aside>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_main_footer_has_implicit_role_generic() {
        let t = render! {
            <main>
                <footer>{"Hello"}</footer>
            </main>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_footer_has_implicit_role_generic() {
        let t = render! {
            <nav>
                <footer>{"Hello"}</footer>
            </nav>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_footer_has_implicit_role_generic() {
        let t = render! {
            <section>
                <footer>{"Hello"}</footer>
            </section>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_article_footer_has_implicit_role_generic() {
        let t = render! {
            <div role="article">
                <footer>{"Hello"}</footer>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_complementary_footer_has_implicit_role_generic() {
        let t = render! {
            <div role="complementary">
                <footer>{"Hello"}</footer>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_main_footer_has_implicit_role_generic() {
        let t = render! {
            <div role="main">
                <footer>{"Hello"}</footer>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_navigation_footer_has_implicit_role_generic() {
        let t = render! {
            <div role="navigation">
                <footer>{"Hello"}</footer>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_region_footer_has_implicit_role_generic() {
        let t = render! {
            <div role="region">
                <footer>{"Hello"}</footer>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_article_header_has_implicit_role_generic() {
        let t = render! {
            <article>
                <header>{"Hello"}</header>
            </article>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_aside_header_has_implicit_role_generic() {
        let t = render! {
            <aside>
                <header>{"Hello"}</header>
            </aside>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_main_header_has_implicit_role_generic() {
        let t = render! {
            <main>
                <header>{"Hello"}</header>
            </main>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_header_has_implicit_role_generic() {
        let t = render! {
            <nav>
                <header>{"Hello"}</header>
            </nav>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_header_has_implicit_role_generic() {
        let t = render! {
            <section>
                <header>{"Hello"}</header>
            </section>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_article_header_has_implicit_role_generic() {
        let t = render! {
            <div role="article">
                <header>{"Hello"}</header>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_complementary_header_has_implicit_role_generic() {
        let t = render! {
            <div role="complementary">
                <header>{"Hello"}</header>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_main_header_has_implicit_role_generic() {
        let t = render! {
            <div role="main">
                <header>{"Hello"}</header>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_navigation_header_has_implicit_role_generic() {
        let t = render! {
            <div role="navigation">
                <header>{"Hello"}</header>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_region_header_has_implicit_role_generic() {
        let t = render! {
            <div role="region">
                <header>{"Hello"}</header>
            </div>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_i_has_implicit_role_generic() {
        let t = render! {
            <i>{"Hello"}</i>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_li_has_implicit_role_generic() {
        let t = render! {
            <li>{"Hello"}</li>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_pre_has_implicit_role_generic() {
        let t = render! {
            <pre>{"Hello"}</pre>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_q_has_implicit_role_generic() {
        let t = render! {
            <q>{"Hello"}</q>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_samp_has_implicit_role_generic() {
        let t = render! {
            <samp>{"Hello"}</samp>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_has_implicit_role_generic() {
        let t = render! {
            <section>{"Hello"}</section>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_does_not_have_implicit_role_generic() {
        let t = render! {
            <section aria-label="Hello">{"Hello"}</section>
        }
        .await;

        assert!(!t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_small_has_implicit_role_generic() {
        let t = render! {
            <small>{"Hello"}</small>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_span_has_implicit_role_generic() {
        let t = render! {
            <span>{"Hello"}</span>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_u_has_implicit_role_generic() {
        let t = render! {
            <u>{"Hello"}</u>
        }
        .await;

        assert!(t.query_by_role("generic").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_group() {
        let t = render! {
            <span role="group">{"Hello"}</span>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_address_has_implicit_role_group() {
        let t = render! {
            <address>{"Hello"}</address>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_details_has_implicit_role_group() {
        let t = render! {
            <details>{"Hello"}</details>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_fieldset_has_implicit_role_group() {
        let t = render! {
            <fieldset>{"Hello"}</fieldset>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_hgroup_has_implicit_role_group() {
        let t = render! {
            <hgroup>{"Hello"}</hgroup>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_optgroup_has_implicit_role_group() {
        let t = render! {
            <optgroup>{"Hello"}</optgroup>
        }
        .await;

        assert!(t.query_by_role("group").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_article() {
        let t = render! {
            <div role="article">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("article").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_article() {
        let t = render! {
            <article>{"Hello"}</article>
        }
        .await;

        assert!(t.query_by_role("article").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_complementary() {
        let t = render! {
            <div role="complementary">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("complementary").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_aside() {
        let t = render! {
            <aside>{"Hello"}</aside>
        }
        .await;

        assert!(t.query_by_role("complementary").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_blockquote() {
        let t = render! {
            <div role="blockquote">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("blockquote").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_blockquote() {
        let t = render! {
            <blockquote>{"Hello"}</blockquote>
        }
        .await;

        assert!(t.query_by_role("blockquote").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_button() {
        let t = render! {
            <div role="button">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_button_has_implicit_role_button() {
        let t = render! {
            <button>{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_button_has_implicit_role_button() {
        let t = render! {
            <input type="button" value="Hello" />
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_image_has_implicit_role_button() {
        let t = render! {
            <input type="image" src="http://example.com/image.png" />
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_reset_has_implicit_role_button() {
        let t = render! {
            <input type="reset" value="Hello" />
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_submit_has_implicit_role_button() {
        let t = render! {
            <input type="submit" value="Hello" />
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_summary_has_implicit_role_button() {
        let t = render! {
            <summary>{"Hello"}</summary>
        }
        .await;

        assert!(t.query_by_role("button").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_caption() {
        let t = render! {
            <div role="caption">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("caption").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_caption() {
        let t = render! {
            <caption>{"Hello"}</caption>
        }
        .await;

        assert!(t.query_by_role("caption").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_code() {
        let t = render! {
            <div role="code">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("code").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_code() {
        let t = render! {
            <code>{"Hello"}</code>
        }
        .await;

        assert!(t.query_by_role("code").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_listbox() {
        let t = render! {
            <div role="listbox">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_datalist_has_implicit_role_datalist() {
        let t = render! {
            <datalist>{"Hello"}</datalist>
        }
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_multiple_has_implicit_role_listbox() {
        let t = render! {
            <select multiple=true>{"Hello"}</select>
        }
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_greater_than_2_has_implicit_role_listbox() {
        let t = render! {
            <select size="2">
                <option>{"Hello"}</option>
                <option>{"World"}</option>
            </select>
        }
        .await;

        assert!(t.query_by_role("listbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_deletion() {
        let t = render! {
            <div role="deletion">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_del_with_implicit_role_deletion() {
        let t = render! {
            <del>{"Hello"}</del>
        }
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_s_with_implicit_role_deletion() {
        let t = render! {
            <s>{"Hello"}</s>
        }
        .await;

        assert!(t.query_by_role("deletion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_term() {
        let t = render! {
            <div role="term">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("term").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_dfn() {
        let t = render! {
            <dfn>{"Hello"}</dfn>
        }
        .await;

        assert!(t.query_by_role("term").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_dialog() {
        let t = render! {
            <div role="dialog">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("dialog").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_dialog() {
        let t = render! {
            <dialog>{"Hello"}</dialog>
        }
        .await;

        assert!(t.query_by_role("dialog").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_emphasis() {
        let t = render! {
            <div role="emphasis">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("emphasis").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_em() {
        let t = render! {
            <em>{"Hello"}</em>
        }
        .await;

        assert!(t.query_by_role("emphasis").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_figure() {
        let t = render! {
            <div role="figure">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("figure").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_implicit_role_figure() {
        let t = render! {
            <figure>{"Hello"}</figure>
        }
        .await;

        assert!(t.query_by_role("figure").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_contentinfo() {
        let t = render! {
            <div role="contentinfo">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("contentinfo").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_footer_has_implicit_role_contentinfo() {
        let t = render! {
            <footer>{"Hello"}</footer>
        }
        .await;

        assert!(t.query_by_role("contentinfo").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_form() {
        let t = render! {
            <div role="form">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("form").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_form_has_implicit_role_form() {
        let t = render! {
            <form>{"Hello"}</form>
        }
        .await;

        assert!(t.query_by_role("form").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_heading() {
        let t = render! {
            <div role="heading">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h1_has_implicit_role_heading() {
        let t = render! {
            <h1>{"Hello"}</h1>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h2_has_implicit_role_heading() {
        let t = render! {
            <h2>{"Hello"}</h2>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h3_has_implicit_role_heading() {
        let t = render! {
            <h3>{"Hello"}</h3>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h4_has_implicit_role_heading() {
        let t = render! {
            <h4>{"Hello"}</h4>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h5_has_implicit_role_heading() {
        let t = render! {
            <h5>{"Hello"}</h5>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_h6_has_implicit_role_heading() {
        let t = render! {
            <h6>{"Hello"}</h6>
        }
        .await;

        assert!(t.query_by_role("heading").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_header_has_implicit_role_banner() {
        let t = render! {
            <header>{"Hello"}</header>
        }
        .await;

        assert!(t.query_by_role("banner").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_separator() {
        let t = render! {
            <div role="separator">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("separator").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_hr_has_implicit_role_separator() {
        let t = render! {
            <hr />
        }
        .await;

        assert!(t.query_by_role("separator").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_document() {
        let t = render! {
            <div role="document">{"Hello"}</div>
        }
        .await;

        assert!(t.query_by_role("document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_html_has_implicit_role_document() {
        let t = render! {
            <html>{"Hello"}</html>
        }
        .await;

        assert!(t.query_by_role("document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_checkbox() {
        let t = render! {
            <button role="checkbox">{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("checkbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_checkbox_has_implicit_role_checkbox() {
        let t = render! {
            <input type="checkbox" />
        }
        .await;

        assert!(t.query_by_role("checkbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_textbox() {
        let t = render! {
            <button role="textbox">{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_with_no_type_has_implicit_role_textbox() {
        let t = render! {
            <input />
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_text_has_implicit_role_textbox() {
        let t = render! {
            <input type="text" />
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_has_implicit_role_textbox() {
        let t = render! {
            <input type="email" />
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_with_list_does_not_have_implicit_role_textbox() {
        let t = render! {
            <>
                <input type="email" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_has_implicit_role_textbox() {
        let t = render! {
            <input type="url" />
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_with_list_does_not_have_implicit_role_textbox() {
        let t = render! {
            <>
                <input type="url" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_has_implicit_role_textbox() {
        let t = render! {
            <input type="tel" />
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_with_list_does_not_have_implicit_role_textbox() {
        let t = render! {
            <>
                <input type="tel" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_textarea_has_implicit_role_textbox() {
        let t = render! {
            <textarea>{"Hello"}</textarea>
        }
        .await;

        assert!(t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_spinbutton() {
        let t = render! {
            <button role="spinbutton">{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("spinbutton").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_number_has_implicit_role_spinbutton() {
        let t = render! {
            <input type="number" />
        }
        .await;

        assert!(t.query_by_role("spinbutton").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_radio() {
        let t = render! {
            <button role="radio">{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("radio").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_radio_has_implicit_role_radio() {
        let t = render! {
            <input type="radio" />
        }
        .await;

        assert!(t.query_by_role("radio").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_range_has_implicit_role_slider() {
        let t = render! {
            <input type="range" />
        }
        .await;

        assert!(t.query_by_role("slider").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_has_implicit_role_searchbox() {
        let t = render! {
            <input type="search" />
        }
        .await;

        assert!(t.query_by_role("searchbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_with_list_does_not_have_role_searchbox() {
        let t = render! {
             <>
                <input type="search" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(!t.query_by_role("searchbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_element_with_explicit_role_combobox() {
        let t = render! {
            <button role="combobox">{"Hello"}</button>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_text_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input type="text" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_email_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input type="email" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_tel_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input type="tel" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_url_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input type="url" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_search_with_list_has_implicit_role_combobox() {
        let t = render! {
            <>
                <input type="search" list="datalist" />

                <datalist id="datalist">
                    <option value="Hello" />
                </datalist>
            </>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_has_implicit_role_combobox() {
        let t = render! {
            <select>
                <option value="Hello">{"Hello"}</option>
            </select>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_1_has_implicit_role_combobox() {
        let t = render! {
            <select size="1">
                <option value="Hello">{"Hello"}</option>
            </select>
        }
        .await;

        assert!(t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_multiple_does_not_have_implicit_role_combobox() {
        let t = render! {
            <select multiple=true>
                <option value="Hello">{"Hello"}</option>
            </select>
        }
        .await;

        assert!(!t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_with_size_greater_than_1_does_not_have_implicit_role_combobox(
    ) {
        let t = render! {
            <select size="2">
                <option value="Hello">{"Hello"}</option>
                <option value="Hello">{"Hello"}</option>
            </select>
        }
        .await;

        assert!(!t.query_by_role("combobox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_time_does_not_have_implicit_role() {
        let t = render! {
            <input type="time" />
        }
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_input_type_week_does_not_have_implicit_role() {
        let t = render! {
            <input type="week" />
        }
        .await;

        assert!(!t.query_by_role("textbox").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ins_has_implicit_role_insertion() {
        let t = render! {
            <ins>{"Hello"}</ins>
        }
        .await;

        assert!(t.query_by_role("insertion").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ul_li_has_implicit_role_listitem() {
        let t = render! {
            <ul>
                <li>{"Hello"}</li>
            </ul>
        }
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ol_li_has_implicit_role_listitem() {
        let t = render! {
            <ol>
                <li>{"Hello"}</li>
            </ol>
        }
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_menu_li_has_implicit_role_listitem() {
        let t = render! {
            <menu>
                <li>{"Hello"}</li>
            </menu>
        }
        .await;

        assert!(t.query_by_role("listitem").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_menu_has_implicit_role_menu() {
        let t = render! {
            <menu>{"Hello"}</menu>
        }
        .await;

        assert!(t.query_by_role("menu").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ul_has_implicit_role_list() {
        let t = render! {
            <ul>{"Hello"}</ul>
        }
        .await;

        assert!(t.query_by_role("list").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_ol_has_implicit_role_list() {
        let t = render! {
            <ol>{"Hello"}</ol>
        }
        .await;

        assert!(t.query_by_role("list").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_math_has_implicit_role_math() {
        let t = render! {
            <math>{"Hello"}</math>
        }
        .await;

        assert!(t.query_by_role("math").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_meter_has_implicit_role_meter() {
        let t = render! {
            <meter>{"Hello"}</meter>
        }
        .await;

        assert!(t.query_by_role("meter").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_nav_has_implicit_role_navigation() {
        let t = render! {
            <nav>{"Hello"}</nav>
        }
        .await;

        assert!(t.query_by_role("navigation").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_select_option_has_implicit_role_option() {
        let t = render! {
            <select>
                <option>{"Hello"}</option>
            </select>
        }
        .await;

        assert!(t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_datalist_option_has_implicit_role_option() {
        let t = render! {
            <datalist>
                <option>{"Hello"}</option>
            </datalist>
        }
        .await;

        assert!(t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_option_does_not_have_implicit_role_option() {
        let t = render! {
            <option>{"Hello"}</option>
        }
        .await;

        assert!(!t.query_by_role("option").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_output_has_implicit_role_status() {
        let t = render! {
            <output>{"Hello"}</output>
        }
        .await;

        assert!(t.query_by_role("status").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_p_has_implicit_role_paragraph() {
        let t = render! {
            <p>{"Hello"}</p>
        }
        .await;

        assert!(t.query_by_role("paragraph").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_progress_has_implicit_role_progressbar() {
        let t = render! {
            <progress>{"Hello"}</progress>
        }
        .await;

        assert!(t.query_by_role("progressbar").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_search_has_implicit_role_search() {
        let t = render! {
            <search>{"Hello"}</search>
        }
        .await;

        assert!(t.query_by_role("search").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_aria_label_has_implicit_role_region() {
        let t = render! {
            <section aria-label="Hello">{"Hello"}</section>
        }
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_aria_labelledby_has_implicit_role_region() {
        let t = render! {
            <>
                <label id="label">{"Hello"}</label>
                <section aria-labelledby="label">{"Hello"}</section>
            </>
        }
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_section_with_title_has_implicit_role_region() {
        let t = render! {
            <section title="Hello">{"Hello"}</section>
        }
        .await;

        assert!(t.query_by_role("region").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_strong_has_implicit_role_strong() {
        let t = render! {
            <strong>{"Hello"}</strong>
        }
        .await;

        assert!(t.query_by_role("strong").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_sub_has_implicit_role_subscript() {
        let t = render! {
            <sub>{"Hello"}</sub>
        }
        .await;

        assert!(t.query_by_role("subscript").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_svg_has_implicit_role_graphics_document() {
        let t = render! {
            <svg>{"Hello"}</svg>
        }
        .await;

        assert!(t.query_by_role("graphics-document").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_has_implicit_role_table() {
        let t = render! {
            <table>{"Hello"}</table>
        }
        .await;

        assert!(t.query_by_role("table").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tbody_has_implicit_role_rowgroup() {
        let t = render! {
            <tbody>{"Hello"}</tbody>
        }
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_thead_has_implicit_role_rowgroup() {
        let t = render! {
            <thead>{"Hello"}</thead>
        }
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tfoot_has_implicit_role_rowgroup() {
        let t = render! {
            <tfoot>{"Hello"}</tfoot>
        }
        .await;

        assert!(t.query_by_role("rowgroup").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_td_has_implicit_role_cell() {
        let t = render! {
            <table>
                <td>{"Hello"}</td>
            </table>
        }
        .await;

        assert!(t.query_by_role("cell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_table_role_td_has_implicit_role_cell() {
        let t = render! {
            <div role="table">
                <td>{"Hello"}</td>
            </div>
        }
        .await;

        assert!(t.query_by_role("cell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_table_th_has_implicit_role_cell_or_columnheader_or_rowheader() {
        let t = render! {
            <table>
                <th>{"Hello"}</th>
            </table>
        }
        .await;

        assert!(t.query_by_role("cell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_explicit_table_role_th_has_implicit_role_cell_or_columnheader_or_rowheader(
    ) {
        let t = render! {
            <div role="table">
                <th>{"Hello"}</th>
            </div>
        }
        .await;

        assert!(t.query_by_role("cell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_grid_td_has_implicit_role_gridcell() {
        let t = render! {
            <div role="grid">
                <td>{"Hello"}</td>
            </div>
        }
        .await;

        assert!(t.query_by_role("gridcell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_treegrid_td_has_implicit_role_gridcell() {
        let t = render! {
            <div role="treegrid">
                <td>{"Hello"}</td>
            </div>
        }
        .await;

        assert!(t.query_by_role("gridcell").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_grid_th_has_implicit_role_gridcell_or_columnheader_or_rowheader() {
        let t = render! {
            <div role="grid">
                <th>{"Hello"}</th>
            </div>
        }
        .await;

        assert!(t.query_by_role("gridcell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_treegrid_th_has_implicit_role_gridcell_or_columnheader_or_rowheader(
    ) {
        let t = render! {
            <div role="treegrid">
                <th>{"Hello"}</th>
            </div>
        }
        .await;

        assert!(t.query_by_role("gridcell").exists());
        assert!(t.query_by_role("columnheader").exists());
        assert!(t.query_by_role("rowheader").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_time_has_implicit_role_time() {
        let t = render! {
            <time>{"Hello"}</time>
        }
        .await;

        assert!(t.query_by_role("time").exists());
    }

    #[wasm_bindgen_test]
    async fn test_query_by_role_tr_has_implicit_role_row() {
        let t = render! {
            <tr>{"Hello"}</tr>
        }
        .await;

        assert!(t.query_by_role("row").exists());
    }
}
