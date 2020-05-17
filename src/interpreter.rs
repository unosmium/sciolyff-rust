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
pub mod team;
pub mod tournament;

use crate::interpreter::event::Event;
use crate::interpreter::penalty::Penalty;
use crate::interpreter::placing::Placing;
use crate::interpreter::team::Team;
use crate::interpreter::tournament::Tournament;

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
    pub fn new(source: &str) -> Interpreter {
        let rep: Rep = serde_yaml::from_str(&source).unwrap();

        let mut i = Self::create_models(rep);
        i.link_models();
        i.sort_events_naturally();
        i.sort_teams_by_rank();
        i
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

    fn create_models(rep: Rep) -> Interpreter {
        let rep_clone = rep.clone();
        Interpreter {
            tournament: Box::new(Tournament::new(rep.tournament)),
            events: rep.events.into_iter().map(Event::new).collect(),
            teams: rep.teams.into_iter().map(Team::new).collect(),
            placings: rep.placings.into_iter().map(Placing::new).collect(),
            penalties: match rep.penalties {
                None => Vec::new(),
                Some(p) => p.into_iter().map(Penalty::new).collect(),
            },
            rep: rep_clone,
        }
    }

    fn link_models(&mut self) {
        let tournament = self.tournament.as_ref() as *const _;

        self.link_penalties_and_placings(tournament);
        self.link_teams(tournament);
        self.link_events(tournament);
        self.link_tournament();
    }

    fn link_penalties_and_placings(&mut self, tournament: *const Tournament) {
        let teams_by_number = self
            .teams
            .iter()
            .map(|t| (t.rep.number, t as *const _))
            .collect::<HashMap<_, _>>();

        let events_by_name = self
            .events
            .iter()
            .map(|e| (&e.rep.name, e as *const _))
            .collect::<HashMap<_, _>>();

        for p in self.penalties.iter_mut() {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
        }

        for p in self.placings.iter_mut() {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
            p.event = events_by_name[&p.rep.event];
        }
    }

    fn link_teams(&mut self, tournament: *const Tournament) {
        let mut penalties_by_team = HashMap::new();
        for p in &self.penalties {
            penalties_by_team
                .entry(p.team().number())
                .or_insert_with(Vec::new)
                .push(p as *const _)
        }

        let mut placings_by_team = HashMap::new();
        for p in &self.placings {
            placings_by_team
                .entry(p.team().number())
                .or_insert_with(Vec::new)
                .push(p as *const _)
        }

        for t in self.teams.iter_mut() {
            t.tournament = tournament;
            t.placings = placings_by_team.remove(&t.rep.number).unwrap();
            t.penalties =
                penalties_by_team.remove(&t.rep.number).unwrap_or_default();
        }
    }

    fn link_events(&mut self, tournament: *const Tournament) {
        let mut placings_by_event = HashMap::new();
        for p in &self.placings {
            placings_by_event
                .entry(p.event().name().to_string())
                .or_insert_with(Vec::new)
                .push(p as *const _)
        }

        for e in self.events.iter_mut() {
            e.tournament = tournament;
            e.placings = placings_by_event.remove(&e.rep.name).unwrap();
        }
    }

    fn link_tournament(&mut self) {
        let t = &mut self.tournament;
        t.events = self.events.iter().map(|e| e as *const _).collect();
        t.teams = self.teams.iter().map(|e| e as *const _).collect();
        t.placings = self.placings.iter().map(|e| e as *const _).collect();
        t.penalties = self.penalties.iter().map(|e| e as *const _).collect();
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
