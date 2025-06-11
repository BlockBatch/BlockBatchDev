use soroban_sdk::{contracterror, contracttype, Address, Map, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractStatus {
    Active,
    Paused,
    Terminated,
    PendingApproval,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentFrequency {
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    Annually,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeSchedule {
    pub frequency: PaymentFrequency,
    pub start_date: u64, // Unix timestamp
    pub next_payment_date: u64, // Unix timestamp
    pub end_date: Option<u64>, // Optional end date
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BenefitType {
    HealthInsurance,
    Retirement,
    VacationDays,
    SickLeave,
    Bonus,
    Other(String),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Benefit {
    pub benefit_type: BenefitType,
    pub amount: i128, // Amount in stroops or percentage (basis points)
    pub description: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    pub payment_id: String,
    pub employee_id: String,
    pub gross_amount: i128,
    pub tax_amount: i128,
    pub net_amount: i128,
    pub payment_date: u64, // Unix timestamp
    pub status: PaymentStatus,
    pub transaction_hash: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub account_id: Address,
    pub employee_id: String,
    pub base_salary: i128, // Annual salary in stroops
    pub tax_jurisdiction: String,
    pub benefits: Vec<Benefit>,
    pub payment_history: Vec<Payment>,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub token: Address,
    pub symbol: String,
    pub decimals: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PayrollContract {
    pub employer_account: Address,
    pub payment_schedule: TimeSchedule,
    pub employees: Vec<Employee>,
    pub tax_rates: Map<String, i128>, // Tax rates as basis points (10000 = 100%)
    pub payment_token: Asset,
    pub minimum_balance: i128, // Minimum balance required in stroops
    pub signers: Vec<Address>,
    pub status: ContractStatus,
    pub contract_id: String,
    pub created_at: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PayrollError {
    Unauthorized = 1,
    InvalidAmount = 2,
    InvalidStatus = 3,
    InsufficientBalance = 4,
    EmployeeNotFound = 5,
    EmployeeAlreadyExists = 6,
    InvalidPaymentSchedule = 7,
    PaymentNotDue = 8,
    InvalidTaxRate = 9,
    ContractNotActive = 10,
    InvalidEmployee = 11,
    PaymentFailed = 12,
    NotInitialized = 13,
    InvalidSigner = 14,
    DuplicatePayment = 15,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Contract,           // PayrollContract data
    Employee(String),   // Employee data by employee_id
    PaymentHistory,     // Global payment history
    NextPaymentId,      // Counter for payment IDs
    TotalEmployees,     // Total number of employees
} 