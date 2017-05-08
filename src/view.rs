extern crate nickel;
extern crate mustache;
use std::str;
use std::collections::HashMap;
pub struct NickelView;
impl NickelView {
    /// Render mustache
    ///
    /// # Examples
    ///
    /// ```
    /// let data: HashMap<&'static str, String> = HashMap::new();
    /// let form_element = NickelView.element("templates/welcome/_form.mustache", &data);
    /// ```
    pub fn element(&mut self, path: &str, data: &HashMap<&'static str, String>) -> String {
        let mut bytes = vec![];
        let template = mustache::compile_path(path).unwrap();
        let _ = template.render(&mut bytes, data);
        str::from_utf8(&bytes).unwrap().to_string()
    }
}