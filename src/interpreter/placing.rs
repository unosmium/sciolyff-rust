use super::raw::Raw;
use super::*;

#[derive(Debug)]
pub struct Placing {
    pub(super) tournament: *const Tournament,
    pub(super) team: *const Team,
    pub(super) event: *const Event,
    pub(super) rep: rep::Placing,
    points: Cell<Option<usize>>,
    isolated_points: Cell<Option<usize>>,
}

impl Placing {
    pub(super) fn new(rep: rep::Placing) -> Placing {
        Placing {
            tournament: ptr::null(),
            team: ptr::null(),
            event: ptr::null(),
            rep,
            points: Cell::new(None),
            isolated_points: Cell::new(None),
        }
    }

    pub fn tournament(&self) -> &Tournament {
        unsafe { &*self.tournament }
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

    pub fn tie(&self) -> bool {
        false
    }

    pub fn place(&self) -> Option<usize> {
        if self.raw().is_some() {
            Some(0)
        } else {
            self.rep.place
        }
    }

    pub fn raw(&self) -> Option<Raw> {
        match self.rep.raw.clone() {
            Some(raw) => Some(Raw {
                low_score_wins: self.event().low_score_wins(),
                rep: raw,
            }),
            None => None,
        }
    }

    pub fn did_not_participate(&self) -> bool {
        !self.participated()
    }

    pub fn participation_only(&self) -> bool {
        self.participated()
            && self.place().is_none()
            && !self.disqualified()
            && !self.unknown()
    }

    pub fn dropped_as_part_of_worst_placings(&self) -> bool {
        self.team()
            .worst_placings_to_be_dropped()
            .any(|p| ptr::eq(self, p))
    }

    pub fn points(&self) -> usize {
        cache!(self, points, {
            if self.considered_for_team_points() {
                self.isolated_points()
            } else {
                0
            }
        })
    }

    pub fn isolated_points(&self) -> usize {
        cache!(self, isolated_points, {
            let max_place = self.event().maximum_place();
            let n = max_place + self.tournament().n_offset() as usize;
            if self.disqualified() {
                n + 2
            } else if self.did_not_participate() {
                n + 1
            } else if self.participation_only() || self.unknown() {
                n
            } else {
                cmp::min(self.calculate_points(), max_place)
            }
        })
    }

    pub fn considered_for_team_points(&self) -> bool {
        self.initially_considered_for_team_points()
            && !self.dropped_as_part_of_worst_placings()
    }

    pub fn initially_considered_for_team_points(&self) -> bool {
        !(self.event().trial() || self.event().trialed() || self.exempt())
    }

    pub fn points_affected_by_exhibition(&self) -> bool {
        self.considered_for_team_points()
            && self.place().is_some()
            && self.exhibition_placings_behind() != 0
    }

    pub fn points_limited_by_maximum_place(&self) -> bool {
        self.tournament().custom_maximum_place()
            && (self.unknown()
                || (self.place().is_some()
                    && (self.calculate_points()
                        > self.event().maximum_place()
                        || self.calculate_points()
                            == self.event().maximum_place()
                            && self.tie())))
    }

    fn calculate_points(&self) -> usize {
        if self.event().trial() {
            self.place().unwrap()
        } else {
            self.place().unwrap() - self.exhibition_placings_behind()
        }
    }

    fn exhibition_placings_behind(&self) -> usize {
        self.event()
            .placings()
            .filter(|p| {
                (p.exempt() || p.team().exhibition())
                    && p.place().is_some()
                    && p.place().unwrap() < self.place().unwrap()
            })
            .count()
    }
}
