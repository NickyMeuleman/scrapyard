#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // self.scores.last().map(|n| *n)
        // equivalent
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // self.scores.iter().max().map(|n| *n)
        // equivalent
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort_unstable();
        scores.into_iter().rev().take(3).collect()
    }
}
