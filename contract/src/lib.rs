/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Balance, near_bindgen, log, Promise};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::{U128, U64};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habit {
    description: String,
    deadline: U64,
    deposit: U128,
    beneficiary: AccountId,
    evidence: String,
    approved_user: bool,
    approved_beneficiary: bool
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StickyHabitsContract {
    owner: AccountId,
    balance: Balance,
    habit_acquisition_period: u16, // Days
    approval_grace_period: u16,    // Days
    habits: UnorderedMap<AccountId, Vector<Habit>>,
}


// Define the default, which automatically initializes the contract
impl Default for StickyHabitsContract {
    fn default() -> Self {
        Self {
            owner: env::current_account_id(),
            balance: Balance::from(U128(0)),
            habit_acquisition_period: 21,
            approval_grace_period: 30,
            habits: UnorderedMap::new(b"d") }
    }
}

// Implement the contract structure
#[near_bindgen]
impl StickyHabitsContract {
    pub fn init(owner: AccountId, habit_acquisition_period: u16, approval_grace_period: u16) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            balance: Balance::from(U128(0)),
            habit_acquisition_period,
            approval_grace_period,
            habits: UnorderedMap::new(b"d") }
    }

    // Returns an array of habits for the user with from and limit parameters.
    pub fn get_habits(&self, user: AccountId, from_index:Option<U128>, limit:Option<u64>) -> Vec<Habit> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        let existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"m"),
        };

        existing_habits.iter()
            .skip(from as usize)
            .take(limit.unwrap_or(7) as usize)
            .collect()
    }

    // Adds new habit
    #[payable]
    pub fn add_habit(&mut self, description: String, deadline: U64, beneficiary: AccountId,
                     evidence: String) {
            log!("Adding new habit {}", description);
            // Get who is calling the method and how much $NEAR they attached
            let user: AccountId = env::predecessor_account_id();
            let deposit: Balance = env::attached_deposit();

            // Check if user has already any stored habits
            let mut existing_habits = match self.habits.get(&user) {
                Some(i) => i,
                None => Vector::new(b"m"),
            };

            let to_lock: Balance = if existing_habits.len() == 0 {
                 // This is the user's first deposit, lets register it, which increases storage
                 assert!(deposit > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);

                // Subtract the storage cost to the amount to transfer
                deposit - STORAGE_COST
            } else {
                deposit
            };

            existing_habits.push(&Habit{
                description: description.clone(),
                deadline,
                deposit: U128::from(to_lock),
                beneficiary,
                evidence,
                approved_user: true,
                approved_beneficiary: false
            });

            self.habits.insert(&user, &existing_habits);
            self.balance += Balance::from(to_lock);

            log!("Deposit of {} has been made for habit {}!", to_lock, description);

    }

    #[payable]
    pub fn unlock_deposit(&mut self, user: AccountId, description: String, from_index:Option<U128>) -> Promise {
        let limit = Some(0);
        Promise::new(user)
    }

    #[payable]
    pub fn update_evidence(&mut self, user: AccountId, description: String, from_index:Option<U128>,
                        evidence: String) {
        let limit = Some(0);

    }

    #[payable]
    pub fn approve_result_user(&mut self, user: AccountId, beneficiary: AccountId, description: String,
                          from_index:Option<U128>, approved: bool) {
        let limit = Some(0);
    }

    #[payable]
    pub fn approve_result_beneficiary(&mut self, user: AccountId, beneficiary: AccountId, description: String,
                               from_index:Option<U128>, approved: bool) {
        let limit = Some(0);
    }


    // TODO: implement lock by user and unlock by his friend
    // 1) user locks the deposit
    // 2) user keeps doing the habit and gathers evidence until deadline
    // 3) both user and friend should approve habit was or wasn't done.
    //    if both agree it was done, user receives money back - deposit is unlocked,
    //    if both agree it wasn't done, friend receives the deposit,
    //    if they cannot agree, smart contract (developer) receives the deposit after grace period :)


}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;
    use super::*;

    const OWNER: &str = "joe";
    const NEAR: u128 = 1000000000000000000000000;

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

    #[test]
    fn initializes() {
        let contract = StickyHabitsContract::init(OWNER.parse().unwrap(),
                                                  66, 30);
        assert_eq!(contract.owner, OWNER.parse().unwrap())
    }

    #[test]
    fn add_habit() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 10*NEAR);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(1664553599000000000),
            AccountId::from_str("adam").unwrap(),
            "http://www.icloud.com/myfile.mov".to_string(),
        );

        let posted_habit = &contract.get_habits(AccountId::from_str("roman").unwrap(),None, None)[0];
        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(posted_habit.deposit, U128(10*NEAR-STORAGE_COST));
    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 20*NEAR);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(1664553599000000000),
            AccountId::from_str("josef").unwrap(),
            "https://www.icloud.com/myfile.mov".to_string(),
        );

        set_context("roman", 20*NEAR);
        contract.add_habit(
            "Eat two tomatoes every day".to_string(),
            U64(1664553599000000001),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1235").unwrap(),
            "https://www.icloud.com/myfile2.mov".to_string(),
        );
        set_context("roman", 20*NEAR);
        contract.add_habit(
            "Exercise without smartphone".to_string(),
            U64(1664553599000000002),
            AccountId::from_str("alice").unwrap(),
            "http://www.icloud.com/myfile3.mov".to_string(),
        );


        let habits = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                          None, None);
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                              Some(U128::from(1)), Some(2))[1];
        assert_eq!(last_habit.deadline, U64(1664553599000000002));
        assert_eq!(last_habit.beneficiary, AccountId::from_str("alice").unwrap());
        assert_eq!(last_habit.approved_user, true);
        assert_eq!(last_habit.approved_beneficiary, false);
    }
}
