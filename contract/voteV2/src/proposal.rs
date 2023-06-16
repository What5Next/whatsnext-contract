use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize, Debug)]
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
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context()-> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .signer_account_id("alice_near".parse().unwrap());
        builder
    }

    #[test]
    fn test_default(){
        let mut context = get_context();
        testing_env!(context.build());

        let contract = Contract::default();
        testing_env!(context.is_view(true).build());

        assert_eq!(contract.get_currrent_candidate().0, 0);
    }

    #[test]
    fn test_add_proposal(){
        let mut context = get_context();
        testing_env!(context.build());

        let mut contract = Contract::default();
        contract.add_proposal(Proposal { 
            title: "test title".to_string(), 
            writer: AccountId::try_from("alice_near".to_string()).unwrap(), 
            keywords: "astronaut riding horse".to_string(), 
            description: "progressive scene".to_string(), 
        });
        
        testing_env!(context.is_view(true).build());
        let proposal = contract.proposals.get(& 1).unwrap();

        println!("New Proposal : {:?}", proposal);
    }
    
}