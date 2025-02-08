use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub proposer: AccountId,
    pub description: String,
    pub votes_for: u128,
    pub votes_against: u128,
    pub status: ProposalStatus,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VotingContract {
    /// Owner of the contract
    pub owner_id: AccountId,
    /// veNEAR token contract address
    pub ve_token: AccountId,
    /// List of proposals
    pub proposals: Vector<Proposal>,
    /// Mapping of account ID to their votes
    pub votes: UnorderedMap<(u64, AccountId), bool>,
}

#[near_bindgen]
impl VotingContract {
    #[init]
    pub fn new(owner_id: AccountId, ve_token: AccountId) -> Self {
        Self {
            owner_id,
            ve_token,
            proposals: Vector::new(b"p"),
            votes: UnorderedMap::new(b"v"),
        }
    }

    // Basic view methods
    pub fn get_proposal(&self, id: u64) -> Option<Proposal> {
        self.proposals.get(id)
    }
} 