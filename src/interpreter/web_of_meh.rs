use super::*;

impl super::Interpreter {
    pub(super) fn create_models(rep: Rep) -> Interpreter {
        let rep_clone = rep.clone();
        Interpreter {
            tournament: Box::new(Tournament::new(rep.tournament)),
            events: rep.events.into_iter().map(Event::new).collect(),
            teams: rep.teams.into_iter().map(Team::new).collect(),
            placings: rep.placings.into_iter().map(Placing::new).collect(),
            penalties: match rep.penalties {
                None => Vec::new(),
                Some(p) => p.into_iter().map(Penalty::new).collect(),
            },
            subdivisions: HashMap::new(),
            rep: rep_clone,
        }
    }

    pub(super) fn link_models(&mut self) {
        let tournament = self.tournament.as_ref() as *const _;

        // must be done in this exact order because reasons
        self.link_penalties_and_placings(tournament);
        self.sort_models();
        self.link_teams(tournament);
        self.link_events(tournament);
        self.link_tournament();
        self.sort_placings_in_events();
    }

    pub(super) fn link_models_to_subdivision_models(&mut self) {
        let mut placings_by_team_and_event = HashMap::new();
        let mut teams_by_number = HashMap::new();
        for (_, sub_i) in self.subdivisions.iter() {
            for p in sub_i.placings.iter() {
                placings_by_team_and_event
                    .insert((p.rep.team, &p.rep.event), p as *const Placing);
            }
            for t in sub_i.teams.iter() {
                teams_by_number.insert(t.number(), t as *const Team);
            }
        }

        for p in self.placings.iter_mut() {
            p.subdivision_placing =
                placings_by_team_and_event.remove(&(p.rep.team, &p.rep.event));
        }
        for t in self.teams.iter_mut() {
            t.subdivision_team = teams_by_number.remove(&t.number());
        }
        self.tournament.subdivisions = !self.subdivisions.is_empty();
    }

    fn link_penalties_and_placings(&mut self, tournament: *const Tournament) {
        let teams_by_number = self
            .teams
            .iter()
            .map(|t| (t.rep.number, t as *const _))
            .collect::<HashMap<_, _>>();

        let events_by_name = self
            .events
            .iter()
            .map(|e| (&e.rep.name, e as *const _))
            .collect::<HashMap<_, _>>();

        for p in self.penalties.iter_mut() {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
        }

        for p in self.placings.iter_mut() {
            p.tournament = tournament;
            p.team = teams_by_number[&p.rep.team];
            p.event = events_by_name[&p.rep.event];
            p.raw = match p.rep.raw.clone() {
                Some(raw) => Some(Raw {
                    low_score_wins: p.event().low_score_wins(),
                    rep: raw,
                }),
                None => None,
            };
        }
    }

    fn sort_models(&mut self) {
        self.events.sort();
        // teams will be sorted by rank later
        self.placings.sort_by(|p1, p2| {
            p1.event()
                .cmp(&p2.event())
                .then(p1.team().number().cmp(&p2.team().number()))
        });
        self.penalties.sort_by_key(|p| p.team().number());
    }

    fn link_teams(&mut self, tournament: *const Tournament) {
        let mut penalties_by_team = HashMap::new();
        for p in &self.penalties {
            penalties_by_team
                .entry(p.team().number())
                .or_insert_with(Vec::new)
                .push(p as *const _)
        }

        let mut placings_by_team = HashMap::new();
        for p in &self.placings {
            placings_by_team
                .entry(p.team().number())
                .or_insert_with(Vec::new)
                .push(p as *const _)
        }

        for t in self.teams.iter_mut() {
            t.tournament = tournament;
            t.placings = placings_by_team.remove(&t.rep.number).unwrap();
            t.penalties =
                penalties_by_team.remove(&t.rep.number).unwrap_or_default();
        }
    }

    fn link_events(&mut self, tournament: *const Tournament) {
        let mut placings_by_event = Vec::new();
        for (_, placings) in &self
            .placings
            .iter()
            .group_by(|p| (p.event().trial(), p.event().name()))
        {
            placings_by_event.push(placings.map(|p| p as *const _).collect());
        }

        for e in self.events.iter_mut() {
            e.tournament = tournament;
            e.placings = placings_by_event.remove(0);
            e.raws = e
                .placings()
                .filter(|p| p.raw().is_some())
                .map(|p| p.raw().as_ref().unwrap() as *const _)
                .collect();
            e.raws.sort()
        }
    }

    fn link_tournament(&mut self) {
        let t = &mut self.tournament;
        t.events = self.events.iter().map(|e| e as *const _).collect();
        t.teams = self.teams.iter().map(|e| e as *const _).collect();
        t.placings = self.placings.iter().map(|e| e as *const _).collect();
        t.penalties = self.penalties.iter().map(|e| e as *const _).collect();
    }

    fn sort_placings_in_events(&mut self) {
        for e in self.events.iter_mut() {
            e.placings.sort_by_key(|p| unsafe {
                (
                    (&**p).place().is_none(),
                    (&**p).place(),
                    !(&**p).unknown(),
                    (&**p).isolated_points(),
                    (&**p).team().number(),
                )
            })
        }
    }
}
