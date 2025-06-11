use crate::storage::{
    get_contract, store_contract, increment_payment_id, get_total_employees, 
    set_total_employees, get_payment_history, store_payment_history, initialize_storage,
    employee_exists, store_employee, deactivate_employee, get_employee
};
use crate::utils::{
    calculate_gross_payment, calculate_tax_amount, calculate_net_amount, 
    calculate_next_payment_date, generate_payment_id, generate_transaction_hash
};
use crate::validation::{
    validate_contract_creation, validate_authorization, validate_contract_active,
    validate_employer_only, validate_employee_data, validate_payment_due,
    validate_sufficient_balance, validate_tax_rate
};
use crate::types::{
    Asset, ContractStatus, Employee, Payment, PaymentStatus,
    PayrollContract, PayrollError, TimeSchedule,
};
use soroban_sdk::{
    contract, contractimpl, symbol_short, token::Client, Address, Env, String, Vec,
};

#[contract]
pub struct PayrollContractImpl;

#[contractimpl]
impl PayrollContractImpl {
    /// Initialize a new payroll contract
    pub fn create_payroll_contract(
        env: Env,
        employer_account: Address,
        payment_schedule: TimeSchedule,
        payment_token: Asset,
        minimum_balance: i128,
        signers: Vec<Address>,
        contract_id: String,
    ) -> Result<(), PayrollError> {
        // Check if contract is already initialized
        if env.storage().persistent().has(&crate::types::DataKey::Contract) {
            return Err(PayrollError::NotInitialized);
        }

        // Validate contract creation parameters
        validate_contract_creation(&env, minimum_balance, &signers, &payment_schedule)?;

        // Create the payroll contract
        let payroll_contract = PayrollContract {
            employer_account: employer_account.clone(),
            payment_schedule,
            employees: Vec::new(&env),
            tax_rates: soroban_sdk::Map::new(&env),
            payment_token,
            minimum_balance,
            signers: signers.clone(),
            status: ContractStatus::Active,
            contract_id: contract_id.clone(),
            created_at: env.ledger().timestamp(),
        };

        // Store the contract and initialize storage
        store_contract(&env, &payroll_contract);
        initialize_storage(&env);

        // Emit event
        env.events().publish(
            (symbol_short!("created"), employer_account, contract_id),
            signers.len(),
        );

        Ok(())
    }

    /// Add a new employee to the payroll
    pub fn add_employee(
        env: Env,
        caller: Address,
        employee: Employee,
    ) -> Result<(), PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate authorization and contract state
        validate_authorization(&contract, &caller)?;
        validate_contract_active(&contract)?;

        // Check if employee already exists
        if employee_exists(&env, &employee.employee_id) {
            return Err(PayrollError::EmployeeAlreadyExists);
        }

        // Validate employee data
        validate_employee_data(&employee)?;

        // Store employee
        store_employee(&env, &employee);

        // Add employee to contract's employee list
        contract.employees.push_back(employee.clone());
        store_contract(&env, &contract);

        // Update total employees counter
        let total_employees = get_total_employees(&env);
        set_total_employees(&env, total_employees + 1);

        // Emit event
        env.events().publish(
            (symbol_short!("emp_add"), employee.employee_id.clone()),
            employee.base_salary,
        );

        Ok(())
    }

    /// Remove an employee from the payroll
    pub fn remove_employee(
        env: Env,
        caller: Address,
        employee_id: String,
    ) -> Result<(), PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate authorization and contract state
        validate_authorization(&contract, &caller)?;
        validate_contract_active(&contract)?;

        // Check if employee exists
        if !employee_exists(&env, &employee_id) {
            return Err(PayrollError::EmployeeNotFound);
        }

        // Deactivate employee
        deactivate_employee(&env, &employee_id)?;

        // Remove from contract's employee list
        let mut new_employees = Vec::new(&env);
        for emp in contract.employees.iter() {
            if emp.employee_id != employee_id {
                new_employees.push_back(emp);
            }
        }
        contract.employees = new_employees;
        store_contract(&env, &contract);

        // Update total employees counter
        let total_employees = get_total_employees(&env);
        if total_employees > 0 {
            set_total_employees(&env, total_employees - 1);
        }

        // Emit event
        env.events()
            .publish((symbol_short!("emp_rem"), employee_id), 0);

        Ok(())
    }

    /// Schedule the next payment based on the payment frequency
    pub fn schedule_payment(env: Env, caller: Address) -> Result<u64, PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate authorization and contract state
        validate_authorization(&contract, &caller)?;
        validate_contract_active(&contract)?;

        let next_payment_date = calculate_next_payment_date(
            &contract.payment_schedule.frequency,
            contract.payment_schedule.next_payment_date,
        );

        // Update the next payment date
        contract.payment_schedule.next_payment_date = next_payment_date;
        store_contract(&env, &contract);

        // Emit event
        env.events().publish(
            (symbol_short!("scheduled"), next_payment_date),
            contract.employees.len(),
        );

        Ok(next_payment_date)
    }

    /// Execute payments for all active employees
    pub fn execute_payment(env: Env, caller: Address) -> Result<Vec<String>, PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate authorization and contract state
        validate_authorization(&contract, &caller)?;
        validate_contract_active(&contract)?;

        let current_time = env.ledger().timestamp();

        // Check if payment is due
        validate_payment_due(&env, contract.payment_schedule.next_payment_date)?;

        let mut payment_ids = Vec::new(&env);
        let mut global_payment_history = get_payment_history(&env);

        // Process payments for each active employee
        for employee in contract.employees.iter() {
            if !employee.is_active {
                continue;
            }

            let payment_result = Self::process_employee_payment(&env, &employee, &contract);
            match payment_result {
                Ok(payment) => {
                    payment_ids.push_back(payment.payment_id.clone());
                    global_payment_history.push_back(payment.clone());

                    // Update employee's payment history
                    let mut updated_employee = employee.clone();
                    updated_employee.payment_history.push_back(payment);
                    store_employee(&env, &updated_employee);
                }
                Err(_) => {
                    // Log failed payment but continue with others
                    env.events().publish(
                        (symbol_short!("pay_fail"), employee.employee_id.clone()),
                        0,
                    );
                }
            }
        }

        // Update global payment history
        store_payment_history(&env, &global_payment_history);

        // Schedule next payment
        contract.payment_schedule.next_payment_date = calculate_next_payment_date(
            &contract.payment_schedule.frequency,
            current_time,
        );
        store_contract(&env, &contract);

        // Emit event
        env.events().publish(
            (symbol_short!("executed"), current_time),
            payment_ids.len(),
        );

        Ok(payment_ids)
    }

    /// Get the current status of the payroll contract
    pub fn get_payroll_status(env: Env) -> Result<ContractStatus, PayrollError> {
        let contract = get_contract(&env)?;
        Ok(contract.status)
    }

    /// Get all employees in the payroll
    pub fn get_employees(env: Env) -> Result<Vec<Employee>, PayrollError> {
        let contract = get_contract(&env)?;
        Ok(contract.employees)
    }

    /// Get payment history for all employees or a specific employee
    pub fn get_payment_history(
        env: Env,
        employee_id: Option<String>,
    ) -> Result<Vec<Payment>, PayrollError> {
        match employee_id {
            Some(id) => {
                // Get payment history for specific employee
                let employee = get_employee(&env, &id)?;
                Ok(employee.payment_history)
            }
            None => {
                // Get global payment history
                Ok(get_payment_history(&env))
            }
        }
    }

    /// Update contract status (admin only)
    pub fn update_contract_status(
        env: Env,
        caller: Address,
        new_status: ContractStatus,
    ) -> Result<(), PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate employer-only access
        validate_employer_only(&contract, &caller)?;

        contract.status = new_status.clone();
        store_contract(&env, &contract);

        // Emit event
        env.events()
            .publish((symbol_short!("status"), caller), new_status as u32);

        Ok(())
    }

    /// Add or update tax rate for a jurisdiction
    pub fn set_tax_rate(
        env: Env,
        caller: Address,
        jurisdiction: String,
        tax_rate: i128, // Basis points (10000 = 100%)
    ) -> Result<(), PayrollError> {
        caller.require_auth();

        let mut contract = get_contract(&env)?;

        // Validate authorization and tax rate
        validate_authorization(&contract, &caller)?;
        validate_tax_rate(tax_rate)?;

        contract.tax_rates.set(jurisdiction.clone(), tax_rate);
        store_contract(&env, &contract);

        // Emit event
        env.events()
            .publish((symbol_short!("tax_set"), jurisdiction), tax_rate);

        Ok(())
    }

    // Private helper methods

    fn process_employee_payment(
        env: &Env,
        employee: &Employee,
        contract: &PayrollContract,
    ) -> Result<Payment, PayrollError> {
        // Calculate payment amounts
        let gross_amount = calculate_gross_payment(&contract.payment_schedule.frequency, employee.base_salary);
        let tax_rate = contract
            .tax_rates
            .get(employee.tax_jurisdiction.clone())
            .unwrap_or(0);
        let tax_amount = calculate_tax_amount(gross_amount, tax_rate);
        let net_amount = calculate_net_amount(gross_amount, tax_amount);

        // Generate payment ID
        let payment_id_counter = increment_payment_id(env);
        let payment_id = generate_payment_id(env, payment_id_counter);

        // Check contract balance
        let token_client = Client::new(env, &contract.payment_token.token);
        let contract_balance = token_client.balance(&env.current_contract_address());
        
        validate_sufficient_balance(contract_balance, net_amount)?;

        // Execute the payment
        let _payment_result = token_client.transfer(
            &env.current_contract_address(),
            &employee.account_id,
            &net_amount,
        );

        // Create transaction hash
        let tx_hash = generate_transaction_hash(env, payment_id_counter);

        let payment = Payment {
            payment_id: payment_id.clone(),
            employee_id: employee.employee_id.clone(),
            gross_amount,
            tax_amount,
            net_amount,
            payment_date: env.ledger().timestamp(),
            status: PaymentStatus::Completed,
            transaction_hash: Some(tx_hash),
        };

        // Emit payment event
        env.events().publish(
            (symbol_short!("payment"), employee.employee_id.clone(), payment_id),
            net_amount,
        );

        Ok(payment)
    }
} 