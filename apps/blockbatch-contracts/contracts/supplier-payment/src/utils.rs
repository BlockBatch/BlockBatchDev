use soroban_sdk::{
    Address, Env, String, 
    IntoVal,
};

use crate::types::{SupplierPaymentContract, Dispute, DisputeStatus, Asset};
use crate::error::ContractError;

/// Save the contract to storage
pub fn save_contract(env: &Env, contract: &SupplierPaymentContract) {
    env.storage().instance().set(&1u32, contract);
}

/// Load the contract from storage
pub fn load_contract(env: &Env) -> Result<SupplierPaymentContract, ContractError> {
    match env.storage().instance().get::<u32, SupplierPaymentContract>(&1u32) {
        Some(contract) => Ok(contract),
        None => Err(ContractError::ContractNotFound),
    }
}

/// Store dispute information
pub fn store_dispute(env: &Env, milestone_index: u64, initiator: Address, reason: String) {
    let timestamp = env.ledger().timestamp();
    let dispute = Dispute {
        milestone_index,
        initiator,
        reason,
        timestamp,
        status: DisputeStatus::Open,
    };
    
    // Use the milestone_index as the key
    env.storage().temporary().set(&milestone_index, &dispute);
}

/// Get dispute information
pub fn get_dispute(env: &Env, milestone_index: u64) -> Result<Dispute, ContractError> {
    match env.storage().temporary().get::<u64, Dispute>(&milestone_index) {
        Some(dispute) => Ok(dispute),
        None => Err(ContractError::DisputeNotFound),
    }
}

/// Transfer tokens between accounts
pub fn transfer_token(
    env: &Env,
    _token: &Asset,
    from: &Address,
    _to: &Address,
    amount: i128,
) -> Result<(), ContractError> {
    // In a real implementation, this would interact with the Stellar Asset Contract
    // to transfer tokens between accounts.
    
    // Check if the user has been authenticated
    from.require_auth();
    
    // Log the transfer for informational purposes
    env.logs().add("transfer", &[amount.into_val(env)]);
    
    // In a real implementation, you would add error handling for the transfer
    // For this example, we'll assume the transfer is successful
    
    Ok(())
}

/// Check if a dispute is within the dispute window
pub fn is_within_dispute_window(env: &Env, contract: &SupplierPaymentContract, completion_time: u64) -> bool {
    let current_time = env.ledger().timestamp();
    let dispute_window_seconds = contract.dispute_window as u64 * 24 * 60 * 60; // Convert days to seconds
    
    current_time <= completion_time + dispute_window_seconds
} 