use near_sdk::{near_bindgen, require, env, AccountId, Balance, BorshStorageKey};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U64, U128};

pub mod proposal;
pub mod internal;
pub mod util;

use crate::proposal::*;
use crate::internal::*;
use crate::util::*;

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
            self._vote(voter.clone(), proposal_id.0.clone(), amount.0.clone());
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
}