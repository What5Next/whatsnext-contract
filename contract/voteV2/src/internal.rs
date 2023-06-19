use crate::*;


#[near_bindgen]
impl Contract {
    #[private]
    pub fn internal_vote(&mut self, voter: AccountId, proposal_id: u64, amount: Balance) -> bool {

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

#[cfg(test)]
mod VoteTest {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(accounts(1));
        builder
    }

    #[test]
    fn vote_test(){
        let mut context = get_context();
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_proposal(Proposal { 
            title: "test title".to_string(), 
            writer: accounts(1), 
            keywords: "astronaut riding horse".to_string(), 
            description: "NFDF".to_string() 
        });

        const PROPOSAL_ID : u64 = 1;
        const VOTE_AMOUNT : u128 = 100;

        testing_env!(context.is_view(true).build());
        let proposal = contract.get_proposal(PROPOSAL_ID.into());
        
        println!("Proposal in Voting Test: {:?}", proposal);

        testing_env!(context.is_view(false).build());
        contract.internal_vote(accounts(1), 1, 100);

        testing_env!(context.is_view(true).build());
        let proposal_votes = contract.get_proposal_votes(PROPOSAL_ID.into());

        require!(proposal_votes.0 == VOTE_AMOUNT, "Not working votes method");

    }
}