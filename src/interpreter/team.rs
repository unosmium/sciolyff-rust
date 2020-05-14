use crate::interpreter::*;

#[derive(Debug)]
pub struct Team {
    pub(in crate::interpreter) tournament: *const Tournament,
    pub(in crate::interpreter) placings: Vec<*const Placing>,
    pub(in crate::interpreter) penalties: Vec<*const Penalty>,
    pub(in crate::interpreter) rep: rep::Team,
}

impl Team {
    pub(in crate::interpreter) fn new(rep: rep::Team) -> Team {
        Team {
            tournament: ptr::null(),
            placings: Vec::new(),
            penalties: Vec::new(),
            rep,
        }
    }
}
