use crate::*;


#[near_bindgen]
impl Contract {
    #[private]
    pub fn _vote(&mut self, voter: AccountId, proposal_id: u64, amount: Balance) -> bool {

        // Adding Total Votes this contract
        self.total_votes += amount;
        
        // Adding Proposal's Total Votes
        let current_proposal_total_votes = self.proposal_total_votes.get(&proposal_id).unwrap_or(0);
        self.proposal_total_votes.insert(&proposal_id, &(amount + current_proposal_total_votes));

        // Adding Proposal Votes with Account ID
        let mut proposal_votes_with_account_wrapped = self.proposal_votes_with_accountId.get(&voter)
            .unwrap_or_else(|| {UnorderedMap::new(StorageKey::ProposalVotes(proposal_id))});

        let current_proposal_votes_with_account = proposal_votes_with_account_wrapped.get(&proposal_id).unwrap_or(0);

        proposal_votes_with_account_wrapped.insert(&proposal_id, &( current_proposal_votes_with_account + amount ));
        self.proposal_votes_with_accountId.insert(&voter, &proposal_votes_with_account_wrapped);

        true
    }
}