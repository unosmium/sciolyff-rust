use crate::interpreter::*;

#[derive(Debug)]
pub struct Tournament {
    pub(in crate::interpreter) rep: rep::Tournament,
}

impl Tournament {
    pub(in crate::interpreter) fn new(rep: rep::Tournament) -> Tournament {
        Tournament { rep }
    }
}
