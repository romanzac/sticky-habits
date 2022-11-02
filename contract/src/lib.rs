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
    approved: bool
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StickyHabitsContract {
    owner: AccountId,
    balance: Balance,
    dev_fee: u64,                  // percent
    habit_acquisition_period: u64, // Nanoseconds
    approval_grace_period: u64,    // Nanoseconds
    habits: UnorderedMap<AccountId, Vector<Habit>>,
}


// Define the default, which automatically initializes the contract
impl Default for StickyHabitsContract {
    fn default() -> Self {
        Self {
            owner: env::current_account_id(),
            balance: Balance::from(U128(0)),
            dev_fee: 5,
            habit_acquisition_period: 21*24*3600*1000000000 as u64,
            approval_grace_period: 15*24*3600*1000000000 as u64,
            habits: UnorderedMap::new(b"d") }
    }
}

// Implement the contract structure
#[near_bindgen]
impl StickyHabitsContract {
    #[init]
    #[private]
    pub fn init(owner: AccountId, dev_fee: U64, habit_acquisition_period: U64, approval_grace_period: U64) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            balance: Balance::from(U128(0)),
            dev_fee: u64::from(dev_fee),
            habit_acquisition_period: u64::from(habit_acquisition_period),
            approval_grace_period: u64::from(approval_grace_period),
            habits: UnorderedMap::new(b"d") }
    }

    // Returns an array of habits for the user with from and limit parameters.
    pub fn get_habits(&self, user: AccountId, from_index:Option<U128>, limit_to:Option<U64>) -> Vec<Habit> {
        let from = u128::from(from_index.unwrap_or(U128(0)));
        let limit = u64::from(limit_to.unwrap_or(U64(1)));

        let existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"m"),
        };

        existing_habits.iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    // Returns actual contract balance
    pub fn get_balance(&self) -> U64 {
        assert!(env::state_exists(), "Not initialized yet");
        U64(self.balance as u64)
    }


    // TODO: Add one more get function for habits assigned to a beneficiary from multiple users

    // Adds new habit
    #[payable]
    pub fn add_habit(&mut self, description: String, deadline_extension: U64, beneficiary: AccountId) {
            log!("Adding new habit: {}", description);
            // Get who is calling the method and how much $NEAR they attached
            let user: AccountId = env::predecessor_account_id();
            let deposit: Balance = env::attached_deposit();
            let deadline = env::block_timestamp() + self.habit_acquisition_period +
                u64::from(deadline_extension);

            // Check if user is different from beneficiary
            assert_ne!(user, beneficiary, "User and Beneficiary should be different accounts");

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
                deadline: U64(deadline),
                deposit: U128(to_lock),
                beneficiary,
                evidence: "".to_string(),
                approved: false
            });

            self.habits.insert(&user, &existing_habits);
            self.balance += Balance::from(to_lock);

            log!("Deposit of {} has been made for habit {}", to_lock, description);

    }

    #[payable]
    pub fn update_evidence(&mut self, at_index: U64, evidence: String) {
        let index = u64::from(at_index);
        let user: AccountId = env::predecessor_account_id();

        log!("Updating habit evidence for user {}", user);

        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"m"),
        };
        if existing_habits.len() > index {
            match &mut existing_habits.get(index) {
                Some(habit) => {
                    habit.evidence = evidence;
                    let _evicted = existing_habits.replace(index, habit);
                },
                None => (),
            };
        }
    }

    // Beneficiary sets habit's flag to approved
    #[payable]
    pub fn approve_habit(&mut self, user: AccountId, at_index: U64) -> bool {
        let index = u64::from(at_index);
        let beneficiary: AccountId = env::predecessor_account_id();
        let current_time = env::block_timestamp();

        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"m"),
        };
        if existing_habits.len() > index {
            match &mut existing_habits.get(index) {
                Some(habit) => {
                    if habit.beneficiary == beneficiary &&
                       u64::from(habit.deadline) < current_time &&
                        u64::from(habit.deadline) + self.approval_grace_period > current_time {
                            habit.approved = true;
                            let _evicted = existing_habits.replace(index, habit);
                            return true;
                    }
                },
                None => (),
            };
        }
        false
    }

    #[payable]
    pub fn unlock_deposit(&mut self, user: AccountId, at_index: U64) -> String {
        let index = u64::from(at_index);
        let account: AccountId = env::predecessor_account_id();
        let current_time = env::block_timestamp();

        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"m"),
        };
        if existing_habits.len() > index {
            match &mut existing_habits.get(index) {
                Some(habit) => {
                    // Return all deposit to user if conditions met
                    if account == user && habit.approved &&
                        u64::from(habit.deadline) + self.approval_grace_period < current_time {
                        Promise::new(account.clone()).transfer(u128::from(habit.deposit));
                        self.balance -= u128::from(habit.deposit);
                        habit.deposit = U128(0);
                        let _evicted = existing_habits.replace(index, habit);
                        return user.to_string();
                    }
                    // Split deposit between developer and beneficiary if conditions met
                    if account == habit.beneficiary && !habit.approved &&
                        u64::from(habit.deadline) + self.approval_grace_period < current_time {
                            let to_beneficiary = u128::from(habit.deposit) / (100-self.dev_fee as u128);
                            let to_developer = u128::from(habit.deposit) - to_beneficiary;
                            Promise::new(account.clone()).transfer(to_beneficiary);
                            Promise::new(self.owner.clone()).transfer(to_developer);

                            self.balance -= u128::from(habit.deposit);
                            habit.deposit = U128(0);
                            let _evicted = existing_habits.replace(index, habit);
                            return account.to_string();
                    }
                },
                None => (),
            };
        }
        "".to_string()
    }

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
    fn set_context(predecessor: &str, amount: Balance, timestamp: u64) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);
        builder.block_timestamp(timestamp);

        testing_env!(builder.build());
    }

    #[test]
    fn initializes() {
        let contract = StickyHabitsContract::init(
            OWNER.parse().unwrap(),
            U64(7),
            U64(1*24*3600*1000000000),
            U64(1*24*3600*1000000000));
        assert_eq!(contract.owner, OWNER.parse().unwrap())
    }

    #[test]
    fn adds_habit() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 10*NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("adam").unwrap()
        );

        let posted_habit = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                                None, None)[0];
        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(u128::from(posted_habit.deposit), 10*NEAR-STORAGE_COST);
    }

    #[test]
    fn updates_evidence() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 10*NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("adam").unwrap()
        );

        set_context("roman", 10*NEAR, 1664172263000000000);
        contract.add_habit(
            "Wake up every day at the same time".to_string(),
            U64(0),
            AccountId::from_str("maria").unwrap()
        );

        contract.update_evidence(U64(1),"https://www.icloud.com/myfile.mov".to_string());

        let updated_habit = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                                 None, Some(U64(2)))[1];
        assert_eq!(updated_habit.evidence, "https://www.icloud.com/myfile.mov".to_string());

    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 20*NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap()
        );

        set_context("roman", 20*NEAR, 1664172263000000000);
        contract.add_habit(
            "Eat two tomatoes every day".to_string(),
            U64(0),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1235").unwrap()
        );

        set_context("roman", 20*NEAR, 1664172263000000000);
        contract.add_habit(
            "Exercise without smartphone".to_string(),
            U64(60000000000),
            AccountId::from_str("alice").unwrap()
        );

        let habits = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                          None, Some(U64(3)));
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits(AccountId::from_str("roman").unwrap(),
                                              Some(U128(1)), Some(U64(2)))[1];
        assert_eq!(u64::from(last_habit.deadline), 1664172263000000000 +
            contract.habit_acquisition_period + 60000000000);
        assert_eq!(last_habit.beneficiary, AccountId::from_str("alice").unwrap());
        assert_eq!(last_habit.approved, false);
    }

    #[test]
    pub fn unlocks_deposit() {
        // Add habit
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 20*NEAR, 1662312790000000000);
        contract.add_habit(
            "Do 15 push-ups everyday".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap()
        );

        // Failed unlock from user side - on habit not approved
        set_context("roman", 0, 1663132260000000000);
        let receiver = contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));
        assert_eq!(receiver, "".to_string());


        // Failed unlock from beneficiary side - on too early
        set_context("josef", 0, 1663132260000000000);
        let receiver = contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));
        assert_eq!(receiver, "".to_string());

        // Success unlock from user side
        set_context("josef", 0, 1664302901000000000);
        contract.approve_habit(AccountId::from_str("roman").unwrap(), U64(0));
        set_context("roman", 0, 1665771701000000000);
        let receiver = contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));
        assert_eq!(receiver, "roman".to_string());

        // Success unlock from beneficiary side
        set_context("roman", 20*NEAR, 1662312790000000000);
        contract.add_habit(
            "Eat vegetarian food once a day".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap()
        );
        set_context("josef", 0, 1665771701000000000);
        let receiver = contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(1));
        assert_eq!(receiver, "josef".to_string());

    }


}

