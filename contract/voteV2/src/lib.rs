use near_sdk::{near_bindgen, require, env, AccountId, Balance, BorshStorageKey};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;

pub mod proposal;
use crate::proposal::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    proposals : UnorderedMap<u64, Proposal>,
    proposal_total_votes: UnorderedMap<u64, Balance>,
    proposal_votes_with_accountId: LookupMap< AccountId, UnorderedMap<u64, Balance> >,
    total_votes : Balance,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Proposals,
    ProposalTotalVotes,
    ProposalVotes,
}

impl Default for Contract{
    fn default() -> Self{
        Self { 
            proposals: UnorderedMap::new(StorageKey::Proposals), 
            proposal_total_votes: UnorderedMap::new(StorageKey::ProposalTotalVotes), 
            proposal_votes_with_accountId: LookupMap::new(StorageKey::ProposalVotes), 
            total_votes: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    
}