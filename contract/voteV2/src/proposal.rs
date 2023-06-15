use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub title: String,
    pub writer : AccountId,
    pub keywords: String,
    pub description: String,
}

pub trait ProposalData {
    fn proposal( &self, proposal_id:U64 ) -> Proposal;
    fn all_proposal(&self) -> Vec<(U64, Proposal)>;
    fn add_proposal(&mut self, proposal: Proposal) -> bool;
    fn remove_proposal(&mut self, proposal_id: U64) -> bool;
}

#[near_bindgen]
impl ProposalData for Contract{
    fn proposal(&self, proposal_id: U64) -> Proposal {
        self.proposals.get(&proposal_id.0).unwrap()
    }

    fn all_proposal(&self) -> Vec<(U64, Proposal)> {
        self.proposals
            .iter()
            .map(
                |proposal| {(U64(proposal.0), proposal.1)}
            )
            .collect()
    }

    fn add_proposal(&mut self, proposal: Proposal) -> bool {
        let current_len = self.proposals.len();
        let proposal_id = current_len + 1;
        self.proposals.insert(&(proposal_id), &proposal);

        true
    }

    fn remove_proposal(&mut self, proposal_id: U64) -> bool {
        let total_votes_for_proposal = self.proposal_total_votes.get(&proposal_id.0).unwrap_or(0);

        // Already voted proposal cannot be removed.
        require!(total_votes_for_proposal == 0, "Proposal: Already Voted on the proposal.");

        let proposal_wrapped = self.proposals.get(&proposal_id.0);
        // Checking that if the signer is the owner;
        if let Some(proposal) = proposal_wrapped {
            require!(
                proposal.writer.eq(&env::predecessor_account_id()), 
                "Proposal: The signer is same to writer to remove proposal."
            )
        } else {
            panic!("Proposal: Didn't find proposal with this ID");
        }

        self.proposals.remove(&proposal_id.0);

        true
    }
}

#[cfg(test)]
mod proposal_tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;

    fn init()-> VMContextBuilder {
        
        todo!();
    }
    
}