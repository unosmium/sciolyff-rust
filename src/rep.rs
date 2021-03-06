use serde::Deserialize;
use serde::Serialize;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub name: Option<String>,
    #[serde(rename = "short name")]
    pub short_name: Option<String>,
    pub location: String,
    pub level: String,
    pub state: Option<String>,
    pub division: String,
    pub year: usize,
    pub date: String,
    #[serde(rename = "worst placings dropped")]
    pub worst_placings_dropped: Option<u8>,
    #[serde(rename = "exempt placings")]
    pub exempt_placings: Option<u8>,
    #[serde(rename = "maximum place")]
    pub maximum_place: Option<usize>,
    #[serde(rename = "per-event n")]
    pub per_event_n: Option<String>,
    #[serde(rename = "n offset")]
    pub n_offset: Option<i8>,
    pub trophies: Option<u8>,
    pub medals: Option<u8>,
    pub bids: Option<u8>,
    #[serde(rename = "bids per school")]
    pub bids_per_school: Option<u8>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subdivision {
    pub name: String,
    #[serde(rename = "maximum place")]
    pub maximum_place: Option<usize>,
    pub trophies: Option<u8>,
    pub medals: Option<u8>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub trial: Option<bool>,
    pub trialed: Option<bool>,
    pub scoring: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub school: String,
    #[serde(rename = "school abbreviation")]
    pub school_abbreviation: Option<String>,
    pub suffix: Option<String>,
    pub subdivision: Option<String>,
    pub exhibition: Option<bool>,
    pub disqualified: Option<bool>,
    pub number: usize,
    pub city: Option<String>,
    pub state: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placing {
    pub event: String,
    pub team: usize,
    pub participated: Option<bool>,
    pub disqualified: Option<bool>,
    pub exempt: Option<bool>,
    pub unknown: Option<bool>,
    pub tie: Option<bool>,
    pub place: Option<usize>,
    pub raw: Option<Raw>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raw {
    pub score: f64,
    pub tier: Option<u8>,
    #[serde(rename = "tiebreaker rank")]
    pub tiebreaker_rank: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub team: usize,
    pub points: u8,
}
