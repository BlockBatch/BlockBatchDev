# Payroll Contract

A Stellar smart contract for managing automatic payroll payments with support for multiple employees, tax calculations, and flexible payment schedules.

## Features

- **Automatic Payroll Management**: Create and manage payroll contracts with configurable payment schedules
- **Employee Management**: Add, remove, and manage employee information including salaries and benefits
- **Tax Calculation**: Support for jurisdiction-based tax rates with automatic deductions
- **Payment Scheduling**: Flexible payment frequencies (weekly, bi-weekly, monthly, quarterly, annually)
- **Payment History**: Complete audit trail of all payments made
- **Multi-signature Support**: Support for multiple authorized signers
- **Token Flexibility**: Support for any Stellar asset as payment token

## Contract Structure

### PayrollContract
```rust
struct PayrollContract {
    employer_account: Address,
    payment_schedule: TimeSchedule,
    employees: Vec<Employee>,
    tax_rates: Map<String, i128>, // Tax rates as basis points (10000 = 100%)
    payment_token: Asset,
    minimum_balance: i128,
    signers: Vec<Address>,
    status: ContractStatus,
    contract_id: String,
    created_at: u64,
}
```

### Employee
```rust
struct Employee {
    account_id: Address,
    employee_id: String,
    base_salary: i128, // Annual salary in stroops
    tax_jurisdiction: String,
    benefits: Vec<Benefit>,
    payment_history: Vec<Payment>,
    is_active: bool,
}
```

## API Methods

### Contract Management

#### `create_payroll_contract`
Initialize a new payroll contract.

**Parameters:**
- `employer_account`: Address of the employer
- `payment_schedule`: Payment frequency and timing
- `payment_token`: Asset to use for payments
- `minimum_balance`: Minimum contract balance required
- `signers`: List of authorized signers
- `contract_id`: Unique identifier for the contract

#### `get_payroll_status`
Get the current status of the payroll contract.

**Returns:** `ContractStatus` (Active, Paused, Terminated, PendingApproval)

#### `update_contract_status`
Update the contract status (employer only).

**Parameters:**
- `caller`: Address of the caller (must be employer)
- `new_status`: New contract status

### Employee Management

#### `add_employee`
Add a new employee to the payroll.

**Parameters:**
- `caller`: Address of the caller (must be authorized)
- `employee`: Employee data structure

#### `remove_employee`
Remove an employee from the payroll.

**Parameters:**
- `caller`: Address of the caller (must be authorized)
- `employee_id`: ID of the employee to remove

#### `get_employees`
Get all active employees in the payroll.

**Returns:** `Vec<Employee>`

### Payment Management

#### `schedule_payment`
Schedule the next payment based on the payment frequency.

**Parameters:**
- `caller`: Address of the caller (must be authorized)

**Returns:** Next payment date (Unix timestamp)

#### `execute_payment`
Execute payments for all active employees.

**Parameters:**
- `caller`: Address of the caller (must be authorized)

**Returns:** `Vec<String>` - List of payment IDs

#### `get_payment_history`
Get payment history for all employees or a specific employee.

**Parameters:**
- `employee_id`: Optional employee ID (if None, returns global history)

**Returns:** `Vec<Payment>`

### Tax Management

#### `set_tax_rate`
Set or update tax rate for a jurisdiction.

**Parameters:**
- `caller`: Address of the caller (must be authorized)
- `jurisdiction`: Tax jurisdiction (e.g., "US", "CA", "UK")
- `tax_rate`: Tax rate in basis points (2500 = 25%)

## Payment Frequencies

- **Weekly**: Payments every 7 days
- **BiWeekly**: Payments every 14 days
- **Monthly**: Payments every 30 days (approximation)
- **Quarterly**: Payments every 90 days
- **Annually**: Payments every 365 days

## Error Handling

The contract includes comprehensive error handling for various scenarios:

- `Unauthorized`: Caller is not authorized to perform the action
- `InvalidAmount`: Invalid payment amount or salary
- `InvalidStatus`: Contract is not in the correct status
- `InsufficientBalance`: Contract doesn't have enough balance for payments
- `EmployeeNotFound`: Employee doesn't exist
- `EmployeeAlreadyExists`: Employee already exists in the system
- `InvalidPaymentSchedule`: Payment schedule is invalid
- `PaymentNotDue`: Payment is not yet due
- `InvalidTaxRate`: Tax rate is outside valid range (0-100%)

## Usage Example

```rust
// Create a payroll contract
let payment_schedule = TimeSchedule {
    frequency: PaymentFrequency::Monthly,
    start_date: current_time + 86400, // Start tomorrow
    next_payment_date: current_time + 86400,
    end_date: None,
};

let asset = Asset {
    token: usdc_token_address,
    symbol: String::from_str(&env, "USDC"),
    decimals: 7,
};

client.create_payroll_contract(
    &employer_address,
    &payment_schedule,
    &asset,
    &1000_0000000, // 1000 USDC minimum balance
    &signers,
    &String::from_str(&env, "PAYROLL_001"),
);

// Add an employee
let employee = Employee {
    account_id: employee_address,
    employee_id: String::from_str(&env, "EMP_001"),
    base_salary: 100_000_0000000, // 100,000 USDC annually
    tax_jurisdiction: String::from_str(&env, "US"),
    benefits: Vec::new(&env),
    payment_history: Vec::new(&env),
    is_active: true,
};

client.add_employee(&employer_address, &employee);

// Set tax rate for US (25%)
client.set_tax_rate(&employer_address, &String::from_str(&env, "US"), &2500);

// Execute payments when due
client.execute_payment(&employer_address);
```

## Building and Testing

```bash
# Build the contract
make build

# Run tests
make test

# Format code
make fmt

# Clean build artifacts
make clean
```

## Security Considerations

1. **Authorization**: All sensitive operations require proper authorization
2. **Balance Checks**: Contract verifies sufficient balance before payments
3. **Input Validation**: All inputs are validated for correctness
4. **Audit Trail**: Complete payment history is maintained
5. **Multi-signature**: Support for multiple authorized signers

## License

This contract is part of the BlockBatch project and follows the project's licensing terms. 