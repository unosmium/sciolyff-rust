use crate::interpreter::*;

#[derive(Debug)]
pub struct Event {
    pub(super) tournament: *const Tournament,
    pub(super) placings: Vec<*const Placing>,
    pub(super) rep: rep::Event,
}

impl Event {
    pub(super) fn new(rep: rep::Event) -> Event {
        Event {
            tournament: ptr::null(),
            placings: Vec::new(),
            rep,
        }
    }

    pub fn placings(&self) -> impl Iterator<Item = &Placing> {
        unsafe { self.placings.clone().into_iter().map(|p| &*p) }
    }

    pub fn name(&self) -> &str {
        &self.rep.name
    }

    pub fn trial(&self) -> bool {
        self.rep.trial.unwrap_or(false)
    }

    pub fn trialed(&self) -> bool {
        self.rep.trialed.unwrap_or(false)
    }

    pub fn high_score_wins(&self) -> bool {
        !self.low_score_wins()
    }

    pub fn low_score_wins(&self) -> bool {
        match self.rep.scoring.as_ref() {
            Some(scoring) => scoring == &String::from("low"),
            None => false,
        }
    }
}
