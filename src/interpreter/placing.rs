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
}
