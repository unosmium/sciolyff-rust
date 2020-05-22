use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        // (don't) look for templates in non-existent dir on the filesystem,
        // we will include them in the binary using include_str! macro
        let mut tera = Tera::new("asdf/*").unwrap();
        tera.add_raw_templates(vec![
            ("template.html", include_str!("html/template.html")),
            ("style.css",     include_str!("html/style.css")),
            ("script.js",     include_str!("html/script.js")),
        ]).unwrap();
        tera
    };
}

impl super::Interpreter {
    pub fn to_html(&self) -> String {
        TEMPLATES.render("template.html", &Context::new()).unwrap()
    }
}
