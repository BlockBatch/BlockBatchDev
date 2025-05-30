use crate::types::{Employee, PayrollError};

/// Validate employee data before adding to the contract
pub fn validate_employee_data(employee: &Employee) -> Result<(), PayrollError> {
    // Validate base salary
    if employee.base_salary <= 0 {
        return Err(PayrollError::InvalidAmount);
    }

    // Validate employee ID is not empty
    if employee.employee_id.is_empty() {
        return Err(PayrollError::InvalidEmployee);
    }

    // Validate tax jurisdiction is not empty
    if employee.tax_jurisdiction.is_empty() {
        return Err(PayrollError::InvalidEmployee);
    }

    Ok(())
}

/// Validate that an employee is active
pub fn validate_employee_active(employee: &Employee) -> Result<(), PayrollError> {
    if !employee.is_active {
        return Err(PayrollError::InvalidEmployee);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Employee;
    use soroban_sdk::{Address, Env, String, Vec};
    use soroban_sdk::testutils::Address as _;

    fn create_valid_employee(env: &Env) -> Employee {
        Employee {
            account_id: Address::generate(env),
            employee_id: String::from_str(env, "EMP_001"),
            base_salary: 100_000_0000000,
            tax_jurisdiction: String::from_str(env, "US"),
            benefits: Vec::new(env),
            payment_history: Vec::new(env),
            is_active: true,
        }
    }

    #[test]
    fn test_validate_employee_data() {
        let env = Env::default();
        let valid_employee = create_valid_employee(&env);

        // Valid employee should pass
        assert!(validate_employee_data(&valid_employee).is_ok());

        // Invalid salary
        let mut invalid_employee = valid_employee.clone();
        invalid_employee.base_salary = 0;
        assert_eq!(
            validate_employee_data(&invalid_employee),
            Err(PayrollError::InvalidAmount)
        );

        // Empty employee ID
        let mut invalid_employee = valid_employee.clone();
        invalid_employee.employee_id = String::from_str(&env, "");
        assert_eq!(
            validate_employee_data(&invalid_employee),
            Err(PayrollError::InvalidEmployee)
        );

        // Empty tax jurisdiction
        let mut invalid_employee = valid_employee.clone();
        invalid_employee.tax_jurisdiction = String::from_str(&env, "");
        assert_eq!(
            validate_employee_data(&invalid_employee),
            Err(PayrollError::InvalidEmployee)
        );
    }

    #[test]
    fn test_validate_employee_active() {
        let env = Env::default();
        let mut employee = create_valid_employee(&env);

        // Active employee should pass
        assert!(validate_employee_active(&employee).is_ok());

        // Inactive employee should fail
        employee.is_active = false;
        assert_eq!(
            validate_employee_active(&employee),
            Err(PayrollError::InvalidEmployee)
        );
    }
} 