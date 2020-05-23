use lazy_static::lazy_static;
use serde::Serialize;
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
        let rep = Rep {
            tournament: self.tournament_info(),
        };
        let context = Context::from_serialize(rep).unwrap();
        TEMPLATES.render("template.html", &context).unwrap()
    }

    fn tournament_info(&self) -> Tournament {
        let t = &self.tournament;
        Tournament {
            title: format!("{} {}", t.year(), t.name()),
            short_title: format!("{} {}", t.year(), t.short_name()),
            date: t.date().format("%A, %B %-d, %Y"),
            location: t.location().to_string(),
            division: format!("(Div. {})", t.division()),
        }
    }
}

#[derive(Serialize)]
struct Rep {
    tournament: Tournament,
}

#[derive(Serialize)]
struct Tournament {
    short_title: String,
    title: String,
    date: String,
    location: String,
    division: String,
}
