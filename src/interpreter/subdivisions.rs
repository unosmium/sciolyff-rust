use super::*;

impl super::Interpreter {
    pub(super) fn subdivision_rep(&self, sub: &rep::Subdivision) -> Rep {
        let mut rep = self.rep.clone();

        Self::remove_teams_not_in_subdivision(&mut rep, sub);
        Self::fix_and_replace_subdivision_tournament_fields(&mut rep, sub);
        Self::limit_maximum_place(&mut rep);
        if !self.raws() {
            Self::fix_placings_for_existing_teams(&mut rep);
        }
        rep
    }

    fn remove_teams_not_in_subdivision(rep: &mut Rep, sub: &rep::Subdivision) {
        rep.teams.retain(|t| match &t.subdivision {
            Some(s) => s == &sub.name,
            None => false,
        });
        for t in &mut rep.teams {
            t.subdivision = None;
        }
        let mut team_numbers: Vec<_> =
            rep.teams.iter().map(|t| t.number).collect();
        team_numbers.sort();
        rep.placings
            .retain(|p| team_numbers.binary_search(&p.team).is_ok());
    }

    fn fix_and_replace_subdivision_tournament_fields(
        rep: &mut Rep,
        sub: &rep::Subdivision,
    ) {
        rep.tournament.medals = sub.medals;
        rep.tournament.trophies = sub.trophies;
        rep.tournament.maximum_place = sub.maximum_place;

        rep.tournament.bids = None;
        rep.tournament.bids_per_school = None;
        rep.subdivisions = None;
    }

    fn limit_maximum_place(rep: &mut Rep) {
        let max_place = rep.tournament.maximum_place;
        if max_place.is_none() {
            return;
        }

        let team_count = rep
            .teams
            .iter()
            .filter(|t| match t.exhibition {
                Some(e) => !e,
                None => true,
            })
            .count();

        if max_place.unwrap() > team_count {
            rep.tournament.maximum_place = None;
        }
    }

    fn fix_placings_for_existing_teams(rep: &mut Rep) {
        for event in rep.events.iter() {
            let mut event_placings = rep
                .placings
                .iter_mut()
                .filter(|p| p.event == event.name)
                .filter(|p| p.place.is_some())
                .collect::<Vec<_>>();

            event_placings.sort_by_key(|p| p.place.unwrap());

            let mut untied_place = 1;
            let mut tied_place = 1;
            let mut last_place_seen = 0;
            let mut iter = event_placings.into_iter().peekable();

            while iter.peek().is_some() {
                let p = iter.next().unwrap();

                if p.place.unwrap() == last_place_seen {
                    p.place = Some(tied_place);
                    p.tie = Some(true);
                } else {
                    last_place_seen = p.place.unwrap();
                    p.place = Some(untied_place);
                    tied_place = untied_place + 1;

                    if iter.peek().is_some()
                        && iter.peek().unwrap().place.unwrap()
                            == last_place_seen
                    {
                        p.tie = Some(true);
                    } else {
                        p.tie = None;
                    }
                }

                untied_place += 1;
            }
        }
    }
}
