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

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

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
pub struct AccBuddy {
    habits: Vector<Habit>,
}


// Define the default, which automatically initializes the contract
impl Default for AccBuddy {
    fn default() -> Self{
        Self{ habits: Vector::new(b"m") }
    }
}

// Implement the contract structure
#[near_bindgen]
impl AccBuddy {

    // Returns an array of habits.
    pub fn get_habits(&self, from_index:Option<U128>, limit:Option<u64>) -> Vec<Habit>{
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.habits.iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
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
    fn get_default_greeting() {
        let contract = AccBuddy::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = AccBuddy::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
