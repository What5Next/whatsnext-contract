use near_sdk::{near_bindgen, AccountId};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

const community_factory_id: &str  = "init";
const vote_factory_id: &str = "init";
const content_factory_id: &str = "init";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    trinity: UnorderedSet<(AccountId, AccountId)>
}

impl Default for Contract{
    fn default()-> Self {
        Self { 
            trinity: UnorderedSet::new(b"d"),
        }
    }
}


impl Contract{
    // pub fn get_communities(&self) -> Vec<AccountId>{
    //     self.communities.to_vec()
    // }

    // pub fn add_communites(&mut self, community_account_id : AccountId) {
    //     self.communities.insert(&community_account_id);
    // }
}
