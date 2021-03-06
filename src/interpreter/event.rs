use super::*;

#[derive(Debug)]
pub struct Event {
    pub(super) tournament: *const Tournament,
    pub(super) placings: Vec<*const Placing>,
    pub(super) raws: Vec<*const Raw>,
    pub(super) rep: rep::Event,
}

impl Event {
    pub(super) fn new(rep: rep::Event) -> Event {
        Event {
            tournament: ptr::null(),
            placings: Vec::new(),
            raws: Vec::new(),
            rep,
        }
    }

    pub fn tournament(&self) -> &Tournament {
        unsafe { &*self.tournament }
    }

    pub fn placings(&self) -> impl Iterator<Item = &Placing> {
        unsafe { self.placings.clone().into_iter().map(|p| &*p) }
    }

    pub fn raws(&self) -> impl Iterator<Item = &Raw> {
        unsafe { self.raws.clone().into_iter().map(|r| &*r) }
    }

    pub fn name(&self) -> &str {
        &self.rep.name
    }

    pub fn trial(&self) -> bool {
        self.rep.trial.unwrap_or(false)
    }

    pub fn trialed(&self) -> bool {
        self.rep.trialed.unwrap_or(false)
    }

    pub fn high_score_wins(&self) -> bool {
        !self.low_score_wins()
    }

    pub fn low_score_wins(&self) -> bool {
        match &self.rep.scoring {
            Some(scoring) => scoring == "low",
            None => false,
        }
    }

    pub fn placing_for(&self, team: &Team) -> &Placing {
        self.placings().find(|p| ptr::eq(p.team, team)).unwrap()
    }

    pub fn maximum_place(&self) -> usize {
        if self.trial() {
            self.placings().count()
        } else if self.tournament().per_event_n().is_some() {
            cmp::min(
                self.per_event_maximum_place(),
                self.tournament().maximum_place(),
            )
        } else {
            self.tournament().maximum_place()
        }
    }

    fn per_event_maximum_place(&self) -> usize {
        let per_event_n = self.tournament().per_event_n().unwrap_or("");
        if per_event_n == "participation" {
            self.competing_teams_count()
        } else {
            self.placings().filter(|p| p.place().is_some()).count() + 1
        }
    }

    fn competing_teams_count(&self) -> usize {
        if self.trial() {
            self.placings().filter(|p| p.participated()).count()
        } else {
            self.placings()
                .filter(|p| {
                    p.participated() && !(p.team().exhibition() || p.exempt())
                })
                .count()
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.trial()
            .cmp(&other.trial())
            .then(self.name().cmp(&other.name()))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
