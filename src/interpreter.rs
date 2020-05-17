use crate::rep;
use crate::rep::Rep;
use itertools::Itertools;
use std::cell::Cell;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ptr;

#[macro_use]
mod cache;
pub mod event;
pub mod penalty;
pub mod placing;
pub mod raw;
mod subdivisions;
pub mod team;
pub mod tournament;
mod web_of_meh;

use self::event::Event;
use self::penalty::Penalty;
use self::placing::Placing;
use self::team::Team;
use self::tournament::Tournament;

#[derive(Debug)]
pub struct Interpreter {
    tournament: Box<Tournament>,
    events: Vec<Event>,
    teams: Vec<Team>,
    placings: Vec<Placing>,
    penalties: Vec<Penalty>,
    rep: Rep,
}

impl Interpreter {
    pub fn new(rep: Rep) -> Interpreter {
        let mut i = Self::create_models(rep);
        i.link_models();
        i.sort_events_naturally();
        i.sort_teams_by_rank();
        i
    }

    pub fn from_yaml(source: &str) -> Interpreter {
        Self::new(serde_yaml::from_str(&source).unwrap())
    }

    pub fn tournament(&self) -> &Tournament {
        self.tournament.as_ref()
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn teams(&self) -> &Vec<Team> {
        &self.teams
    }

    pub fn placings(&self) -> &Vec<Placing> {
        &self.placings
    }

    pub fn penalties(&self) -> &Vec<Penalty> {
        &self.penalties
    }

    pub fn subdivisions(&self) -> HashMap<&String, Interpreter> {
        match &self.rep.subdivisions {
            Some(subdivisions) => subdivisions
                .iter()
                .map(|s| (&s.name, Self::new(self.subdivision_rep(&s))))
                .collect::<HashMap<_, _>>(),
            None => HashMap::new(),
        }
    }

    fn sort_events_naturally(&mut self) {
        self.events.sort_unstable_by(|e1, e2| {
            e1.trial().cmp(&e2.trial()).then(e1.name().cmp(&e2.name()))
        });
    }

    fn sort_teams_by_rank(&mut self) {
        self.teams.sort_unstable_by(|t1, t2| {
            t1.disqualified()
                .cmp(&t2.disqualified())
                .then(t1.exhibition().cmp(&t2.exhibition()))
                .then(t1.points().cmp(&t2.points()))
                .then(t1.medal_counts().cmp(t2.medal_counts()).reverse())
                .then(t1.trial_event_points().cmp(&t2.trial_event_points()))
                .then(
                    t1.trial_event_medal_counts()
                        .cmp(t2.trial_event_medal_counts())
                        .reverse(),
                )
                .then(t1.number().cmp(&t2.number()))
        });
    }
}
