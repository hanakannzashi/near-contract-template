use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
struct Contract {

}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {

        }
    }
}
