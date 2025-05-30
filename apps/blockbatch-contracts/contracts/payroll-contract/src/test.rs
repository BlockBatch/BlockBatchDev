#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env, String, Vec,
};

fn create_test_env() -> Env {
    Env::default()
}

fn create_test_addresses(env: &Env) -> (Address, Address, Address, Address) {
    (
        Address::generate(env),
        Address::generate(env),
        Address::generate(env),
        Address::generate(env),
    )
}

fn create_test_asset(env: &Env) -> Asset {
    Asset {
        token: Address::generate(env),
        symbol: String::from_str(env, "USDC"),
        decimals: 7,
    }
}

fn create_test_payment_schedule(env: &Env) -> TimeSchedule {
    let current_time = env.ledger().timestamp();
    TimeSchedule {
        frequency: PaymentFrequency::Monthly,
        start_date: current_time + 86400, // Start tomorrow
        next_payment_date: current_time + 86400,
        end_date: None,
    }
}

fn create_test_employee(env: &Env, account_id: Address, employee_id: &str) -> Employee {
    Employee {
        account_id,
        employee_id: String::from_str(env, employee_id),
        base_salary: 100_000_0000000, // 100,000 USDC (7 decimals)
        tax_jurisdiction: String::from_str(env, "US"),
        benefits: Vec::new(env),
        payment_history: Vec::new(env),
        is_active: true,
    }
}

#[test]
fn test_create_payroll_contract() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, _employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000, // 1000 USDC minimum balance
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Test that contract status is active
    let status = client.get_payroll_status();
    assert_eq!(status, ContractStatus::Active);

    // Test that employees list is empty initially
    let employees = client.get_employees();
    assert_eq!(employees.len(), 0);
}

#[test]
fn test_add_employee() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Mock authorization for the employer
    env.mock_all_auths();

    // Add employee
    let employee = create_test_employee(&env, employee1.clone(), "EMP_001");
    client.add_employee(&employer, &employee);

    // Verify employee was added
    let employees = client.get_employees();
    assert_eq!(employees.len(), 1);
    assert_eq!(employees.get(0).unwrap().employee_id, employee.employee_id);
}

#[test]
fn test_remove_employee() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Mock authorization for the employer
    env.mock_all_auths();

    // Add employee
    let employee = create_test_employee(&env, employee1.clone(), "EMP_001");
    client.add_employee(&employer, &employee);

    // Remove employee
    client.remove_employee(&employer, &String::from_str(&env, "EMP_001"));

    // Verify employee was removed from active list
    let employees = client.get_employees();
    assert_eq!(employees.len(), 0);
}

#[test]
fn test_schedule_payment() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, _employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Mock authorization for the employer
    env.mock_all_auths();

    // Schedule payment
    let next_payment_date = client.schedule_payment(&employer);
    assert!(next_payment_date > env.ledger().timestamp());
}

#[test]
fn test_set_tax_rate() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, _employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Mock authorization for the employer
    env.mock_all_auths();

    // Set tax rate (25% = 2500 basis points)
    client.set_tax_rate(&employer, &String::from_str(&env, "US"), &2500);
}

#[test]
fn test_get_payment_history() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, _employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Get global payment history (should be empty initially)
    let history = client.get_payment_history(&None);
    assert_eq!(history.len(), 0);
}

#[test]
fn test_update_contract_status() {
    let env = create_test_env();
    let contract_id = env.register(PayrollContractImpl, ());
    let client = PayrollContractImplClient::new(&env, &contract_id);

    let (employer, _employee1, _employee2, _signer) = create_test_addresses(&env);
    let asset = create_test_asset(&env);
    let schedule = create_test_payment_schedule(&env);
    let signers = Vec::from_array(&env, [employer.clone()]);

    // Create contract
    client.create_payroll_contract(
        &employer,
        &schedule,
        &asset,
        &1000_0000000,
        &signers,
        &String::from_str(&env, "PAYROLL_001"),
    );

    // Mock authorization for the employer
    env.mock_all_auths();

    // Update status to paused
    client.update_contract_status(&employer, &ContractStatus::Paused);

    // Verify status was updated
    let status = client.get_payroll_status();
    assert_eq!(status, ContractStatus::Paused);
} 