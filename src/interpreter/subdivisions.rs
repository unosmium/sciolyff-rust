use super::*;

#[allow(dead_code)]
#[allow(unused_variables)]

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

    fn fix_placings_for_existing_teams(rep: &mut Rep) {}
    fn fix_event_placings(rep: &mut Rep) {}
    fn fix_placing_ties(rep: &mut Rep) {}
}
