use crate::rep;
use crate::rep::Rep;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct Interpreter {
    tournament: Rc<RefCell<Tournament>>,
    events: Vec<Rc<RefCell<Event>>>,
    teams: Vec<Rc<RefCell<Team>>>,
    placings: Vec<Rc<RefCell<Placing>>>,
    penalties: Vec<Rc<RefCell<Penalty>>>,
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
        let r = rep.clone();
        Interpreter {
            tournament: Rc::new(RefCell::new(Tournament { rep: r.tournament })),
            // todo: remove duplication using generics?
            events: r
                .events
                .into_iter()
                .map(|rep| {
                    Rc::new(RefCell::new(Event {
                        tournament: None,
                        placings: Vec::new(),
                        rep,
                    }))
                })
                .collect(),
            teams: r
                .teams
                .into_iter()
                .map(|rep| {
                    Rc::new(RefCell::new(Team {
                        tournament: None,
                        placings: Vec::new(),
                        penalties: Vec::new(),
                        rep,
                    }))
                })
                .collect(),
            placings: r
                .placings
                .into_iter()
                .map(|rep| {
                    Rc::new(RefCell::new(Placing {
                        tournament: None,
                        team: None,
                        event: None,
                        rep,
                    }))
                })
                .collect(),
            penalties: match r.penalties {
                None => Vec::new(),
                Some(penalties) => penalties
                    .into_iter()
                    .map(|rep| {
                        Rc::new(RefCell::new(Penalty {
                            tournament: None,
                            team: None,
                            rep,
                        }))
                    })
                    .collect(),
            },
            rep,
        }
    }

    fn link_models(&mut self) {
        self.penalties.iter().for_each(|p| {
            p.borrow_mut().tournament = Some(Rc::downgrade(&Rc::clone(&self.tournament)));
            p.borrow_mut().team = Some(Rc::downgrade(&Rc::clone(
                self.teams
                    .iter()
                    .find(|t| t.borrow().rep.number == p.borrow().rep.team)
                    .unwrap(),
            )))
        });

        self.placings.iter().for_each(|p| {
            p.borrow_mut().tournament = Some(Rc::downgrade(&Rc::clone(&self.tournament)));
            p.borrow_mut().event = Some(Rc::downgrade(&Rc::clone(
                self.events
                    .iter()
                    .find(|e| e.borrow().rep.name == p.borrow().rep.event)
                    .unwrap(),
            )));
            p.borrow_mut().team = Some(Rc::downgrade(&Rc::clone(
                self.teams
                    .iter()
                    .find(|t| t.borrow().rep.number == p.borrow().rep.team)
                    .unwrap(),
            )))
        });

        self.teams.iter().for_each(|t| {
            t.borrow_mut().tournament = Some(Rc::downgrade(&Rc::clone(&self.tournament)));
            t.borrow_mut().placings = self
                .placings
                .iter()
                .filter(|p| Rc::ptr_eq(&p.borrow().team.as_ref().unwrap().upgrade().unwrap(), t))
                .map(|p| Rc::downgrade(&Rc::clone(&p)))
                .collect();
            t.borrow_mut().penalties = self
                .penalties
                .iter()
                .filter(|p| Rc::ptr_eq(&p.borrow().team.as_ref().unwrap().upgrade().unwrap(), t))
                .map(|p| Rc::downgrade(&Rc::clone(&p)))
                .collect();
        });

        self.events.iter().for_each(|e| {
            e.borrow_mut().tournament = Some(Rc::downgrade(&Rc::clone(&self.tournament)));
            e.borrow_mut().placings = self
                .placings
                .iter()
                .filter(|p| Rc::ptr_eq(&p.borrow().event.as_ref().unwrap().upgrade().unwrap(), e))
                .map(|p| Rc::downgrade(&Rc::clone(&p)))
                .collect();
        });
    }
}

#[derive(Debug)]
struct Tournament {
    rep: rep::Tournament,
}

#[derive(Debug)]
struct Subdivision {
    tournament: Option<Weak<RefCell<Tournament>>>,
    rep: rep::Subdivision,
}

#[derive(Debug)]
struct Event {
    tournament: Option<Weak<RefCell<Tournament>>>,
    placings: Vec<Weak<RefCell<Placing>>>,
    rep: rep::Event,
}

#[derive(Debug)]
struct Team {
    tournament: Option<Weak<RefCell<Tournament>>>,
    placings: Vec<Weak<RefCell<Placing>>>,
    penalties: Vec<Weak<RefCell<Penalty>>>,
    rep: rep::Team,
}

#[derive(Debug)]
struct Placing {
    tournament: Option<Weak<RefCell<Tournament>>>,
    team: Option<Weak<RefCell<Team>>>,
    event: Option<Weak<RefCell<Event>>>,
    rep: rep::Placing,
}

#[derive(Debug)]
struct Penalty {
    tournament: Option<Weak<RefCell<Tournament>>>,
    team: Option<Weak<RefCell<Team>>>,
    rep: rep::Penalty,
}
