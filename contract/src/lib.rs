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
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U128};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habit {
    description: String,
    deadline: u64,
    penalty: Balance,
    beneficiary: AccountId,
    evidence: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StickyHabits {
    habits: Vector<Habit>,
}


// Define the default, which automatically initializes the contract
impl Default for StickyHabits {
    fn default() -> Self{
        Self{ habits: Vector::new(b"m") }
    }
}

// Implement the contract structure
#[near_bindgen]
impl StickyHabits {

    // Returns an array of habits.
    pub fn get_habits(&self, from_index:Option<U128>, limit:Option<u64>) -> Vec<Habit> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.habits.iter()
            .skip(from as usize)
            .take(limit.unwrap_or(7) as usize)
            .collect()
    }

    // Adds new habit
    #[payable]
    pub fn add_habit(&mut self, description: String, deadline: u64, penalty: Balance,
                     beneficiary: AccountId, evidence: String) {
        if self.habits.len() < 7 {
            log!("Adding new habit {}", description);
            self.habits.push(&Habit{ description, deadline, penalty,
                beneficiary, evidence });
        } else {
            log!("Only 7 habits are supported at the same time");
        }
    }

    // TODO: implement lock by user and unlock by his friend
    // How about evidence screenshot or selfie for the friend
    // 1) user locks to smart contract
    // 2) user does the thing and gathers evidence
    // 3) both user and friend should approve it was or wasn't done.
    //    If both agree it was done, user receives money back,
    //    if both agree it wasn't dont, friend receives money back and
    //    if they cannot agree, developer gets money :)
    pub fn transfer(&self, to: AccountId, amount: Balance) {
        Promise::new(to).transfer(amount);
    }
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
        let mut contract = StickyHabits::default();
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            1664553599000000000,
            Balance::from(50u32),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba0109").unwrap(),
            "http://www.icloud.com/myfile.mov".to_string(),
        );

        let posted_habit = &contract.get_habits(None, None)[0];
        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(posted_habit.penalty, Balance::from(50u32));
    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabits::default();
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
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1236").unwrap(),
            "http://www.icloud.com/myfile3.mov".to_string(),
        );


        let habits = &contract.get_habits(None, None);
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits(Some(U128::from(1)), Some(2))[1];
        assert_eq!(last_habit.deadline, 1664553599000000002);
        assert_eq!(last_habit.beneficiary, AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1236").unwrap());
    }
}

