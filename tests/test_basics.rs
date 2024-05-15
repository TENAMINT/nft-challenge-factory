use std::env;
use std::str::FromStr;

use near_sdk::log;
use near_sdk::AccountId;
use near_sdk::NearToken;
use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    log!("User account: {}", user_account.secret_key());

    let outcome = user_account
        .call(contract.id(), "create_challenge")
        .args_json(json!({
            "challenge_name": "test_challenge",
            "challenge_nft": "test_challenge_nft",
            "termination_date": 1000,
            "winner_limit": 1,
            "reward_nft": "test_reward_nft"}))
        .max_gas()
        .deposit(NearToken::from_near(10))
        .transact()
        .await?;
    log!("Outcome: {:?}", outcome.failures());

    for log in outcome.logs() {
        log!("Log: {}", log);
    }

    assert!(outcome.is_success());

    // let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
    // assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}
