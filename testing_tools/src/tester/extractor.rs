pub trait Extractor {
    fn attribute(&self, name: &str) -> Option<String>;
    fn text(&self) -> String;
}
