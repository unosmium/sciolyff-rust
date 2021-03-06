use super::raw::Raw;
use super::*;

#[derive(Debug)]
#[allow(clippy::option_option)]
pub struct Placing {
    pub(super) tournament: *const Tournament,
    pub(super) team: *const Team,
    pub(super) event: *const Event,
    pub(super) subdivision_placing: Option<*const Placing>,
    pub(super) raw: Option<Raw>,
    pub(super) rep: rep::Placing,
    pub(super) order: Cell<Option<usize>>,
    tie: Cell<Option<bool>>,
    place: Cell<Option<Option<usize>>>,
    points: Cell<Option<usize>>,
    isolated_points: Cell<Option<usize>>,
    dropped_as_part_of_worst_placings: Cell<Option<bool>>,
}

impl Placing {
    pub(super) fn new(rep: rep::Placing) -> Placing {
        Placing {
            tournament: ptr::null(),
            team: ptr::null(),
            event: ptr::null(),
            subdivision_placing: None,
            raw: None,
            rep,
            order: Cell::new(None), // assigned to in sort_placings_in_events
            tie: Cell::new(None),
            place: Cell::new(None),
            points: Cell::new(None),
            isolated_points: Cell::new(None),
            dropped_as_part_of_worst_placings: Cell::new(None),
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

    pub fn subdivision_placing(&self) -> Option<&Placing> {
        match self.subdivision_placing {
            Some(p) => unsafe { Some(&*p) },
            None => None,
        }
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

    pub fn order(&self) -> usize {
        self.order.get().unwrap()
    }

    pub fn medal(&self) -> Option<usize> {
        if self.isolated_points() <= self.tournament().medals() as usize {
            Some(self.isolated_points())
        } else {
            None
        }
    }

    pub fn tie(&self) -> bool {
        cache!(self.tie, {
            if self.raw().is_some() {
                self.event()
                    .raws()
                    .filter(|&r| r == self.raw().as_ref().unwrap())
                    .count()
                    > 1
            } else {
                self.rep.tie.is_some() && self.rep.tie.unwrap()
            }
        })
    }

    pub fn place(&self) -> Option<usize> {
        cache!(self.place, {
            if self.raw().is_some() {
                let place = self
                    .event()
                    .raws()
                    .position(|r| r == self.raw().as_ref().unwrap())
                    .unwrap()
                    + 1;
                Some(place)
            } else {
                self.rep.place
            }
        })
    }

    pub fn raw(&self) -> &Option<Raw> {
        &self.raw
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
        cache!(self.dropped_as_part_of_worst_placings, {
            self.team()
                .worst_placings_to_be_dropped()
                .any(|p| ptr::eq(self, p))
        })
    }

    pub fn points(&self) -> usize {
        cache!(self.points, {
            if self.considered_for_team_points() {
                self.isolated_points()
            } else {
                0
            }
        })
    }

    pub fn isolated_points(&self) -> usize {
        cache!(self.isolated_points, {
            let max_place = self.event().maximum_place();
            let n = (max_place as isize + self.tournament().n_offset() as isize)
                as usize;
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
