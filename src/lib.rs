#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec, panic_with_error};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
}

#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

// Define a CustomError enum with explicit integer literals for all variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CustomError {
    TooManyClaimants = 1,
    AlreadyInitialized = 2,
    TimePredicateNotFulfilled = 3,
    ClaimantNotAllowed = 4,
}

// Implement the From<CustomError> trait for soroban_sdk::Error.
impl From<CustomError> for soroban_sdk::Error {
    fn from(error: CustomError) -> Self {
        soroban_sdk::Error::from_contract_error(error as u32)
    }
}

#[contract]
pub struct ClaimableBalanceContract;

// Check that provided timestamp is before/after the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic_with_error!(env, CustomError::TooManyClaimants);
        }
        if is_initialized(&env) {
            panic_with_error!(env, CustomError::AlreadyInitialized);
        }
        from.require_auth();

        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        env.storage().instance().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address) {
        claimant.require_auth();
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic_with_error!(env, CustomError::TimePredicateNotFulfilled);
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic_with_error!(env, CustomError::ClaimantNotAllowed);
        }

        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );
        env.storage().instance().remove(&DataKey::Balance);
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

mod test;
