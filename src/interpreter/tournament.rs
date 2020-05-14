use crate::interpreter::*;

#[derive(Debug)]
pub struct Tournament {
    pub(in crate::interpreter) rep: rep::Tournament,
}

impl Tournament {
    pub(in crate::interpreter) fn new(rep: rep::Tournament) -> Tournament {
        Tournament { rep }
    }

    pub fn name(&self) -> Option<&str> {
        self.rep.name.as_deref()
    }

    pub fn location(&self) -> &str {
        &self.rep.location
    }

    pub fn level(&self) -> &str {
        &self.rep.level
    }

    pub fn state(&self) -> Option<&str> {
        self.rep.state.as_deref()
    }

    pub fn division(&self) -> &str {
        &self.rep.division
    }

    pub fn year(&self) -> u16 {
        self.rep.year
    }

    pub fn short_name(&self) -> Option<&str> {
        self.rep.short_name.as_deref()
    }

    pub fn date(&self) -> &str {
        &self.rep.date
    }
}
