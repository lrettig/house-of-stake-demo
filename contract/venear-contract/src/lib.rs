use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VeNearContract {
    /// Owner of the contract
    pub owner_id: AccountId,
    /// Total supply of veNEAR tokens
    pub total_supply: Balance,
    /// Mapping of account ID to their locked NEAR balance
    pub locked_balances: UnorderedMap<AccountId, Balance>,
    /// Mapping of account ID to their veNEAR balance
    pub ve_balances: UnorderedMap<AccountId, Balance>,
}

#[near_bindgen]
impl VeNearContract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            total_supply: 0,
            locked_balances: UnorderedMap::new(b"l"),
            ve_balances: UnorderedMap::new(b"v"),
        }
    }

    // Basic view methods
    pub fn get_locked_balance(&self, account_id: AccountId) -> Balance {
        self.locked_balances.get(&account_id).unwrap_or(0)
    }

    pub fn get_ve_balance(&self, account_id: AccountId) -> Balance {
        self.ve_balances.get(&account_id).unwrap_or(0)
    }
} 