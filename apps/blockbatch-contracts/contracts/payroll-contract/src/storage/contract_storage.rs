use crate::types::{DataKey, PayrollContract, PayrollError, Payment};
use soroban_sdk::{Env, Vec};

/// Get the payroll contract from storage
pub fn get_contract(env: &Env) -> Result<PayrollContract, PayrollError> {
    env.storage()
        .persistent()
        .get(&DataKey::Contract)
        .ok_or(PayrollError::NotInitialized)
}

/// Store the payroll contract
pub fn store_contract(env: &Env, contract: &PayrollContract) {
    env.storage()
        .persistent()
        .set(&DataKey::Contract, contract);
}

/// Get the next payment ID counter
pub fn get_next_payment_id(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::NextPaymentId)
        .unwrap_or(0)
}

/// Increment and store the payment ID counter
pub fn increment_payment_id(env: &Env) -> u64 {
    let current_id = get_next_payment_id(env);
    let next_id = current_id + 1;
    env.storage()
        .persistent()
        .set(&DataKey::NextPaymentId, &next_id);
    current_id
}

/// Get the total number of employees
pub fn get_total_employees(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalEmployees)
        .unwrap_or(0)
}

/// Update the total number of employees
pub fn set_total_employees(env: &Env, count: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::TotalEmployees, &count);
}

/// Get global payment history
pub fn get_payment_history(env: &Env) -> Vec<Payment> {
    env.storage()
        .persistent()
        .get(&DataKey::PaymentHistory)
        .unwrap_or_else(|| Vec::new(env))
}

/// Store global payment history
pub fn store_payment_history(env: &Env, history: &Vec<Payment>) {
    env.storage()
        .persistent()
        .set(&DataKey::PaymentHistory, history);
}

/// Initialize contract storage with default values
pub fn initialize_storage(env: &Env) {
    env.storage().persistent().set(&DataKey::NextPaymentId, &0u64);
    env.storage().persistent().set(&DataKey::TotalEmployees, &0u32);
    env.storage()
        .persistent()
        .set(&DataKey::PaymentHistory, &Vec::<Payment>::new(env));
} 