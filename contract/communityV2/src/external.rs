use crate::*;
use near_sdk::{PromiseOrValue};

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self, 
        sender_id: AccountId, 
        amount: U128, 
        msg: String
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>{
        let ft_contract_id = env::predecessor_account_id();
        
        // Checking Platform Token Contract ID
        // require!(ft_contract_id == self.ft_id, "Paying must come from Platform Token");

        let signer_id = env::signer_account_id();
        assert_eq!(sender_id, signer_id, "Owner should be same with signer.");

        let proposal_id = msg.parse::<u64>().unwrap();

        self.vote(proposal_id.into(), amount);

        PromiseOrValue::Value(U128::from(0))
    }
}