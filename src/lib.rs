use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{json_types::U128, near_bindgen, AccountId, Promise};

near_sdk::setup_alloc!();
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Faucet {
}

#[near_bindgen]
impl Faucet {
    pub fn pay(amount: U128, to: AccountId) -> Promise {
        Promise::new(to).transfer(amount.0)
    }
}
