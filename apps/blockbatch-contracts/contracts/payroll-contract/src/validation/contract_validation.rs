use crate::types::{ContractStatus, PayrollContract, PayrollError, TimeSchedule};
use soroban_sdk::{Address, Env, Vec};

/// Validate contract initialization parameters
pub fn validate_contract_creation(
    env: &Env,
    minimum_balance: i128,
    signers: &Vec<Address>,
    payment_schedule: &TimeSchedule,
) -> Result<(), PayrollError> {
    // Validate minimum balance
    if minimum_balance <= 0 {
        return Err(PayrollError::InvalidAmount);
    }

    // Validate signers
    if signers.is_empty() {
        return Err(PayrollError::InvalidSigner);
    }

    // Validate payment schedule
    let current_time = env.ledger().timestamp();
    if payment_schedule.start_date < current_time {
        return Err(PayrollError::InvalidPaymentSchedule);
    }

    Ok(())
}

/// Check if a caller is authorized to perform operations on the contract
pub fn is_authorized(contract: &PayrollContract, caller: &Address) -> bool {
    caller == &contract.employer_account || contract.signers.contains(caller)
}

/// Validate that the contract is in an active state for operations
pub fn validate_contract_active(contract: &PayrollContract) -> Result<(), PayrollError> {
    if contract.status != ContractStatus::Active {
        return Err(PayrollError::ContractNotActive);
    }
    Ok(())
}

/// Validate authorization for a specific caller
pub fn validate_authorization(
    contract: &PayrollContract,
    caller: &Address,
) -> Result<(), PayrollError> {
    if !is_authorized(contract, caller) {
        return Err(PayrollError::Unauthorized);
    }
    Ok(())
}

/// Validate that the caller is the employer (for employer-only operations)
pub fn validate_employer_only(
    contract: &PayrollContract,
    caller: &Address,
) -> Result<(), PayrollError> {
    if caller != &contract.employer_account {
        return Err(PayrollError::Unauthorized);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Asset, PaymentFrequency};
    use soroban_sdk::{Env, Map, String};
    use soroban_sdk::testutils::Address as _;

    fn create_test_contract(env: &Env) -> PayrollContract {
        let employer = Address::generate(env);
        let signers = Vec::from_array(env, [employer.clone()]);
        
        PayrollContract {
            employer_account: employer,
            payment_schedule: TimeSchedule {
                frequency: PaymentFrequency::Monthly,
                start_date: env.ledger().timestamp() + 86400,
                next_payment_date: env.ledger().timestamp() + 86400,
                end_date: None,
            },
            employees: Vec::new(env),
            tax_rates: Map::new(env),
            payment_token: Asset {
                token: Address::generate(env),
                symbol: String::from_str(env, "USDC"),
                decimals: 7,
            },
            minimum_balance: 1000_0000000,
            signers,
            status: ContractStatus::Active,
            contract_id: String::from_str(env, "TEST_001"),
            created_at: env.ledger().timestamp(),
        }
    }

    #[test]
    fn test_validate_contract_creation() {
        let env = Env::default();
        let signers = Vec::from_array(&env, [Address::generate(&env)]);
        let schedule = TimeSchedule {
            frequency: PaymentFrequency::Monthly,
            start_date: env.ledger().timestamp() + 86400,
            next_payment_date: env.ledger().timestamp() + 86400,
            end_date: None,
        };

        // Valid parameters
        assert!(validate_contract_creation(&env, 1000, &signers, &schedule).is_ok());

        // Invalid minimum balance
        assert_eq!(
            validate_contract_creation(&env, 0, &signers, &schedule),
            Err(PayrollError::InvalidAmount)
        );

        // Empty signers
        let empty_signers = Vec::new(&env);
        assert_eq!(
            validate_contract_creation(&env, 1000, &empty_signers, &schedule),
            Err(PayrollError::InvalidSigner)
        );
    }

    #[test]
    fn test_is_authorized() {
        let env = Env::default();
        let contract = create_test_contract(&env);
        let unauthorized = Address::generate(&env);

        // Employer should be authorized
        assert!(is_authorized(&contract, &contract.employer_account));

        // Signer should be authorized
        assert!(is_authorized(&contract, &contract.signers.get(0).unwrap()));

        // Random address should not be authorized
        assert!(!is_authorized(&contract, &unauthorized));
    }
} 