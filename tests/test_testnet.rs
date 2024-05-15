use std::env;
use std::str::FromStr;

use near_sdk::log;
use near_sdk::AccountId;
use near_sdk::NearToken;
use near_workspaces::types::SecretKey;
use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational_on_testnet() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::testnet().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let test = SecretKey::from_str("ed25519:2LZhvFqhQ6EKCoVy7UexFWCaCpkgZNsFaYJgbG5zcqCL76H9mCQ8JQbFewedUGW4a2CyPLrfypAHZpdiRLD75hjM").unwrap();
    let res = sandbox
        .create_tla(AccountId::from_str("shakiran.testnet").unwrap(), test)
        .await?;
    let account = res.result;
    log!(account.id());

    let contract_sk = SecretKey::from_str("ed25519:63oo5n8eeLHB4jLkqqkJLmMgntc3hv1BXXgR8YeAzGgcCTSZ1C9JQvVgxqzshJMyDaCpGeQaJ66cw2z9nZf9XnP1").unwrap();
    let contract = sandbox
        .create_tla_and_deploy(
            AccountId::from_str("supreme-squirrel.testnet").unwrap(),
            contract_sk,
            &contract_wasm,
        )
        .await?
        .unwrap();

    let outcome = account
        .call(contract.id(), "create_challenge")
        .args_json(json!({
        "name": "test_challenge2",
        "challenge_nft": "test_challenge_nft",
        "termination_date": 1000,
        "winner_limit": 1,
        "reward_nft": "a12345x.testnet"}))
        .max_gas()
        .deposit(NearToken::from_near(4))
        .transact()
        .await?;

    assert!(outcome.is_success());
    Ok(())
}
