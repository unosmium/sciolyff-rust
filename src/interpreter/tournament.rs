use crate::interpreter::*;

#[derive(Debug)]
pub struct Tournament {
    pub(super) events: Vec<*const Event>,
    pub(super) teams: Vec<*const Team>,
    pub(super) placings: Vec<*const Placing>,
    pub(super) penalties: Vec<*const Penalty>,
    pub(super) rep: rep::Tournament,
}

impl Tournament {
    pub(super) fn new(rep: rep::Tournament) -> Tournament {
        Tournament {
            events: Vec::new(),
            teams: Vec::new(),
            placings: Vec::new(),
            penalties: Vec::new(),
            rep,
        }
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        unsafe { self.events.clone().into_iter().map(|e| &*e) }
    }

    pub fn teams(&self) -> impl Iterator<Item = &Team> {
        unsafe { self.teams.clone().into_iter().map(|t| &*t) }
    }

    pub fn placings(&self) -> impl Iterator<Item = &Placing> {
        unsafe { self.placings.clone().into_iter().map(|p| &*p) }
    }

    pub fn penalties(&self) -> impl Iterator<Item = &Penalty> {
        unsafe { self.penalties.clone().into_iter().map(|p| &*p) }
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

    pub fn year(&self) -> usize {
        self.rep.year
    }

    pub fn short_name(&self) -> Option<&str> {
        self.rep.short_name.as_deref()
    }

    pub fn date(&self) -> &str {
        &self.rep.date
    }

    pub fn medals(&self) -> u8 {
        self.rep
            .medals
            .unwrap_or(cmp::min(self.calc_medals(), self.maximum_place() as u8))
    }

    pub fn trophies(&self) -> u8 {
        self.rep.trophies.unwrap_or(cmp::min(
            self.calc_trophies(),
            self.nonexhibition_team_count() as u8,
        ))
    }

    pub fn bids(&self) -> u8 {
        self.rep.bids.unwrap_or(0)
    }

    pub fn bids_per_school(&self) -> u8 {
        self.rep.bids_per_school.unwrap_or(1)
    }

    pub fn worst_placings_dropped(&self) -> u8 {
        self.rep.worst_placings_dropped.unwrap_or(0)
    }

    pub fn exempt_placings(&self) -> u8 {
        self.rep.exempt_placings.unwrap_or(0)
    }

    pub fn custom_maximum_place(&self) -> bool {
        self.maximum_place() != self.nonexhibition_team_count()
    }

    pub fn maximum_place(&self) -> usize {
        self.rep
            .maximum_place
            .unwrap_or(self.nonexhibition_team_count())
    }

    pub fn per_event_n(&self) -> Option<&str> {
        self.rep.per_event_n.as_deref()
    }

    pub fn n_offset(&self) -> i8 {
        self.rep.n_offset.unwrap_or(0)
    }

    pub fn ties(&self) -> bool {
        self.placings().any(|p| p.tie())
    }

    pub fn ties_outside_of_maximum_places(&self) -> bool {
        self.placings()
            .any(|p| p.tie() && p.points_limited_by_maximum_place())
    }

    pub fn subdivisions(&self) -> bool {
        false
    }

    pub fn nonexhibition_team_count(&self) -> usize {
        self.teams().filter(|t| t.exhibition()).count()
    }

    pub fn top_teams_per_school(&self) -> impl Iterator<Item = &Team> {
        iter::empty()
    }

    pub fn teams_eligible_for_bids(&self) -> impl Iterator<Item = &Team> {
        iter::empty()
    }

    fn calc_medals(&self) -> u8 {
        cmp::max(
            (self.nonexhibition_team_count() as f32 / 10.).ceil() as u8,
            3,
        )
    }

    fn calc_trophies(&self) -> u8 {
        cmp::max(
            (self.nonexhibition_team_count() as f32 / 6.).ceil() as u8,
            3,
        )
    }
}
