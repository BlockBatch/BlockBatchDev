use crate::types::{DataKey, Employee, PayrollError};
use soroban_sdk::{Env, String};

/// Check if an employee exists in storage
pub fn employee_exists(env: &Env, employee_id: &String) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Employee(employee_id.clone()))
}

/// Get an employee from storage
pub fn get_employee(env: &Env, employee_id: &String) -> Result<Employee, PayrollError> {
    env.storage()
        .persistent()
        .get(&DataKey::Employee(employee_id.clone()))
        .ok_or(PayrollError::EmployeeNotFound)
}

/// Store an employee
pub fn store_employee(env: &Env, employee: &Employee) {
    env.storage().persistent().set(
        &DataKey::Employee(employee.employee_id.clone()),
        employee,
    );
}

/// Update an employee's data
pub fn update_employee(env: &Env, employee: &Employee) -> Result<(), PayrollError> {
    if !employee_exists(env, &employee.employee_id) {
        return Err(PayrollError::EmployeeNotFound);
    }
    store_employee(env, employee);
    Ok(())
}

/// Mark an employee as inactive (soft delete)
pub fn deactivate_employee(env: &Env, employee_id: &String) -> Result<(), PayrollError> {
    let mut employee = get_employee(env, employee_id)?;
    employee.is_active = false;
    store_employee(env, &employee);
    Ok(())
}

/// Get all active employees from a contract's employee list
pub fn filter_active_employees(employees: &soroban_sdk::Vec<Employee>) -> soroban_sdk::Vec<Employee> {
    let mut active_employees = soroban_sdk::Vec::new(employees.env());
    for employee in employees.iter() {
        if employee.is_active {
            active_employees.push_back(employee);
        }
    }
    active_employees
} 