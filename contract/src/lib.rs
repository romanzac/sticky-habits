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
    evidence: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StickyHabitsContract {
    owner: AccountId,
    balance: Balance,
    habits: UnorderedMap<AccountId, Vector<Habit>>,
}


// Define the default, which automatically initializes the contract
impl Default for StickyHabitsContract {
    fn default() -> Self{
        Self{
            owner: env::current_account_id(),
            balance: Balance::from(U128(0)),
            habits: UnorderedMap::new(b"d") }
    }
}

// Implement the contract structure
#[near_bindgen]
impl StickyHabitsContract {

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
                evidence });

            self.habits.insert(&user, &existing_habits);
            self.balance += Balance::from(to_lock);

            log!("Deposit of {} has been made for habit {}!", to_lock, description);

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
    use super::*;

    #[test]
    fn add_habit() {
        let mut contract = StickyHabitsContract::default();
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(1664553599000000000),
            AccountId::from_str("joe.near").unwrap(),
            "http://www.icloud.com/myfile.mov".to_string(),
        );

        let posted_habit = &contract.get_habits(None, None)[0];
        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(posted_habit.deposit, Balance::from(50u32));
    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabitsContract::default();
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            1664553599000000000,
            Balance::from(50u32),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1234").unwrap(),
            "http://www.icloud.com/myfile.mov".to_string(),
        );
        contract.add_habit(
            "Eat two tomato every day".to_string(),
            1664553599000000001,
            Balance::from(150u32),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1235").unwrap(),
            "http://www.icloud.com/myfile2.mov".to_string(),
        );
        contract.add_habit(
            "Exercise without smartphone".to_string(),
            1664553599000000002,
            Balance::from(3000u32),
            AccountId::from_str("roman.near").unwrap(),
            "http://www.icloud.com/myfile3.mov".to_string(),
        );


        let habits = &contract.get_habits(None, None);
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits(Some(U128::from(1)), Some(2))[1];
        assert_eq!(last_habit.deadline, 1664553599000000002);
        assert_eq!(last_habit.beneficiary, AccountId::from_str("roman.near").unwrap());
    }
}

