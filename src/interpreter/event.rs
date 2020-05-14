use crate::interpreter::*;

#[derive(Debug)]
pub struct Event {
    pub(in crate::interpreter) tournament: *const Tournament,
    pub(in crate::interpreter) placings: Vec<*const Placing>,
    pub(in crate::interpreter) rep: rep::Event,
}

impl Event {
    pub(in crate::interpreter) fn new(rep: rep::Event) -> Event {
        Event {
            tournament: ptr::null(),
            placings: Vec::new(),
            rep,
        }
    }
}
