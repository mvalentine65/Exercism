#[derive(Debug)]
pub struct HighScores<'a> {
    pub score_list: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            score_list: &scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.score_list
    }

    pub fn latest(&self) -> Option<u32> {
        self.score_list.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.score_list.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp: Vec<u32> = self.score_list.to_vec();
        temp.sort_unstable();
        temp.reverse();
        temp.into_iter().take(3).collect::<Vec<u32>>()
    }
}
