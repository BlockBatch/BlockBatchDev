use soroban_sdk::{contracttype, Address, String, Vec};

/// Asset representation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub code: String,
    pub issuer: Address,
}

/// Purchase order information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchaseOrder {
    pub po_number: String,
    pub description: String,
    pub total_amount: i128,
    pub issue_date: TimePoint,
}

/// Discount terms for early payment
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscountTerms {
    pub discount_percentage: u32,
    pub early_payment_window: Option<u64>,
}

/// Milestone completion status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Pending,
    Completed,
    Paid,
    Disputed,
}

/// Time point representation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePoint {
    pub timestamp: u64,
}

/// Milestone in the supplier payment contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub description: String,
    pub amount: i128,
    pub due_date: TimePoint,
    pub completion_status: Status,
    pub verification_proof: String,
}

/// Supplier payment contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupplierPaymentContract {
    pub company_account: Address,
    pub supplier_account: Address,
    pub purchase_order: PurchaseOrder,
    pub milestones: Vec<Milestone>,
    pub payment_token: Asset,
    pub discount_terms: DiscountTerms,
    pub dispute_window: u32,
    pub required_signatures: u32,
}

/// Overall contract status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractStatus {
    Draft,      // No milestones added yet
    Active,     // Has milestones but none completed
    InProgress, // Some milestones completed/paid
    Completed,  // All milestones paid
    Disputed,   // Has disputed milestones
}

/// Dispute information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub milestone_index: u64,
    pub initiator: Address,
    pub reason: String,
    pub timestamp: u64,
    pub status: DisputeStatus,
}

/// Dispute status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    Open,
    Resolved,
    Rejected,
}

/// Contract data keys
pub const CONTRACT_KEY: &[u8] = b"contract";
pub const DISPUTE_PREFIX: &[u8] = b"dispute_"; 