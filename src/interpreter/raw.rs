use super::*;

#[derive(Debug)]
pub struct Raw {
    pub(super) low_score_wins: bool,
    pub(super) rep: rep::Raw,
}

impl Raw {
    pub fn score(&self) -> f64 {
        self.rep.score
    }

    pub fn tiered(&self) -> bool {
        self.tier() > 1
    }

    pub fn tier(&self) -> u8 {
        self.rep.tier.unwrap_or(1)
    }

    pub fn lost_tiebreaker(&self) -> bool {
        self.tiebreaker_rank() > 1
    }

    pub fn tiebreaker_rank(&self) -> u8 {
        self.rep.tiebreaker_rank.unwrap_or(1)
    }
}

impl Ord for Raw {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tier()
            .cmp(&other.tier())
            .then_with(|| {
                let cmp = self.score().partial_cmp(&other.score()).unwrap();
                if self.low_score_wins {
                    cmp
                } else {
                    cmp.reverse()
                }
            })
            .then(self.tiebreaker_rank().cmp(&other.tiebreaker_rank()))
    }
}

impl PartialOrd for Raw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Raw {}

impl PartialEq for Raw {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
            && self.tier() == other.tier()
            && self.tiebreaker_rank() == other.tiebreaker_rank()
    }
}
