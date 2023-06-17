use near_sdk::{near_bindgen, require, env, AccountId, Balance, BorshStorageKey};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U64, U128};

pub mod proposal;
pub mod internal;
pub mod external;
pub mod util;

use crate::proposal::*;
use crate::internal::*;
use crate::util::*;
use crate::external::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    proposals : UnorderedMap<u64, Proposal>,
    proposal_total_votes: UnorderedMap<u64, Balance>,
    proposal_votes_with_accountId: LookupMap< AccountId, UnorderedMap<u64, Balance> >,
    total_votes : Balance,
    proposal_selected: u64,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Proposals,
    ProposalTotalVotes,
    ProposalVotesAccount,
    ProposalVotes(u64)
}

impl Default for Contract{
    fn default() -> Self{
        Self { 
            proposals: UnorderedMap::new(StorageKey::Proposals), 
            proposal_total_votes: UnorderedMap::new(StorageKey::ProposalTotalVotes), 
            proposal_votes_with_accountId: LookupMap::new(StorageKey::ProposalVotesAccount), 
            total_votes: 0,
            proposal_selected: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn vote(&mut self, proposal_id: U64, amount: U128) {
        let voter = env::predecessor_account_id();

        // Checking Platform Token with voter
        // todo

        // Changing Votes
        if let Some(_proposal) = self.proposals.get(&proposal_id.0){
            let result = self._vote(voter.clone(), proposal_id.0.clone(), amount.0.clone());
            require!(result, "Faild to Votes");
        } else {
            assert!(true, "No persist proposal");
        }

        // Changing Candidate
        let current_candidate = self.proposal_selected;
        if current_candidate == 0 {
            self.proposal_selected = proposal_id.0;
        } else {
            let new_candidate =  self.compare_votes(current_candidate.clone(), proposal_id.0.clone());
            self.proposal_selected = new_candidate;
        }
    }

    pub fn get_currrent_candidate(&self) -> U64 {
        self.proposal_selected.into()
    }

    pub fn get_proposal_votes(&self, proposal_id: U64) -> U128 {
        self.proposal_total_votes.get(&proposal_id.0).unwrap_or(0).into()
    }

    pub fn get_proposal_votes_with_account(
        &self, 
        account_id: AccountId, 
        proposal_id: U64
    ) -> U128 {
        let all_votes_with_account = self.proposal_votes_with_accountId.get(&account_id)
            .unwrap_or_else(|| panic!("Not exists account id in Data"));
        all_votes_with_account.get(&proposal_id.0).unwrap_or(0).into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(accounts(1));
        builder
    }
}