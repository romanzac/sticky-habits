use near_units::parse_near;
use near_sdk::json_types::{U64};
use serde_json::json;
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

    sh.call("init")
        .args_json(serde_json::json!({
            "owner": owner.id(),
            "dev_fee": 5u16,
            "habit_acquisition_period": U64(21*24*3600*1000000000),
            "approval_grace_period": U64(1*24*3600*1000000000)
        }))
        .transact()
        .await?
        .into_result()?;

    Ok(sh)
}

async fn test_default_workflow(
    user: &Account,
    beneficiary: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>
) -> anyhow::Result<()> {

    // Add habit
    let ah_res = user
        .call(contract.id(), "add_habit")
        .args_json(json!({
            "description": "Eat vitamin C everyday".to_string(),
            "deadline": U64(1664553599000000000),
            "beneficiary": beneficiary.id()
            }))
        .deposit(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    println!("Add habit response: {:?}\n", ah_res);

    // Update evidence
    let ue_res = user
        .call(contract.id(), "update_evidence")
        .args_json(json!({
            "at_index": U64(0),
            "evidence": "https://www.googlecloud.com/myfile.mov".to_string(),
            }))
        .transact()
        .await?
        .into_result()?;

    println!("Update evidence response: {:?}\n", ue_res);

    // Approve habit
    let ap_res = beneficiary
        .call(contract.id(), "approve_habit")
        .args_json(json!({
            "user": user.id(),
            "at_index": U64(0),
            }))
        .transact()
        .await?
        .into_result()?;

    println!("Approve habit response: {:?}\n", ap_res);

    // Forward time into future
    worker.fast_forward(150000).await?;

    let block_info = worker.view_latest_block().await?;
    println!("BlockInfo post-fast_forward {:?}", block_info);

    // let (timestamp, epoch_height): (u64, u64) =
    //     contract.call("current_env_data").view().await?.json()?;
    //println!("timestamp = {}, epoch_height = {}", timestamp, epoch_height);

    // Unlock deposit back to user
    let ud_res = user
        .call(contract.id(), "unlock_deposit")
        .args_json(json!({
            "user": user.id(),
            "at_index": U64(0),
            }))
        .transact()
        .await?
        .json::<String>()?;

    println!("Unlock habit response: {:?}\n", ud_res);

    println!("Passed ✅ default workflow");
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
    test_default_workflow(&alice,&bob, &contract, &worker).await?;


    Ok(())
}

