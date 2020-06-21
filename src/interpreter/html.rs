use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        // (don't) look for templates in non-existent dir on the filesystem,
        // we will include them in the binary using include_str! macro
        let mut tera = Tera::new("asdf/*").unwrap();
        tera.add_raw_templates(vec![
            ("template.html",    include_str!("html/template.html")),
            ("style.css",        include_str!("html/style.css")),
            ("script.js",        include_str!("html/script.js")),
            ("chartist.min.css", include_str!("html/chartist.min.css")),
            ("chartist.min.js",  include_str!("html/chartist.min.js")),
            ("polyfills.min.js", include_str!("html/polyfills.min.js")),
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
            rep_yaml: serde_yaml::to_string(&self.rep).unwrap(),
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
            nonexhibition_team_count: t.nonexhibition_team_count(),
            subdivisions: t.subdivisions(),
            bids: t.bids() > 0,
            exempt_or_dropped_placings: t.exempt_placings() > 0
                || t.worst_placings_dropped() > 0,
            ties: t.ties_outside_of_maximum_places(),
            qualification_message: if t.bids() > 0 {
                let qualifiee = if t.bids_per_school() > 1 {
                    "team"
                } else {
                    "school"
                };
                let next = if t.level() == "Regionals" {
                    format!("{} State Tournament", t.state().unwrap())
                } else {
                    "National Tournament".to_string()
                };
                format!("Qualified {} for the {} {}", qualifiee, t.year(), next)
            } else {
                "".to_string()
            },
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
                trial: e.trial(),
                trialed: e.trialed(),
                participation_count: e
                    .placings()
                    .filter(|p| p.participated())
                    .count(),
                raws: e
                    .placings()
                    .filter(|p| p.raw().is_some())
                    .map(|p| {
                        let raw = p.raw().as_ref().unwrap();
                        Raw {
                            place: p.place().unwrap(),
                            score: raw.score(),
                            tier: raw.tier(),
                            tiebreaker_rank: raw.tiebreaker_rank(),
                        }
                    })
                    .collect(),
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
                school: t.school().to_string(),
                trophy: t.trophy(),
                exhibition: t.exhibition(),
                disqualified: t.disqualified(),
                state: t.state().to_string(),
                rank: t.rank(),
                points: t.points(),
                earned_bid: t.earned_bid(),
                placings: Self::placings_info(&t),
                penalties: t.penalties().map(|p| p.points()).sum::<u8>(),
                events_participated: t
                    .placings()
                    .filter(|p| p.participated())
                    .count(),
            })
            .collect()
    }

    fn placings_info(t: &super::team::Team) -> Vec<Placing> {
        t.placings()
            .map(|p| Placing {
                disqualified: p.disqualified(),
                exempt: p.exempt(),
                unknown: p.unknown(),
                order: p.order(),
                medal: p.medal(),
                tie: p.tie(),
                place: p.place(),
                did_not_participate: p.did_not_participate(),
                participation_only: p.participation_only(),
                dropped_as_part_of_worst_placings: p
                    .dropped_as_part_of_worst_placings(),
                points: p.points(),
                isolated_points: p.isolated_points(),
                considered_for_team_points: p.considered_for_team_points(),
                points_affected_by_exhibition: p
                    .points_affected_by_exhibition(),
                points_limited_by_maximum_place: p
                    .points_limited_by_maximum_place(),
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
    rep_yaml: String,
}

#[derive(Serialize)]
struct Tournament {
    short_title: String,
    title: String,
    date: String,
    location: String,
    division: String,
    nonexhibition_team_count: usize,
    subdivisions: bool,
    bids: bool,
    exempt_or_dropped_placings: bool,
    ties: bool,
    qualification_message: String,
}

#[derive(Serialize)]
struct Subdivision {
    name: String,
}

#[derive(Serialize)]
struct Event {
    name: String,
    trial: bool,
    trialed: bool,
    participation_count: usize,
    raws: Vec<Raw>,
}

#[derive(Serialize)]
struct Raw {
    place: usize,
    score: f64,
    tier: u8,
    tiebreaker_rank: u8,
}

#[derive(Serialize)]
struct Team {
    number: usize,
    name: String,
    short_name: String,
    location: String,
    school: String,
    trophy: Option<usize>,
    exhibition: bool,
    disqualified: bool,
    state: String,
    rank: usize,
    points: usize,
    earned_bid: bool,
    placings: Vec<Placing>,
    penalties: u8,
    events_participated: usize,
}

#[derive(Serialize)]
struct Placing {
    disqualified: bool,
    exempt: bool,
    unknown: bool,
    order: usize,
    medal: Option<usize>,
    tie: bool,
    place: Option<usize>,
    did_not_participate: bool,
    participation_only: bool,
    dropped_as_part_of_worst_placings: bool,
    points: usize,
    isolated_points: usize,
    considered_for_team_points: bool,
    points_affected_by_exhibition: bool,
    points_limited_by_maximum_place: bool,
}
