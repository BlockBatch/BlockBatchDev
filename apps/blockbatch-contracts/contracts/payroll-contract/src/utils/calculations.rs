use crate::types::PaymentFrequency;

/// Calculate the gross payment amount based on frequency and annual salary
pub fn calculate_gross_payment(frequency: &PaymentFrequency, annual_salary: i128) -> i128 {
    match frequency {
        PaymentFrequency::Weekly => annual_salary / 52,
        PaymentFrequency::BiWeekly => annual_salary / 26,
        PaymentFrequency::Monthly => annual_salary / 12,
        PaymentFrequency::Quarterly => annual_salary / 4,
        PaymentFrequency::Annually => annual_salary,
    }
}

/// Calculate tax amount based on gross amount and tax rate (in basis points)
pub fn calculate_tax_amount(gross_amount: i128, tax_rate: i128) -> i128 {
    (gross_amount * tax_rate) / 10000 // Convert basis points to actual amount
}

/// Calculate net payment amount after tax deduction
pub fn calculate_net_amount(gross_amount: i128, tax_amount: i128) -> i128 {
    gross_amount - tax_amount
}

/// Calculate the next payment date based on frequency
pub fn calculate_next_payment_date(frequency: &PaymentFrequency, current_date: u64) -> u64 {
    match frequency {
        PaymentFrequency::Weekly => current_date + (7 * 24 * 60 * 60), // 7 days
        PaymentFrequency::BiWeekly => current_date + (14 * 24 * 60 * 60), // 14 days
        PaymentFrequency::Monthly => current_date + (30 * 24 * 60 * 60), // 30 days (approximation)
        PaymentFrequency::Quarterly => current_date + (90 * 24 * 60 * 60), // 90 days
        PaymentFrequency::Annually => current_date + (365 * 24 * 60 * 60), // 365 days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gross_payment() {
        let annual_salary = 120_000_0000000; // 120,000 with 7 decimals
        
        assert_eq!(calculate_gross_payment(&PaymentFrequency::Monthly, annual_salary), 10_000_0000000);
        assert_eq!(calculate_gross_payment(&PaymentFrequency::Weekly, annual_salary), annual_salary / 52);
        assert_eq!(calculate_gross_payment(&PaymentFrequency::Annually, annual_salary), annual_salary);
    }

    #[test]
    fn test_calculate_tax_amount() {
        let gross_amount = 10_000_0000000; // 10,000 with 7 decimals
        let tax_rate = 2500; // 25% in basis points
        
        let expected_tax = 2_500_0000000; // 25% of 10,000
        assert_eq!(calculate_tax_amount(gross_amount, tax_rate), expected_tax);
    }

    #[test]
    fn test_calculate_net_amount() {
        let gross_amount = 10_000_0000000;
        let tax_amount = 2_500_0000000;
        let expected_net = 7_500_0000000;
        
        assert_eq!(calculate_net_amount(gross_amount, tax_amount), expected_net);
    }
} 