pub trait Extractor {
    fn attribute(&self, name: &str) -> Option<String>;
    fn text(&self) -> String;
    #[deprecated(since = "1.2.1", note = "Please use `get_remembered_value` instead")]
    fn get_state<T: Clone + 'static>(&self) -> T;
    fn get_remembered_value<T: Clone + 'static>(&self) -> T;
}
