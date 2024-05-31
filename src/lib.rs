// ChallengeFactory:supreme-squirrel.testnet
// test nft: artbattle.mintspace2.testnet

use std::str::FromStr;

// Find all our documentation at https://docs.near.org
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::Base64VecU8,
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

// missing borsh serialize and de serializex
#[derive(Clone, Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
pub struct TokenMetadata {
    /// the Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
    pub title: Option<String>,
    /// free-form description of this token.
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    /// Base64-encoded sha256 hash of content referenced by the `media` field.
    /// Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    /// number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u16>,
    /// ISO 8601 datetime when token expires.
    pub expires_at: Option<String>,
    /// ISO 8601 datetime when token starts being valid.
    pub starts_at: Option<String>,
    /// When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>,
    /// URL to an off-chain JSON file with more info. The Mintbase Indexer refers
    /// to this field as `thing_id` or sometimes, `meta_id`.
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if
    /// `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
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
        id_prefix: String,
        name: String,
        description: String,
        image_link: String,
        reward_nft: String,
        challenge_nft_ids: std::vec::Vec<String>,
        _termination_date_in_ns: String,
        _winner_limit: String,
        // only necessary if contract will be minting reward nft
        reward_token_metadata: TokenMetadata,
    ) -> Promise {
        log!("Creating challenge: {}", name);
        assert!(
            env::attached_deposit().as_yoctonear() >= bytes_to_stake(400_000),
            "To cover the storage required for your store, you need to attach at least {} yoctoNEAR to this transaction.",
            bytes_to_stake(400_000)
        );
        self.assert_no_challenge_with_id(name.clone());
        let formatted_challenge_id = format!("{}.{}", id_prefix, env::current_account_id());
        let challenge_account_id = AccountId::from_str(&formatted_challenge_id).unwrap();
        let winner_limit: u64 = _winner_limit.parse().unwrap();
        let termination_date_in_ns: u64 = _termination_date_in_ns.parse().unwrap();

        Promise::new(challenge_account_id.clone())
            .create_account()
            .transfer(NearToken::from_yoctonear(bytes_to_stake(400_000)))
            .deploy_contract(include_bytes!("../wasm/nft-challenge.wasm").to_vec())
            .function_call(
                String::from("new"),
                json!({
                "owner_id": env::predecessor_account_id(),
                "name":name,
                "description":description,
                "image_link":image_link,
                "reward_nft":reward_nft,
                "_challenge_nft_ids": challenge_nft_ids,
                "termination_date_in_ns": termination_date_in_ns,
                "winner_limit": winner_limit,
                "reward_token_metadata": reward_token_metadata
                 })
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
