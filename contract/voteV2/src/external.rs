use crate::*;

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String);
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String){
        let ft_contract_id = env::predecessor_account_id();
        
        // Checking Platform Token Contract ID
        // require!(ft_contract_id == self.ft_id, "Paying must come from Platform Token");

        let signer_id = env::signer_account_id();
        assert_eq!(sender_id, signer_id, "Owner should be same with signer.");

        let proposal_id = msg.parse::<u64>().unwrap();

        self._vote(sender_id, proposal_id, amount.0);
    }
}