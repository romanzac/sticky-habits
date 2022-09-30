use near_units::parse_near;
use near_sdk::json_types::{U128, U64};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../out/main.wasm";

// Create custom Sticky Habits contract and setup the initial state.
async fn create_sticky_habits(
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<Contract> {
    let sh: Contract = worker
        .dev_deploy(&std::fs::read(WASM_FILEPATH)?)
        .await?;

    // Initialize our FT contract with owner metadata and total supply available
    // to be traded and transfered into other contracts such as Ref-Finance
    sh.call("init")
        .args_json(serde_json::json!({
            "owner": owner.id(),
            "dev_fee": 5u16,
            "habit_acquisition_period": U64(21*24*3600*1000000000),
            "approval_grace_period": U64(15*24*3600*1000000000)
        }))
        .transact()
        .await?
        .into_result()?;
    ();

    Ok(sh)
}

async fn test_default_workflow(
    user: &Account,
    beneficiary: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let response = user
        .call(contract.id(), "add_habit")
        .args_json(json!({
            "description": "Eat vitamin C everyday".to_string(),
            "deadline": U64(1664553599000000000),
            "beneficiary": beneficiary.id()
            }))
        .deposit(parse_near!("1 N"))
        .transact()
        .await?
        .into_result()?;

    //assert_eq!(message, "Hello".to_string());
    println!("Response message {:?}", response);
    println!("      Passed ✅ default workflow");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

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

    // Initialize smart contract
    let contract = create_sticky_habits(&owner, &worker).await?;

    // Begin tests
    let test1 = test_default_workflow(&alice,&bob, &contract).await?;
    println!("Test1 response {:?}", test1);

    Ok(())
}

