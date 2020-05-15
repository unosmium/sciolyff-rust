use crate::interpreter::*;

#[derive(Debug)]
pub struct Placing {
    pub(super) tournament: *const Tournament,
    pub(super) team: *const Team,
    pub(super) event: *const Event,
    pub(super) rep: rep::Placing,
}

impl Placing {
    pub(super) fn new(rep: rep::Placing) -> Placing {
        Placing {
            tournament: ptr::null(),
            team: ptr::null(),
            event: ptr::null(),
            rep,
        }
    }

    pub fn event(&self) -> &Event {
        unsafe { &*self.event }
    }

    pub fn team(&self) -> &Team {
        unsafe { &*self.team }
    }

    pub fn participated(&self) -> bool {
        self.rep.participated.unwrap_or(true)
    }

    pub fn disqualified(&self) -> bool {
        self.rep.disqualified.unwrap_or(false)
    }

    pub fn exempt(&self) -> bool {
        self.rep.exempt.unwrap_or(false)
    }

    pub fn unknown(&self) -> bool {
        self.rep.unknown.unwrap_or(false)
    }

    pub fn tie(&self) -> bool {
        false
    }

    pub fn place(&self) -> Option<usize> {
        self.rep.place
    }

    pub fn points(&self) -> usize {
        0
    }

    pub fn points_limited_by_maximum_place(&self) -> bool {
        false
    }


}
