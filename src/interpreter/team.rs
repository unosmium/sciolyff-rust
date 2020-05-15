use crate::interpreter::*;

#[derive(Debug)]
pub struct Team {
    pub(super) tournament: *const Tournament,
    pub(super) placings: Vec<*const Placing>,
    pub(super) penalties: Vec<*const Penalty>,
    pub(super) rep: rep::Team,
}

impl Team {
    pub(super) fn new(rep: rep::Team) -> Team {
        Team {
            tournament: ptr::null(),
            placings: Vec::new(),
            penalties: Vec::new(),
            rep,
        }
    }

    pub fn school(&self) -> &str {
        &self.rep.school
    }

    pub fn school_abbreviation(&self) -> Option<&str> {
        self.rep.school_abbreviation.as_deref()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.rep.suffix.as_deref()
    }

    pub fn subdivision(&self) -> Option<&str> {
        self.rep.subdivision.as_deref()
    }

    pub fn exhibition(&self) -> bool {
        self.rep.exhibition.unwrap_or(false)
    }

    pub fn disqualified(&self) -> bool {
        self.rep.disqualified.unwrap_or(false)
    }

    pub fn number(&self) -> usize {
        self.rep.number
    }

    pub fn city(&self) -> Option<&str> {
        self.rep.city.as_deref()
    }

    pub fn state(&self) -> &str {
        &self.rep.state
    }
}
