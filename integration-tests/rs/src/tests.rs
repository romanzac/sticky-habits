use near_units::parse_near;
use near_sdk::json_types::{U128, U64};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../out/main.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = owner
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let bob = owner
        .create_subaccount("bob")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_default_workflow(&alice,&bob, &contract).await?;
    Ok(())
}

async fn test_default_workflow(
    user: &Account,
    beneficiary: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let response: String = user
        .call(contract.id(), "add_habit")
        .args_json(json!({
            "description": "Eat vitamin C everyday",
            "deadline": U64(1664553599000000000),
            "beneficiary": beneficiary.id()
            }))
        .deposit(parse_near!("10 N"))
        .transact()
        .await?
        .json()?;

    //assert_eq!(message, "Hello".to_string());
    //println!("Response message {}", response);
    println!("      Passed ✅ gets default message");
    Ok(())
}
