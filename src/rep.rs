use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rep {
    #[serde(rename = "Tournament")]
    pub tournament: Tournament,
    #[serde(rename = "Subdivisions")]
    pub subdivisions: Option<Vec<Subdivision>>,
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
    #[serde(rename = "Teams")]
    pub teams: Vec<Team>,
    #[serde(rename = "Placings")]
    pub placings: Vec<Placing>,
    #[serde(rename = "Penalties")]
    pub penalties: Option<Vec<Penalty>>,
}

#[derive(Debug, Deserialize)]
pub struct Tournament {
    pub name: Option<String>,
    #[serde(rename = "short name")]
    pub short_name: Option<String>,
    pub location: String,
    pub level: String,
    pub state: Option<String>,
    pub division: String,
    pub year: u16,
    pub date: String,
    #[serde(rename = "worst placings dropped")]
    pub worst_placings_dropped: Option<u8>,
    #[serde(rename = "exempt placings")]
    pub exempt_placings: Option<u8>,
    #[serde(rename = "maximum place")]
    pub maximum_place: Option<u8>,
    #[serde(rename = "per-event n")]
    pub per_event_n: Option<String>,
    #[serde(rename ="n offset")]
    pub n_offset: Option<String>,
    pub trophies: Option<u8>,
    pub medals: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Subdivision;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub trial: Option<bool>,
    pub trialed: Option<bool>,
    pub scoring: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub school: String,
    #[serde(rename = "school abbreviation")]
    pub school_abbreviation: Option<String>,
    pub suffix: Option<String>,
    pub subdivision: Option<String>,
    pub exhibition: Option<bool>,
    pub disqualified: Option<bool>,
    pub number: u16,
    pub city: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Placing {
    pub event: String,
    pub team: u16,
    pub participated: Option<bool>,
    pub disqualified: Option<bool>,
    pub exempt: Option<bool>,
    pub unknown: Option<bool>,
    pub tie: Option<bool>,
    pub place: Option<u8>,
    pub raw: Option<Raw>,
}

#[derive(Debug, Deserialize)]
pub struct Raw {
    pub score: f64,
    pub tier: u8,
    #[serde(rename = "tiebreaker rank")]
    pub tiebreaker_rank: u8,
}

#[derive(Debug, Deserialize)]
pub struct Penalty {
    pub team: u16,
    pub points: u8,
}
