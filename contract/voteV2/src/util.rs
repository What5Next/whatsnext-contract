use crate::*;

#[near_bindgen]
impl Contract {
    #[private]
    pub fn compare_votes(&self, proposal_a: u64, proposal_b: u64) -> u64 {
        let votes_proposal_a = self.proposal_total_votes.get(&proposal_a).unwrap_or(0);
        let votes_proposal_b = self.proposal_total_votes.get(&proposal_b).unwrap_or(0);

        if votes_proposal_a > votes_proposal_b {
            proposal_a
        } else {
            proposal_b
        }
    }
}