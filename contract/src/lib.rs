/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, AccountId, Balance, near_bindgen};
use near_sdk::collections::{Vector};
use near_sdk::json_types::{U128};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Action {
    description: String,
    deadline: String,
    penalty: String,
    beneficiary: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AccBuddy {
    actions: Vector<Action>,
}


// Define the default, which automatically initializes the contract
impl Default for AccBuddy{
    fn default() -> Self{
        Self{ actions: Vector::new() }
    }
}

// Implement the contract structure
#[near_bindgen]
impl AccBuddy {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn add_action(&mut self, description: String, deadline: String, penalty: String, beneficiary: String) {
        if self.actions.len() < 7 {
            log!("Adding new action {}", description);
            self.actions.push(Action{description, deadline, penalty, beneficiary});
        } else {
            log!("Only 7 actions are supported at the same time");
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
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
