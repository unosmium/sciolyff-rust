use crate::rep;
use crate::rep::Rep;
use std::cmp;
use std::collections::HashMap;
use std::iter;
use std::ptr;

pub mod event;
pub mod penalty;
pub mod placing;
pub mod team;
pub mod tournament;

use crate::interpreter::event::Event;
use crate::interpreter::penalty::Penalty;
use crate::interpreter::placing::Placing;
use crate::interpreter::team::Team;
use crate::interpreter::tournament::Tournament;

#[derive(Debug)]
pub struct Interpreter {
    pub tournament: Tournament,
    pub events: Vec<Event>,
    pub teams: Vec<Team>,
    pub placings: Vec<Placing>,
    pub penalties: Vec<Penalty>,
    rep: Rep,
}

impl Interpreter {
    pub fn new(source: &str) -> Interpreter {
        let rep: Rep = serde_yaml::from_str(&source).unwrap();

        let mut i = Self::create_models(rep);
        i.link_models();
        i
    }

    fn create_models(rep: Rep) -> Interpreter {
        let rep_clone = rep.clone();
        Interpreter {
            tournament: Tournament::new(rep.tournament),
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
        let tournament = &self.tournament as *const Tournament;

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
}
