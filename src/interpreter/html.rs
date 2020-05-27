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
            subdivisions: self.subdivisions_info(),
            events: self.events_info(),
            teams: self.teams_info(),
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
            subdivisions: t.subdivisions(),
        }
    }

    fn subdivisions_info(&self) -> Vec<Subdivision> {
        let mut subs = self
            .subdivisions()
            .iter()
            .map(|(name, _)| Subdivision {
                name: name.to_string(),
            })
            .collect::<Vec<_>>();
        subs.sort_by(|s1, s2| s1.name.cmp(&s2.name));
        subs
    }

    fn events_info(&self) -> Vec<Event> {
        self.events()
            .iter()
            .map(|e| Event {
                name: e.name().to_string(),
            })
            .collect()
    }

    fn teams_info(&self) -> Vec<Team> {
        self.teams()
            .iter()
            .map(|t| Team {
                number: t.number(),
                name: t.name(),
                short_name: t.short_name(),
                location: t.location(),
                rank: t.rank(),
                points: t.points(),
                placings: t
                    .placings()
                    .map(|p| Placing {
                        isolated_points: p.isolated_points(),
                    })
                    .collect(),
                penalties: t.penalties().map(|p| p.points()).sum::<u8>(),
            })
            .collect()
    }
}

#[derive(Serialize)]
struct Rep {
    tournament: Tournament,
    subdivisions: Vec<Subdivision>,
    events: Vec<Event>,
    teams: Vec<Team>,
}

#[derive(Serialize)]
struct Tournament {
    short_title: String,
    title: String,
    date: String,
    location: String,
    division: String,
    subdivisions: bool,
}

#[derive(Serialize)]
struct Subdivision {
    name: String,
}

#[derive(Serialize)]
struct Event {
    name: String,
}

#[derive(Serialize)]
struct Team {
    number: usize,
    name: String,
    short_name: String,
    location: String,
    rank: usize,
    points: usize,
    placings: Vec<Placing>,
    penalties: u8,
}

#[derive(Serialize)]
struct Placing {
    isolated_points: usize,
}
