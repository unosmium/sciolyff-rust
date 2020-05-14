use crate::interpreter::*;

#[derive(Debug)]
pub struct Penalty {
    pub(in crate::interpreter) tournament: *const Tournament,
    pub(in crate::interpreter) team: *const Team,
    pub(in crate::interpreter) rep: rep::Penalty,
}

impl Penalty {
    pub(in crate::interpreter) fn new(rep: rep::Penalty) -> Penalty {
        Penalty {
            tournament: ptr::null(),
            team: ptr::null(),
            rep,
        }
    }

    pub fn team(&self) -> &Team {
        unsafe { &*self.team }
    }

    pub fn points(&self) -> u8 {
        self.rep.points
    }
}
