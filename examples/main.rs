#[macro_use] extern crate nickel;
#[warn(unreachable_code)]
extern crate mustache;
use std::str;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
use nickel_view::view::NickelView;

fn main() {
    let mut server = Nickel::new();
    #[allow(unreachable_code)]
    server.get("/", middleware! { |_req, res|
        let data: HashMap<&'static str, String> = HashMap::new();
        let mut hash: HashMap<&'static str, String> =  HashMap::new();

        let form_element = NickelView.element("templates/welcome/_form.mustache", &data);

        hash.insert("form", form_element);
        hash.insert("title", "sample".to_string());
        let content = NickelView.element("templates/welcome/index.mustache", &hash);

        let mut data = HashMap::new();
        data.insert("content", &*content);
        data.insert("title", "sample");
        data.insert("summary", "this is sample html");
        data.insert("copyright_year", "2017");
        return res.render("templates/layouts/default.mustache", &data);
    });

    let _ = server.listen("127.0.0.1:6767");
}