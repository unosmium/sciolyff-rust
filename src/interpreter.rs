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
            events: rep.events.into_iter().map(|rep| Event::new(rep)).collect(),
            teams: rep.teams.into_iter().map(|rep| Team::new(rep)).collect(),
            placings: rep
                .placings
                .into_iter()
                .map(|rep| Placing::new(rep))
                .collect(),
            penalties: match rep.penalties {
                None => Vec::new(),
                Some(p) => p.into_iter().map(|rep| Penalty::new(rep)).collect(),
            },
            rep: rep_clone,
        }
    }

    fn link_models(&mut self) {
        let tournament = self.tournament.as_ref() as *const Tournament;

        let mut teams_by_number = HashMap::new();
        for team in &self.teams {
            teams_by_number.insert(team.rep.number, team as *const Team);
        }

        let mut events_by_name = HashMap::new();
        for event in &self.events {
            events_by_name.insert(&event.rep.name, event as *const Event);
        }

        self.penalties.iter_mut().for_each(|p| {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
        });

        self.placings.iter_mut().for_each(|p| {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
            p.event = events_by_name[&p.rep.event];
        });

        let mut placings_by_team = HashMap::new();
        for team in &self.teams {
            let mut placings_of_team = Vec::new();
            for placing in &self.placings {
                if placing.rep.team == team.rep.number {
                    placings_of_team.push(placing as *const Placing);
                }
            }
            placings_by_team.insert(team.rep.number, placings_of_team);
        }

        let mut placings_by_event = HashMap::new();
        for event in &self.events {
            let mut placings_of_event = Vec::new();
            for placing in &self.placings {
                if placing.rep.event == event.rep.name {
                    placings_of_event.push(placing as *const Placing);
                }
            }
            placings_by_event.insert(event.rep.name.clone(), placings_of_event);
        }

        self.teams.iter_mut().for_each(|t| {
            t.tournament = tournament;
            t.placings = placings_by_team.remove(&t.rep.number).unwrap();
        });

        // TODO: Link teams to their penalties

        self.events.iter_mut().for_each(|e| {
            e.tournament = tournament;
            e.placings = placings_by_event.remove(&e.rep.name).unwrap();
        });

        self.tournament.events =
            self.events.iter().map(|e| e as *const Event).collect();
        self.tournament.teams =
            self.teams.iter().map(|e| e as *const Team).collect();
        self.tournament.placings =
            self.placings.iter().map(|e| e as *const Placing).collect();
        self.tournament.penalties =
            self.penalties.iter().map(|e| e as *const Penalty).collect();
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
