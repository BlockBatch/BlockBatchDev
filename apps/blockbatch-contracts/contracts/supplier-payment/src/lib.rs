#![no_std]
use soroban_sdk::{
    contract, contractimpl, Address, Env, String, Vec,
};

mod types;
mod utils;
mod error;

use types::{
    Milestone, PurchaseOrder, Asset, DiscountTerms, Status, TimePoint,
    SupplierPaymentContract, ContractStatus,
};
use error::ContractError;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Creates a new supplier payment contract
    pub fn create_supplier_contract(
        env: Env,
        company_account: Address,
        supplier_account: Address,
        purchase_order: PurchaseOrder,
        payment_token: Asset,
        discount_terms: DiscountTerms,
        dispute_window: u32,
        required_signatures: u32,
    ) -> Result<SupplierPaymentContract, ContractError> {
        // Ensure at least one signature is required
        if required_signatures < 1 {
            return Err(ContractError::InvalidSignatureRequirement);
        }

        // Create the contract
        let contract = SupplierPaymentContract {
            company_account,
            supplier_account,
            purchase_order,
            milestones: Vec::new(&env),
            payment_token,
            discount_terms,
            dispute_window,
            required_signatures,
        };

        // Store the contract data
        utils::save_contract(&env, &contract);

        Ok(contract)
    }

    /// Adds a new milestone to the contract
    pub fn add_milestone(
        env: Env,
        caller: Address,
        description: String,
        amount: i128,
        due_date: TimePoint,
    ) -> Result<Milestone, ContractError> {
        // Get the contract
        let mut contract = utils::load_contract(&env)?;

        // Verify caller is the company account
        caller.require_auth();
        if caller != contract.company_account {
            return Err(ContractError::Unauthorized);
        }

        // Create a new milestone
        let milestone = Milestone {
            description,
            amount,
            due_date,
            completion_status: Status::Pending,
            verification_proof: String::from_str(&env, ""),
        };

        // Add the milestone to the contract
        contract.milestones.push_back(milestone.clone());

        // Save the updated contract
        utils::save_contract(&env, &contract);

        Ok(milestone)
    }

    /// Updates an existing milestone
    pub fn update_milestone(
        env: Env,
        caller: Address,
        index: u32,
        description: Option<String>,
        amount: Option<i128>,
        due_date: Option<TimePoint>,
    ) -> Result<Milestone, ContractError> {
        // Get the contract
        let mut contract = utils::load_contract(&env)?;

        // Verify caller is the company account
        caller.require_auth();
        if caller != contract.company_account {
            return Err(ContractError::Unauthorized);
        }

        // Ensure the milestone exists
        if index as usize >= contract.milestones.len() as usize {
            return Err(ContractError::MilestoneNotFound);
        }

        // Get the milestone
        let mut milestone = contract.milestones.get(index).unwrap();

        // Only allow updates to pending milestones
        if milestone.completion_status != Status::Pending {
            return Err(ContractError::CannotUpdateCompletedMilestone);
        }

        // Update the milestone fields if provided
        if let Some(desc) = description {
            milestone.description = desc;
        }

        if let Some(amt) = amount {
            milestone.amount = amt;
        }

        if let Some(date) = due_date {
            milestone.due_date = date;
        }

        // Update the milestone in the contract
        contract.milestones.set(index, milestone.clone());

        // Save the updated contract
        utils::save_contract(&env, &contract);

        Ok(milestone)
    }

    /// Verifies a milestone has been completed
    pub fn verify_milestone(
        env: Env,
        caller: Address,
        index: u32,
        verification_proof: String,
    ) -> Result<Milestone, ContractError> {
        // Get the contract
        let mut contract = utils::load_contract(&env)?;

        // Verify caller is the supplier account
        caller.require_auth();
        if caller != contract.supplier_account {
            return Err(ContractError::Unauthorized);
        }

        // Ensure the milestone exists
        if index as usize >= contract.milestones.len() as usize {
            return Err(ContractError::MilestoneNotFound);
        }

        // Get the milestone
        let mut milestone = contract.milestones.get(index).unwrap();

        // Only allow verification for pending milestones
        if milestone.completion_status != Status::Pending {
            return Err(ContractError::MilestoneAlreadyProcessed);
        }

        // Mark the milestone as completed
        milestone.completion_status = Status::Completed;
        milestone.verification_proof = verification_proof;

        // Update the milestone in the contract
        contract.milestones.set(index, milestone.clone());

        // Save the updated contract
        utils::save_contract(&env, &contract);

        Ok(milestone)
    }

    /// Process payment for a verified milestone
    pub fn process_milestone_payment(
        env: Env,
        caller: Address,
        index: u32,
    ) -> Result<Milestone, ContractError> {
        // Get the contract
        let mut contract = utils::load_contract(&env)?;

        // Verify caller is the company account
        caller.require_auth();
        if caller != contract.company_account {
            return Err(ContractError::Unauthorized);
        }

        // Ensure the milestone exists
        if index as usize >= contract.milestones.len() as usize {
            return Err(ContractError::MilestoneNotFound);
        }

        // Get the milestone
        let mut milestone = contract.milestones.get(index).unwrap();

        // Ensure the milestone is completed but not paid
        if milestone.completion_status != Status::Completed {
            return Err(ContractError::MilestoneNotCompleted);
        }

        // Calculate payment amount with any discount
        let payment_amount = Self::calculate_early_payment_discount(
            &env,
            contract.discount_terms.clone(),
            milestone.amount,
        );

        // Transfer tokens from company to supplier
        utils::transfer_token(
            &env,
            &contract.payment_token,
            &contract.company_account,
            &contract.supplier_account,
            payment_amount,
        )?;

        // Mark the milestone as paid
        milestone.completion_status = Status::Paid;

        // Update the milestone in the contract
        contract.milestones.set(index, milestone.clone());

        // Save the updated contract
        utils::save_contract(&env, &contract);

        Ok(milestone)
    }

    /// Calculate early payment discount
    pub fn calculate_early_payment_discount(
        env: &Env,
        discount_terms: DiscountTerms,
        amount: i128,
    ) -> i128 {
        let current_time = env.ledger().timestamp();

        // Check if payment is within early payment window
        if let Some(window) = discount_terms.early_payment_window {
            if current_time <= window {
                // Apply discount
                let discount_amount = (amount)
                    .checked_mul(discount_terms.discount_percentage as i128)
                    .unwrap_or(0)
                    .checked_div(100)
                    .unwrap_or(0);
                
                return amount.checked_sub(discount_amount).unwrap_or(amount);
            }
        }

        // No discount applies
        amount
    }

    /// Get the current status of the supplier contract
    pub fn get_supplier_contract_status(
        env: Env,
    ) -> Result<ContractStatus, ContractError> {
        let contract = utils::load_contract(&env)?;
        
        // Calculate total and completed/paid milestones
        let total_milestones = contract.milestones.len();
        let mut completed = 0;
        let mut paid = 0;
        
        for i in 0..total_milestones {
            let milestone = contract.milestones.get(i).unwrap();
            match milestone.completion_status {
                Status::Completed => completed += 1,
                Status::Paid => {
                    completed += 1;
                    paid += 1;
                },
                _ => {}
            }
        }
        
        // Determine overall contract status
        let status = if total_milestones == 0 {
            ContractStatus::Draft
        } else if paid == total_milestones {
            ContractStatus::Completed
        } else if completed > 0 {
            ContractStatus::InProgress
        } else {
            ContractStatus::Active
        };
        
        Ok(status)
    }

    /// Get all milestones in the contract
    pub fn get_milestones(
        env: Env,
    ) -> Result<Vec<Milestone>, ContractError> {
        let contract = utils::load_contract(&env)?;
        Ok(contract.milestones)
    }

    /// Initiate a payment dispute
    pub fn initiate_dispute(
        env: Env,
        caller: Address,
        index: u32,
        dispute_reason: String,
    ) -> Result<(), ContractError> {
        // Get the contract
        let mut contract = utils::load_contract(&env)?;

        // Verify caller is either company or supplier
        caller.require_auth();
        if caller != contract.company_account && caller != contract.supplier_account {
            return Err(ContractError::Unauthorized);
        }

        // Ensure the milestone exists
        if index as usize >= contract.milestones.len() as usize {
            return Err(ContractError::MilestoneNotFound);
        }

        // Get the milestone
        let mut milestone = contract.milestones.get(index).unwrap();

        // Can only dispute completed but not paid milestones
        if milestone.completion_status != Status::Completed {
            return Err(ContractError::CannotDisputeMilestone);
        }

        // Mark the milestone as disputed
        milestone.completion_status = Status::Disputed;

        // Store dispute information
        utils::store_dispute(&env, index as u64, caller, dispute_reason);

        // Update the milestone in the contract
        contract.milestones.set(index, milestone);

        // Save the updated contract
        utils::save_contract(&env, &contract);

        Ok(())
    }
}

#[cfg(test)]
mod test; 