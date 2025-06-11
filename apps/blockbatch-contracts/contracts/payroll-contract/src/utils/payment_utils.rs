use soroban_sdk::{Env, String};

/// Generate a unique payment ID
pub fn generate_payment_id(env: &Env, _counter: u64) -> String {
    // Simple payment ID for now - in production, this could be more sophisticated
    String::from_str(env, "PAY")
}

/// Generate a transaction hash
pub fn generate_transaction_hash(env: &Env, _counter: u64) -> String {
    // Simple transaction hash for now - in production, this would be the actual tx hash
    String::from_str(env, "TX")
}

/// Validate payment amount
pub fn validate_payment_amount(amount: i128) -> bool {
    amount > 0
}

/// Validate tax rate (should be between 0 and 100%)
pub fn validate_tax_rate(tax_rate: i128) -> bool {
    tax_rate >= 0 && tax_rate <= 10000 // 0-100% in basis points
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_validate_payment_amount() {
        assert!(validate_payment_amount(1000));
        assert!(!validate_payment_amount(0));
        assert!(!validate_payment_amount(-100));
    }

    #[test]
    fn test_validate_tax_rate() {
        assert!(validate_tax_rate(0));      // 0%
        assert!(validate_tax_rate(2500));   // 25%
        assert!(validate_tax_rate(10000));  // 100%
        assert!(!validate_tax_rate(-100));  // Invalid
        assert!(!validate_tax_rate(15000)); // Over 100%
    }

    #[test]
    fn test_generate_payment_id() {
        let env = Env::default();
        let payment_id = generate_payment_id(&env, 1);
        assert_eq!(payment_id, String::from_str(&env, "PAY"));
    }
} 