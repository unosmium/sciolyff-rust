use crate::interpreter::*;

#[derive(Debug)]
pub struct Placing {
    pub(in crate::interpreter) tournament: *const Tournament,
    pub(in crate::interpreter) team: *const Team,
    pub(in crate::interpreter) event: *const Event,
    pub(in crate::interpreter) rep: rep::Placing,
}

impl Placing {
    pub(in crate::interpreter) fn new(rep: rep::Placing) -> Placing {
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
}
