#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use crate::types::{
    Asset, DiscountTerms, PurchaseOrder, TimePoint,
};

#[test]
fn test_create_supplier_contract() {
    let env = Env::default();
    env.mock_all_auths();

    // Register the contract
    let contract_id = env.register_contract(None, Contract);

    let company = Address::generate(&env);
    let supplier = Address::generate(&env);
    
    // Create purchase order
    let po = PurchaseOrder {
        po_number: String::from_str(&env, "PO123456"),
        description: String::from_str(&env, "Office supplies"),
        total_amount: 5000,
        issue_date: TimePoint { timestamp: env.ledger().timestamp() },
    };

    // Create payment token
    let token = Asset {
        code: String::from_str(&env, "USDC"),
        issuer: Address::generate(&env),
    };

    // Create discount terms
    let discount_terms = DiscountTerms {
        discount_percentage: 2,
        early_payment_window: Some(env.ledger().timestamp() + 86400 * 14), // 14 days
    };

    // Create the contract - wrapping with as_contract
    let contract = env.as_contract(&contract_id, || {
        Contract::create_supplier_contract(
            env.clone(),
            company.clone(),
            supplier.clone(),
            po,
            token,
            discount_terms,
            30, // 30 day dispute window
            1,  // 1 signature required
        )
    }).unwrap();

    // Verify the contract was created with correct values
    assert_eq!(contract.company_account, company);
    assert_eq!(contract.supplier_account, supplier);
    assert_eq!(contract.purchase_order.po_number, String::from_str(&env, "PO123456"));
} 