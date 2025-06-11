use crate::types::PayrollError;
use soroban_sdk::Env;

/// Validate that a payment is due based on the schedule
pub fn validate_payment_due(env: &Env, next_payment_date: u64) -> Result<(), PayrollError> {
    let current_time = env.ledger().timestamp();
    if current_time < next_payment_date {
        return Err(PayrollError::PaymentNotDue);
    }
    Ok(())
}

/// Validate that the contract has sufficient balance for a payment
pub fn validate_sufficient_balance(
    contract_balance: i128,
    payment_amount: i128,
) -> Result<(), PayrollError> {
    if contract_balance < payment_amount {
        return Err(PayrollError::InsufficientBalance);
    }
    Ok(())
}

/// Validate tax rate (should be between 0 and 100%)
pub fn validate_tax_rate(tax_rate: i128) -> Result<(), PayrollError> {
    if tax_rate < 0 || tax_rate > 10000 {
        return Err(PayrollError::InvalidTaxRate);
    }
    Ok(())
}

/// Validate payment amount
pub fn validate_payment_amount(amount: i128) -> Result<(), PayrollError> {
    if amount <= 0 {
        return Err(PayrollError::InvalidAmount);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_validate_payment_due() {
        let env = Env::default();
        let current_time = env.ledger().timestamp();

        // Payment due (current time)
        assert!(validate_payment_due(&env, current_time).is_ok());

        // Payment not due (future date)
        assert_eq!(
            validate_payment_due(&env, current_time + 1000),
            Err(PayrollError::PaymentNotDue)
        );
    }

    #[test]
    fn test_validate_sufficient_balance() {
        // Sufficient balance
        assert!(validate_sufficient_balance(1000, 500).is_ok());

        // Insufficient balance
        assert_eq!(
            validate_sufficient_balance(500, 1000),
            Err(PayrollError::InsufficientBalance)
        );

        // Exact balance
        assert!(validate_sufficient_balance(1000, 1000).is_ok());
    }

    #[test]
    fn test_validate_tax_rate() {
        // Valid tax rates
        assert!(validate_tax_rate(0).is_ok());      // 0%
        assert!(validate_tax_rate(2500).is_ok());   // 25%
        assert!(validate_tax_rate(10000).is_ok());  // 100%

        // Invalid tax rates
        assert_eq!(validate_tax_rate(-100), Err(PayrollError::InvalidTaxRate));
        assert_eq!(validate_tax_rate(15000), Err(PayrollError::InvalidTaxRate));
    }

    #[test]
    fn test_validate_payment_amount() {
        // Valid amounts
        assert!(validate_payment_amount(1).is_ok());
        assert!(validate_payment_amount(1000000).is_ok());

        // Invalid amounts
        assert_eq!(validate_payment_amount(0), Err(PayrollError::InvalidAmount));
        assert_eq!(validate_payment_amount(-100), Err(PayrollError::InvalidAmount));
    }
} 