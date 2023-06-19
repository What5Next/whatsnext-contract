use crate::*;

use near_sdk::{ext_contract};

#[ext_contract(ext_vote)]
trait ExternalVote {
    fn vote(&mut self, proposal_id: U64, amount: U128);
}