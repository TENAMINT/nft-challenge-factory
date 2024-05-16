// ChallengeFactory:supreme-squirrel.testnet
// test nft: artbattle.mintspace2.testnet

use std::str::FromStr;

// Find all our documentation at https://docs.near.org
use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    log, near,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, NearToken, Promise,
};
pub mod external;

use near_sdk::{env, store::LookupSet};

/**
* ALL STUFF THAT WILL EVENTUALLY NEED TO BE MOvED/MODULARIZED
*/
pub const YOCTO_PER_BYTE: u128 = 10_000_000_000_000_000_000;

const fn bytes_to_stake(bytes: u128) -> u128 {
    (bytes as u128) * YOCTO_PER_BYTE
}

#[near(contract_state)]
pub struct ChallengeFactory {
    pub challenges: LookupSet<String>,
}

impl Default for ChallengeFactory {
    fn default() -> Self {
        Self {
            challenges: LookupSet::new(b"a".to_vec()),
        }
    }
}

// Implement the contract structure
#[near]
impl ChallengeFactory {
    /// If a `Challenge` with `challenge_id` has been produced by this `Factory`, return `true`.
    pub fn check_contains_challenge(&self, store_id: String) -> bool {
        self.challenges.contains(&store_id)
    }

    /// Panics if a store with the requested ID already exists
    pub fn assert_no_challenge_with_id(&self, store_id: String) {
        assert!(
            !self.check_contains_challenge(store_id),
            "Challenge with that ID already exists"
        );
    }

    #[payable]
    pub fn create_challenge(
        &mut self,
        name: String,
        challenges: Vec<String>,
        termination_date: String,
        winner_limit: String,
        reward_nft: String,
    ) -> Promise {
        log!("Creating challenge: {}", name);
        assert!(
            env::attached_deposit().as_yoctonear() >= bytes_to_stake(400_000),
            "To cover the storage required for your store, you need to attach at least {} yoctoNEAR to this transaction.",
            bytes_to_stake(400_000)
        );
        self.assert_no_challenge_with_id(name.clone());
        let formatted_challenge_id = format!("{}.{}", name, env::current_account_id());
        let challenge_account_id = AccountId::from_str(&formatted_challenge_id).unwrap();
        let winner_limit_parsed :u64 = winner_limit.parse().unwrap();
        let termination_date_parsed : u64= termination_date.parse().unwrap();
        Promise::new(challenge_account_id.clone())
            .create_account()
            .transfer(NearToken::from_yoctonear(bytes_to_stake(400_000)))
            .deploy_contract(include_bytes!("../wasm/nft-challenge.wasm").to_vec())
            .function_call(
                String::from("new"),
                json!({ 
                        "owner_id":env::predecessor_account_id(),
                        "name":name,
                        "challenge_nfts":challenges,
                        "termination_date":termination_date_parsed,
                        "winner_limit":winner_limit_parsed,
                        "reward_nft":reward_nft})
                .to_string()
                .into_bytes(),
                NearToken::from_near(0),
                Gas::from_tgas(35),
            )
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     // #[should_panic(expected = "There was an error contacting NFT ChallengeFactory")]
//     fn get_default_greeting() {
//         let contract = ChallengeFactory::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         let t = contract.check_can_mint("test".to_string());
//     }
// }
