pub trait Extractor {
    fn attribute(&self, name: &str) -> Option<String>;
    fn text(&self) -> String;
    fn get_state<T: Clone + 'static>(&self) -> T;
}
