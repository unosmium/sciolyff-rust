use super::*;

#[derive(Debug)]
pub struct Team {
    pub(super) tournament: *const Tournament,
    pub(super) placings: Vec<*const Placing>,
    pub(super) penalties: Vec<*const Penalty>,
    pub(super) subdivision_team: Option<*const Team>,
    pub(super) rep: rep::Team,
    pub(super) rank: Option<usize>,
    points: Cell<Option<usize>>,
    trial_event_points: Cell<Option<usize>>,
}

impl Team {
    pub(super) fn new(rep: rep::Team) -> Team {
        Team {
            tournament: ptr::null(),
            placings: Vec::new(),
            penalties: Vec::new(),
            subdivision_team: None,
            rep,
            rank: None,
            points: Cell::new(None),
            trial_event_points: Cell::new(None),
        }
    }

    pub fn tournament(&self) -> &Tournament {
        unsafe { &*self.tournament }
    }

    pub fn placings(&self) -> impl Iterator<Item = &Placing> {
        unsafe { self.placings.clone().into_iter().map(|p| &*p) }
    }

    pub fn penalties(&self) -> impl Iterator<Item = &Penalty> {
        unsafe { self.penalties.clone().into_iter().map(|p| &*p) }
    }

    pub fn subdivision_team(&self) -> Option<&Team> {
        match self.subdivision_team {
            Some(t) => unsafe { Some(&*t) },
            None => None,
        }
    }

    fn name_base(base: &str, suffix: Option<&str>) -> String {
        match suffix {
            Some(suffix) => format!("{} {}", base, suffix),
            None => base.to_string(),
        }
    }

    pub fn name(&self) -> String {
        Self::name_base(self.school(), self.suffix())
    }

    pub fn short_name(&self) -> String {
        match self.school_abbreviation() {
            Some(name) => Self::name_base(name, self.suffix()),
            None => self.name(),
        }
        .replace("Elementary School", "Elementary")
        .replace("Elementary/Middle School", "E.M.S.")
        .replace("Middle School", "M.S.")
        .replace("Junior High School", "J.H.S.")
        .replace("Middle High School", "M.H.S.")
        .replace("Middle/High School", "M.H.S.")
        .replace("Middle-High School", "M.H.S.")
        .replace("Junior/Senior High School", "Jr./Sr. H.S.")
        .replace("High School", "H.S.")
        .replace("Secondary School", "Secondary")
    }

    pub fn location(&self) -> String {
        match self.city() {
            Some(city) => format!("{}, {}", city, self.state()),
            None => self.state().to_string(),
        }
    }

    pub fn school(&self) -> &str {
        &self.rep.school
    }

    pub fn school_abbreviation(&self) -> Option<&str> {
        self.rep.school_abbreviation.as_deref()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.rep.suffix.as_deref()
    }

    pub fn subdivision(&self) -> Option<&str> {
        self.rep.subdivision.as_deref()
    }

    pub fn trophy(&self) -> Option<usize> {
        if self.rank() <= self.tournament().trophies() as usize {
            Some(self.rank())
        } else {
            None
        }
    }

    pub fn exhibition(&self) -> bool {
        self.rep.exhibition.unwrap_or(false)
    }

    pub fn disqualified(&self) -> bool {
        self.rep.disqualified.unwrap_or(false)
    }

    pub fn number(&self) -> usize {
        self.rep.number
    }

    pub fn city(&self) -> Option<&str> {
        self.rep.city.as_deref()
    }

    pub fn state(&self) -> &str {
        &self.rep.state
    }

    pub fn placing_for(&self, event: &Event) -> &Placing {
        self.placings().find(|p| ptr::eq(p.event, event)).unwrap()
    }

    pub fn rank(&self) -> usize {
        self.rank.unwrap()
    }

    pub fn points(&self) -> usize {
        cache!(self.points, {
            self.placings().map(|p| p.points()).sum::<usize>()
                + (self.penalties().map(|p| p.points()).sum::<u8>() as usize)
        })
    }

    pub fn earned_bid(&self) -> bool {
        let school_rank = self
            .tournament()
            .teams_eligible_for_bids()
            .position(|t| ptr::eq(t, self));
        school_rank.is_some()
            && school_rank.unwrap() < self.tournament().bids() as usize
    }

    pub fn worst_placings_to_be_dropped(
        &self,
    ) -> impl Iterator<Item = &Placing> {
        if self.tournament().worst_placings_dropped() == 0 {
            Vec::<&Placing>::new().into_iter().take(0)
        } else {
            let mut considered_placings = self
                .placings()
                .filter(|p| p.initially_considered_for_team_points())
                .collect::<Vec<&Placing>>();
            considered_placings.sort_by_key(|p| p.isolated_points());
            considered_placings.reverse();
            considered_placings
                .into_iter()
                .take(self.tournament().worst_placings_dropped() as usize)
        }
    }

    pub fn trial_event_points(&self) -> usize {
        cache!(self.trial_event_points, {
            self.placings()
                .filter(|p| p.event().trial())
                .map(|p| p.isolated_points())
                .sum()
        })
    }

    pub fn medal_counts(&self) -> impl Iterator<Item = usize> + '_ {
        (1..(self.tournament().teams().count() + 2)).map(move |medal_points| {
            self.placings()
                .filter(|p| p.considered_for_team_points())
                .filter(|p| p.points() == medal_points)
                .count()
        })
    }

    pub fn trial_event_medal_counts(&self) -> impl Iterator<Item = usize> + '_ {
        (1..(self.tournament().teams().count() + 2)).map(move |medal_points| {
            self.placings()
                .filter(|p| p.event().trial())
                .filter(|p| p.isolated_points() == medal_points)
                .count()
        })
    }
}
