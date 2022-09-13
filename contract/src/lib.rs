/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Balance, near_bindgen, log};
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U128};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habit {
    description: String,
    deadline: String,
    penalty: String,
    beneficiary: String
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
    pub fn get_habits(&self, from_index:Option<U128>, limit:Option<u64>) -> Vec<Habit>{
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.habits.iter()
            .skip(from as usize)
            .take(limit.unwrap_or(7) as usize)
            .collect()
    }

    // Adds new habit
    #[payable]
    pub fn add_habit(&mut self, description: String, deadline: String, penalty: String, beneficiary: String) {
        if self.habits.len() < 7 {
            log!("Adding new habit {}", description);
            self.habits.push(&Habit{ description, deadline, penalty, beneficiary });
        } else {
            log!("Only 7 habits are supported at the same time");
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_habit() {
        let mut contract = StickyHabits::default();
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            "2022/09/30".to_string(),
            "50".to_string(),
            "Adam".to_string(),
        );

        let posted_habit = &contract.get_habits(None, None)[0];
        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(posted_habit.penalty, "50".to_string());
    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabits::default();
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            "2022/09/30".to_string(),
            "50".to_string(),
            "Adam".to_string(),
        );
        contract.add_habit(
            "Eat one tomato every day".to_string(),
            "2022/09/20".to_string(),
            "150".to_string(),
            "Lubos".to_string(),
        );
        contract.add_habit(
            "Exercise without smartphone".to_string(),
            "2022/10/31".to_string(),
            "250".to_string(),
            "Arc".to_string(),
        );


        let habits = &contract.get_habits(None, None);
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits(Some(U128::from(1)), Some(2))[1];
        assert_eq!(last_habit.deadline, "2022/10/31".to_string());
        assert_eq!(last_habit.beneficiary, "Arc".to_string());
    }
}

