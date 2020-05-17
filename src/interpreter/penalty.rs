use super::*;

#[derive(Debug)]
pub struct Penalty {
    pub(super) tournament: *const Tournament,
    pub(super) team: *const Team,
    pub(super) rep: rep::Penalty,
}

impl Penalty {
    pub(super) fn new(rep: rep::Penalty) -> Penalty {
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
