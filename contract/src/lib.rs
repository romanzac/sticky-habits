use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, AccountId, Balance, near_bindgen, log, Promise};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::json_types::{U128, U64};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;


#[near_bindgen]
#[derive(Serialize, Deserialize, Debug, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Habit {
    description: String,
    deadline: U64,
    deposit: U128,
    beneficiary: AccountId,
    evidence: String,
    approved: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StickyHabitsContract {
    owner: AccountId,
    balance: Balance,
    dev_fee: u64,                                   // percent
    habit_acquisition_period: u64,                  // Nanoseconds
    approval_grace_period: u64,                     // Nanoseconds
    habits: UnorderedMap<AccountId, Vector<Habit>>,
    beneficiaries: UnorderedMap<AccountId, Vector<AccountId>>,
}


// Default, which automatically initializes the contract during first call
impl Default for StickyHabitsContract {
    fn default() -> Self {
        Self {
            owner: env::current_account_id(),
            balance: Balance::from(U128(0)),
            dev_fee: 5,
            habit_acquisition_period: 21 * 24 * 3600 * 1000000000 as u64,
            approval_grace_period: 15 * 24 * 3600 * 1000000000 as u64,
            habits: UnorderedMap::new(b"map-id-1".to_vec()),
            beneficiaries: UnorderedMap::new(b"map-id-2".to_vec()),
        }
    }
}

#[near_bindgen]
impl StickyHabitsContract {
    #[init]
    #[private]
    pub fn init(
        owner: AccountId,
        dev_fee: U64,
        habit_acquisition_period: U64,
        approval_grace_period: U64,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner,
            balance: Balance::from(U128(0)),
            dev_fee: u64::from(dev_fee),
            habit_acquisition_period: u64::from(habit_acquisition_period),
            approval_grace_period: u64::from(approval_grace_period),
            habits: UnorderedMap::new(b"map-id-1".to_vec()),
            beneficiaries: UnorderedMap::new(b"map-id-2".to_vec()),
        }
    }

    // Adds new habit for the user and links user to his beneficiary
    #[payable]
    pub fn add_habit(
        &mut self,
        description: String,
        deadline_extension: U64,
        beneficiary: AccountId,
    ) {
        log!("Adding new habit: {}", description);
        // Get who is calling the method and how much $NEAR they attached
        let user: AccountId = env::predecessor_account_id();
        let user_str = user.as_str();
        let beneficiary_str = beneficiary.as_str();
        let deposit: Balance = env::attached_deposit();
        let deadline = env::block_timestamp() + self.habit_acquisition_period +
            u64::from(deadline_extension);

        // Check if user is different from beneficiary
        assert_ne!(user, beneficiary, "User and Beneficiary should be different accounts");

        // Check if user has already any stored habits, if not create new vector
        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(("vector-h-id-".to_string() + user_str).as_bytes().to_vec())
        };

        let to_lock: Balance = if existing_habits.len() == 0 {
            // This is the user's first deposit, lets register it, which increases storage
            assert!(deposit > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);

            // Subtract the storage cost to the amount to transfer
            deposit - STORAGE_COST
        } else {
            deposit
        };

        existing_habits.push(&Habit {
            description: description.clone(),
            deadline: U64(deadline),
            deposit: U128(to_lock),
            beneficiary: beneficiary.clone(),
            evidence: "".to_string(),
            approved: false,
        });

        self.habits.insert(&user, &existing_habits);
        self.balance += Balance::from(to_lock);

        log!("Deposit of {} has been made for habit {}", to_lock, description);

        // Check if beneficiary has been assigned any users(habits) before, if not create new vector
        let mut beneficiary_users = match self.beneficiaries.get(&beneficiary) {
            Some(v) => v,
            None => Vector::new(("vector-b-id-".to_string() + beneficiary_str).as_bytes().to_vec())
        };

        // Link user with the beneficiary if not already present
        match beneficiary_users.iter().find(|x| *x == user) {
            Some(_item) => (),
            None => {
                // Add new or update beneficiary with this user
                beneficiary_users.push(&user);
                self.beneficiaries.insert(&beneficiary, &beneficiary_users);
                log!("User {} assigned to the beneficiary {}", user, beneficiary);
            }
        }
    }

    // Adds a single link to the video or image content or cloud storage folder
    #[payable]
    pub fn update_evidence(
        &mut self,
        user: AccountId,
        at_index: U64,
        evidence: String,
    ) {
        self.update_habit(at_index,"update_evidence", user, evidence);
    }

    // Beneficiary approves habit by setting "approved" flag to true
    #[payable]
    pub fn approve_habit(
        &mut self,
        user: AccountId,
        at_index: U64,
    ) {
        self.update_habit(at_index,"approve", user, "".into());
    }

    #[payable]
    pub fn unlock_deposit(
        &mut self,
        user: AccountId,
        at_index: U64,
    ) {
        let index = u64::from(at_index);
        let account: AccountId = env::predecessor_account_id();
        let current_time = env::block_timestamp();

        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => {
                panic!("User {} has no habit yet", user);
            },
        };

        match &mut existing_habits.get(index) {
            Some(habit) => {
                let orig_deposit = u128::from(habit.deposit);
                let orig_deadline = u64::from(habit.deadline);

                // Return all deposit to the requesting user if conditions met
                if account == user && habit.approved &&
                    orig_deadline + self.approval_grace_period < current_time
                {
                    Promise::new(account.clone()).transfer(orig_deposit);
                    self.balance -= orig_deposit;
                    habit.deposit = U128(0);
                    let _updated = existing_habits.replace(index, habit);
                }
                // Split deposit between developer and beneficiary if conditions met, call by beneficiary
                if account == habit.beneficiary && !habit.approved &&
                    orig_deadline + self.approval_grace_period < current_time
                {
                    let to_beneficiary = orig_deposit / (100 - self.dev_fee as u128);
                    let to_developer = orig_deposit - to_beneficiary;
                    Promise::new(account.clone()).transfer(to_beneficiary);
                    Promise::new(self.owner.clone()).transfer(to_developer);

                    self.balance -= orig_deposit;
                    habit.deposit = U128(0);
                    let _updated = existing_habits.replace(index, habit);
                }
            }
            None => panic!("Index {} is out of range", index),
        };
    }

    // Returns an array of habits for the user with from and limit parameters.
    pub fn get_habits_user(
        &self,
        user: AccountId,
        from_index: Option<U128>,
        limit_to: Option<U64>,
    ) -> Vec<Habit> {
        let from = u128::from(from_index.unwrap_or(U128(0)));
        let limit = u64::from(limit_to.unwrap_or(U64(1)));

        let existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => Vector::new(b"vector-id-1".to_vec()),
        };

        existing_habits.iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    // Returns a map of habits of beneficiary's friends with from and limit parameters.
    pub fn get_habits_beneficiary(
        &self,
        beneficiary: AccountId,
        from_index: Option<U128>,
        limit_to: Option<U64>,
    ) -> HashMap<AccountId, Vec<Habit>> {
        let from = u128::from(from_index.unwrap_or(U128(0)));
        let limit = u64::from(limit_to.unwrap_or(U64(1)));

        let mut friends_habits: HashMap<AccountId, Vec<Habit>> = HashMap::new();

        // Get users associated with beneficiary
        let beneficiary_users = match self.beneficiaries.get(&beneficiary) {
            Some(v) => v,
            None => Vector::new(b"vector-id-2".to_vec()),
        };

        // Get habits from all associated users and filter them to those belonging to beneficiary
        for user in beneficiary_users.iter() {
            let user_habits = match self.habits.get(&user) {
                Some(v) => v,
                None => Vector::new(b"vector-id-1".to_vec()),
            };
            let user_habits_filtered: Vec<Habit> = user_habits.iter()
                .skip(from as usize)
                .take(limit as usize)
                .filter(|b| b.beneficiary == beneficiary)
                .collect();
            friends_habits.insert(user, user_habits_filtered);
        }

        friends_habits
    }

    // Returns actual contract balance
    pub fn get_balance(&self) -> U64 {
        assert!(env::state_exists(), "Not initialized yet");
        U64(self.balance as u64)
    }

    fn update_habit(
        &mut self,
        at_index: U64,
        action: &str,
        user: AccountId,
        evidence: String,
    ) {
        let index = u64::from(at_index);
        let account: AccountId = env::predecessor_account_id();
        let current_time = env::block_timestamp();

        log!("Updating habit with action {} for user {}", action, user);

        let mut existing_habits = match self.habits.get(&user) {
            Some(v) => v,
            None => {
                panic!("User {} has no habit yet", user);
            },
        };

        match &mut existing_habits.get(index) {
            Some(habit) => {
                match action {
                    "update_evidence" => {
                        assert_eq!(user, account, "User can update evidence only for her own habit");
                        self.update_evidence_action(index, &mut existing_habits, habit, evidence);
                    },
                    "approve" => {
                        assert_eq!(habit.beneficiary, account, "Only beneficiary can approve habit for user");
                        self.approve_action(index, &mut existing_habits, habit, current_time);
                    },
                    _ => {}
                };
            },

            None => panic!("Index {} is out of range", index),
        }
    }

    fn update_evidence_action(
        &self,
        index: u64,
        existing_habits: &mut Vector<Habit>,
        habit: &mut Habit,
        evidence: String,
    ) {
        habit.evidence = evidence;
        let _updated = existing_habits.replace(index, habit);
    }

    fn approve_action(
        &self,
        index: u64,
        existing_habits: &mut Vector<Habit>,
        habit: &mut Habit,
        current_time: u64
    ) {
        let orig_deadline = u64::from(habit.deadline);

        if  orig_deadline < current_time &&
            orig_deadline + self.approval_grace_period > current_time
        {
            habit.approved = true;
            let _updated = existing_habits.replace(index, habit);
        }
    }

}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::Balance;
    use super::*;

    const OWNER: &str = "joe";
    const NEAR: u128 = 1000000000000000000000000;

    // Auxiliary fn: create a mock context
    fn set_context(
        predecessor: &str,
        amount: Balance,
        timestamp: u64,
    ) {
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
            U64(1 * 24 * 3600 * 1000000000),
            U64(1 * 24 * 3600 * 1000000000));
        assert_eq!(contract.owner, OWNER.parse().unwrap())
    }

    #[test]
    fn adds_habit() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 10 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("adam").unwrap(),
        );

        set_context("adam", 10 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Help father with car repair".to_string(),
            U64(0),
            AccountId::from_str("roman").unwrap(),
        );

        let posted_habit = &contract.get_habits_user(AccountId::from_str("roman").unwrap(),
                                                     None, None)[0];

        let friends_habits = contract.get_habits_beneficiary(
            AccountId::from_str("adam").unwrap(),
            None, None);
        let romans_habits = friends_habits.get(&AccountId::from_str("roman").unwrap()).unwrap();

        assert_eq!(posted_habit.description, "Clean my keyboard once a week".to_string());
        assert_eq!(u128::from(posted_habit.deposit), 10 * NEAR - STORAGE_COST);
        assert_eq!(romans_habits[0].description, "Clean my keyboard once a week".to_string());
    }

    #[test]
    fn updates_evidence() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 10 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("adam").unwrap(),
        );

        set_context("roman", 10 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Wake up every day at the same time".to_string(),
            U64(0),
            AccountId::from_str("maria").unwrap(),
        );

        contract.update_evidence(AccountId::from_str("roman").unwrap(), U64(1),
                                 "https://www.icloud.com/myfile.mov".to_string());

        let updated_habit = &contract.get_habits_user(AccountId::from_str("roman").unwrap(),
                                                      None, Some(U64(2)))[1];
        assert_eq!(updated_habit.evidence, "https://www.icloud.com/myfile.mov".to_string());
    }

    #[test]
    fn iterates_habits() {
        let mut contract = StickyHabitsContract::default();

        set_context("roman", 20 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Clean my keyboard once a week".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap(),
        );

        set_context("roman", 20 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Eat two tomatoes every day".to_string(),
            U64(0),
            AccountId::from_str("b3b3bccd6ceee15c1610421568a03b5dcff6d1672374840d4da2c38c15ba1235").unwrap(),
        );

        set_context("roman", 20 * NEAR, 1664172263000000000);
        contract.add_habit(
            "Exercise without smartphone".to_string(),
            U64(60000000000),
            AccountId::from_str("alice").unwrap(),
        );

        let habits = &contract.get_habits_user(AccountId::from_str("roman").unwrap(),
                                               None, Some(U64(3)));
        assert_eq!(habits.len(), 3);

        let last_habit = &contract.get_habits_user(AccountId::from_str("roman").unwrap(),
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

        set_context("roman", 20 * NEAR, 1662312790000000000);
        contract.add_habit(
            "Do 15 push-ups everyday".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap(),
        );

        // Failed unlock from user side - on habit not approved
        set_context("roman", 0, 1663132260000000000);
        contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));

        // Failed unlock from beneficiary side - on too early
        set_context("josef", 0, 1663132260000000000);
        contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));

        // Success unlock from user side
        set_context("josef", 0, 1664302901000000000);
        contract.approve_habit(AccountId::from_str("roman").unwrap(), U64(0));
        set_context("roman", 0, 1665771701000000000);
        contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(0));

        // Success unlock from beneficiary side
        set_context("roman", 20 * NEAR, 1662312790000000000);
        contract.add_habit(
            "Eat vegetarian food once a day".to_string(),
            U64(0),
            AccountId::from_str("josef").unwrap(),
        );
        set_context("josef", 0, 1665771701000000000);
        contract.unlock_deposit(AccountId::from_str("roman").unwrap(),
                                               U64(1));
    }
}


