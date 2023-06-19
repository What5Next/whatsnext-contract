use near_sdk::{near_bindgen, AccountId, Promise, Balance, env};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LazyOption};
use near_sdk::json_types::{U64, U128};

mod ext_vote;
use crate::ext_vote::*;

const INITIAL_BALANCE: Balance = 3_000_000_000_000_000_000_000_000 ;
const vote_code:&[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/voteV2.wasm");

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    votes: UnorderedMap<u64, AccountId>,
    current_vote: Option<AccountId>,
}

impl Default for Contract {
    fn default()->Self{
        Self{
            votes: UnorderedMap::new(b"d"),
            current_vote: None,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn vote(&self, proposal_id:U64, amount: U128) {
        let current_vote_account = self.current_vote.clone()
            .unwrap_or_else(||{panic!("No vote working")});

        ext_vote::ext_vote::ext(current_vote_account.clone())
        .vote(proposal_id, amount);
    }

    pub fn create_vote(&self) {
        let votes_id = String::from( (self.votes.len()+1).to_string() );
        let sub_account_id = AccountId::new_unchecked(
                format!("{}.{}", votes_id, env::current_account_id())
            );
        
        Promise::new(sub_account_id.clone())
        .create_account()
        .add_full_access_key(env::signer_account_pk())
        .transfer(INITIAL_BALANCE)
        .deploy_contract(vote_code.to_vec())
        .then(
            Self::ext(env::current_account_id())
            .callback_create_vote(sub_account_id.clone())
        );
    }

    pub fn callback_create_vote(&mut self, vote_account_id: AccountId){
        self.current_vote = Some(vote_account_id);
    }
}