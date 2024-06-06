use near_sdk::NearToken;
use nft_challenger_generator::NFTTokenMetadata;
use serde_json::json;

#[tokio::test]
async fn test_can_create_challenge() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let challenge_creation_outcome = user_account
        .call(contract.id(), "create_challenge")
        .args_json(json!({
            "id_prefix": "test-challenge",
            "name": "Test Challenge!",
            "description": "A test description",
            "media_link": "A fake media link",
            "reward_nft_id": "reward-nft-id",
            "challenge_nft_ids": vec!["challenge-nft-id"],
            "_expiration_date_in_ns": "9007199254740991",
            "_winner_limit": "100",
            // only necessary if contract will be minting reward nft
            "reward_nft_metadata": NFTTokenMetadata{
                title: Some("Test Token".to_string()),
                description: Some("Test Token".to_string()),
                media: Some("https://www.creativeuncut.com/gallery-03/art/sa-sonic-05.jpg".to_string()),
                copies: Some(1),
                media_hash: None,
                expires_at: None,
                starts_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
        }))
        .max_gas()
        .deposit(NearToken::from_near(10))
        .transact()
        .await?;

    assert!(challenge_creation_outcome.is_success());

    let outcome_challenge_exists = contract
        .view("challenge_exists")
        .args_json(json!({
            "store_id":"test-challenge"
        }))
        .await?;

    assert!(outcome_challenge_exists.json::<bool>().unwrap());
    Ok(())
}
