# XRPL Smart Contracts Documentation

## Overview

XRPL Smart Contracts are WebAssembly (WASM) modules that run on the XRP Ledger, enabling programmable functionality beyond the native transaction types. Contracts can store data, emit events, and create transactions programmatically.

## Table of Contents

1. [Contract Lifecycle](#contract-lifecycle)
2. [Contract Structure](#contract-structure)
3. [Data Storage Patterns](#data-storage-patterns)
4. [Parameters](#parameters)
5. [Events](#events)
6. [Transaction Emission](#transaction-emission)
7. [Host Functions](#host-functions)
8. [Testing Patterns](#testing-patterns)
9. [Code Examples](#code-examples)

---

## Contract Lifecycle

### Contract Creation

Contracts are created using the `ContractCreate` transaction type. There are two modes:

**1. Deploy New Contract (with ContractCode)**
```typescript
const createTx: ContractCreate = {
  TransactionType: 'ContractCreate',
  Account: ownerAddress,
  ContractCode: wasmBytesHex,  // WASM binary as hex string
  Functions: [
    // Function definitions with parameters
  ],
  InstanceParameters: [
    // Optional: Define instance parameters
  ],
  InstanceParameterValues: [
    // Optional: Initialize instance parameter values
  ],
  Fee: '200000',
}
```

**2. Install Existing Contract (with ContractHash)**
```typescript
const installTx: ContractCreate = {
  TransactionType: 'ContractCreate',
  Account: ownerAddress,
  ContractHash: existingContractHash,  // Reference to existing contract
  InstanceParameterValues: [
    // Provide values for the contract's instance parameters
  ],
  Fee: '200000',
}
```

**Key Concepts:**

- **ContractSource**: Stored once, shared by multiple instances (reference counting)
- **Contract Instance**: A unique deployment with its own state and parameters
- **ContractAccount**: A pseudo-account created for each instance to manage the contract's state

### Contract Modification

Modify an existing contract instance using `ContractModify`:

```typescript
const modifyTx: ContractModify = {
  TransactionType: 'ContractModify',
  Account: ownerAddress,
  ContractAccount: contractPseudoAccount,

  // Option 1: Change the contract code
  ContractCode: newWasmBytesHex,
  // OR
  ContractHash: newContractHash,

  // Option 2: Transfer ownership
  ContractOwner: newOwnerAddress,

  Fee: '200000',
}
```

### Contract Deletion

Delete a contract instance using `ContractDelete`:

```typescript
const deleteTx: ContractDelete = {
  TransactionType: 'ContractDelete',
  Account: ownerAddress,
  ContractAccount: contractPseudoAccount,
  Fee: '10',
}
```

**Deletion Behavior:**
- Deletes the contract instance ledger entry
- Deletes the pseudo-account
- Decrements the ContractSource reference count
- If reference count reaches 0, the ContractSource is also deleted

---

## Contract Structure

### Functions

Functions are the entry points to your contract. Each function must be defined during contract creation:

```typescript
interface Function {
  FunctionName: string,  // Hex-encoded function name
  Parameters?: Parameter[],  // Optional function parameters
}
```

**Constraints:**
- Maximum 12 functions per contract
- Maximum 32 parameters per function
- Function names must be unique

### Flags

Contract flags control contract behavior:

- `tfImmutable` (0x00010000): Contract code cannot be modified after creation

```typescript
const createTx: ContractCreate = {
  // ...
  Flags: ContractFlags.tfImmutable,
}
```

---

## Data Storage Patterns

Contracts can store data in four different patterns. All data is stored per-account basis within the contract's state.

### 1. Simple Object (Key-Value Pairs)

Store flat key-value pairs directly on an account.

**Structure:**
```json
{
  "name": "Alice",
  "age": 30,
  "verified": true
}
```

**Rust Example:**
```rust
use xrpl_wasm_std::core::data::codec::{get_data, set_data};
use xrpl_wasm_std::core::types::account_id::AccountID;

// Set data
set_data::<u32>(&account, "age", 30)?;

// Get data
if let Some(age) = get_data::<u32>(&account, "age") {
    // Use age value
}
```

**Supported Types:**
- `u8`, `u16`, `u32`, `u64`, `u128`
- `UInt160`, `UInt192`, `Hash256` (UInt256)
- `AccountID`
- `TokenAmount` (XRP, IOU, MPT)
- `Number`
- Variable length data (VL/Blob)

### 2. Nested Objects

Store objects within objects (single level of nesting).

**Structure:**
```json
{
  "profile": {
    "firstName": "Alice",
    "lastName": "Smith",
    "age": 25
  },
  "settings": {
    "theme": "dark",
    "notifications": true
  }
}
```

**Rust Example:**
```rust
use xrpl_wasm_std::core::data::codec::{get_nested_data, set_nested_data};

// Set nested data
set_nested_data::<u32>(&account, "profile", "age", 25)?;

// Get nested data
if let Some(age) = get_nested_data::<u32>(&account, "profile", "age") {
    // Use age value
}
```

### 3. Objects with Arrays

Store arrays of simple values.

**Structure:**
```json
{
  "scores": [100, 95, 87],
  "items": [1, 2, 3, 4, 5]
}
```

**Rust Example:**
```rust
use xrpl_wasm_std::core::data::codec::{get_array_element, set_array_element};

// Set array elements
set_array_element::<u32>(&account, "scores", 0, 100)?;
set_array_element::<u32>(&account, "scores", 1, 95)?;

// Get array element
if let Some(score) = get_array_element::<u32>(&account, "scores", 0) {
    // Use score value
}
```

**Note:** Arrays can have gaps. Unset indices may be treated as null/zero values.

### 4. Objects with Nested Arrays

Store arrays of objects (most complex pattern, depth limit: 1).

**Structure:**
```json
{
  "items": [
    { "id": 1, "name": "Item1", "price": 100 },
    { "id": 2, "name": "Item2", "price": 200 }
  ]
}
```

**Rust Example:**
```rust
use xrpl_wasm_std::core::data::codec::{
    get_nested_array_element,
    set_nested_array_element
};

// Set nested array elements
set_nested_array_element::<u32>(&account, "items", 0, "id", 1)?;
set_nested_array_element::<u32>(&account, "items", 0, "price", 100)?;

// Get nested array element
if let Some(price) = get_nested_array_element::<u32>(&account, "items", 0, "price") {
    // Use price value
}
```

**⚠️ Warning:** This pattern is complex and may not be necessary for most use cases. Consider redesigning your data model if you find yourself needing deeply nested structures.

### Data Storage Best Practices

1. **Keep structures flat when possible** - Simpler patterns are easier to maintain
2. **Use appropriate types** - Choose the smallest type that fits your data
3. **Consider gas costs** - More complex structures cost more to read/write
4. **Plan for updates** - Design data models that are easy to update
5. **Avoid deep nesting** - Maximum depth is 1 level

---

## Parameters

Parameters allow you to configure contracts at deployment and runtime.

### Instance Parameters

Instance parameters are defined during contract creation and set per-contract-instance. They are immutable after deployment unless the contract is modified.

**Define during ContractCreate:**
```typescript
const instanceParam = new iInstanceParameter(
  new iParameterFlag(65536),  // tfSendAmount flag
  new iParameterType('AMOUNT')
)

const instanceParamValue = new iInstanceParameterValue(
  instanceParam.flag,
  instanceParam.type,
  new iParameterValue(xrpToDrops('100'))
)

const createTx: ContractCreate = {
  // ...
  InstanceParameters: [instanceParam.toXrpl()],
  InstanceParameterValues: [instanceParamValue.toXrpl()],
}
```

**Access in Contract (Rust):**
```rust
use xrpl_wasm_std::core::params::instance::get_instance_param;
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;

// Get instance parameter by index
let amount = get_instance_param::<TokenAmount>(0)?;
```

### Function Parameters

Function parameters are defined in the function signature and provided during `ContractCall`.

**Define in Function:**
```typescript
const functionParam = new iFunctionParameter(
  new iParameterFlag(0),
  new iParameterType('UINT32')
)

const function1 = new iFunction(
  new iFunctionName('transfer'),
  [functionParam]
)
```

**Provide during ContractCall:**
```typescript
const param = new iParameter(
  new iParameterFlag(0),
  new iParameterType('UINT32'),
  new iParameterValue(12345)
)

const callTx: ContractCall = {
  TransactionType: 'ContractCall',
  Account: callerAddress,
  ContractAccount: contractPseudoAccount,
  FunctionName: convertStringToHex('transfer'),
  Parameters: [param.toXrpl()],
  ComputationAllowance: 1000000,
  Fee: '5000',
}
```

**Access in Contract (Rust):**
```rust
use xrpl_wasm_std::core::params::function::get_function_param;

// Get function parameter by index
let value = get_function_param::<u32>(0)?;
```

### Supported Parameter Types

| Type | Description | Size | Example |
|------|-------------|------|---------|
| `UINT8` | Unsigned 8-bit integer | 1 byte | `255` |
| `UINT16` | Unsigned 16-bit integer | 2 bytes | `65535` |
| `UINT32` | Unsigned 32-bit integer | 4 bytes | `4294967295` |
| `UINT64` | Unsigned 64-bit integer | 8 bytes | `"18446744073709551615"` |
| `UINT128` | Unsigned 128-bit integer | 16 bytes | `"340282366920938463463374607431768211455"` |
| `UINT160` | Unsigned 160-bit integer | 20 bytes | Account IDs, Hashes |
| `UINT192` | Unsigned 192-bit integer | 24 bytes | - |
| `UINT256` | Unsigned 256-bit integer | 32 bytes | Hashes, IDs |
| `VL` | Variable length data | Variable | Hex strings |
| `ACCOUNT` | XRP Ledger account | 20 bytes | `"rN7n7otQDd6FczFgLdlqtyMVrn3z7otQDd"` |
| `AMOUNT` | Currency amount | Variable | XRP, IOU, or MPT |
| `NUMBER` | Decimal number | 12 bytes | `"1.23"` |
| `CURRENCY` | Currency code | 20 bytes | `"USD"` |
| `ISSUE` | Asset identifier | Variable | Currency + Issuer |

### Parameter Flags

Common parameter flags:

- `0` - Default, no special behavior
- `65536` (0x10000) - `tfSendAmount` - Amount to send with transaction

---

## Events

Contracts can emit events to notify external systems of state changes or important occurrences.

### Emitting Events

**Rust Example:**
```rust
use xrpl_wasm_std::core::event::codec_v2::{
    EventBuffer,
    event_add_u32,
    event_add_str,
    event_add_account,
};

#[no_mangle]
pub extern "C" fn my_function() -> i32 {
    let mut event_buffer = EventBuffer::new();

    // Add fields to event
    event_add_u32(&mut event_buffer, "value", 12345).unwrap();
    event_add_str(&mut event_buffer, "message", "Hello, World!").unwrap();
    event_add_account(&mut event_buffer, "sender", &account.0).unwrap();

    // Emit the event with a name
    event_buffer.emit("transfer_complete").unwrap();

    0
}
```

### Event Functions

Available event field functions:

- `event_add_u8(buf, name, value)` - Add u8 field
- `event_add_u16(buf, name, value)` - Add u16 field
- `event_add_u32(buf, name, value)` - Add u32 field
- `event_add_u64(buf, name, value)` - Add u64 field
- `event_add_u128(buf, name, bytes)` - Add u128 field
- `event_add_u160(buf, name, bytes)` - Add u160 field
- `event_add_u192(buf, name, bytes)` - Add u192 field
- `event_add_u256(buf, name, bytes)` - Add u256 field
- `event_add_amount(buf, name, bytes)` - Add amount field
- `event_add_account(buf, name, bytes)` - Add account field
- `event_add_currency(buf, name, bytes)` - Add currency field
- `event_add_str(buf, name, str)` - Add string field

### Subscribing to Events

**WebSocket Subscription:**
```typescript
// Subscribe to contract events
const client = new Client('wss://alphanet.nerdnest.xyz')
await client.connect()

await client.request({
  command: 'subscribe',
  streams: ['contract_events']
})

client.on('contractEvent', (event) => {
  console.log('Event Name:', event.name)
  console.log('Event Data:', event.data)
  console.log('Contract:', event.contract_account)
})
```

### Event Example Output

```json
{
  "type": "contractEvent",
  "name": "transfer_complete",
  "contract_account": "rN7n7otQDd6FczFgLdlqtyMVrn3z7otQDd",
  "data": {
    "value": 12345,
    "message": "Hello, World!",
    "sender": "rPEPPER7kfTD9w2To4CQk6UCfuHM9c6GDY"
  }
}
```

---

## Transaction Emission

Contracts can build and emit transactions programmatically, enabling complex workflows.

### Building a Transaction

**Rust Example:**
```rust
use xrpl_wasm_std::host::{build_txn, add_txn_field, emit_built_txn};
use xrpl_wasm_std::core::transaction_types::TT_PAYMENT;
use xrpl_wasm_std::sfield;

#[no_mangle]
pub extern "C" fn emit_payment() -> i32 {
    // 1. Build transaction
    let txn_index = unsafe { build_txn(TT_PAYMENT) };
    if txn_index < 0 {
        return -1;
    }

    // 2. Add destination field
    let mut dest_buffer = [0u8; 21];
    dest_buffer[0] = 0x14;  // AccountID type marker
    dest_buffer[1..21].copy_from_slice(&destination_account.0);

    unsafe {
        add_txn_field(
            txn_index,
            sfield::Destination,
            dest_buffer.as_ptr(),
            dest_buffer.len()
        );
    }

    // 3. Add amount field (192 drops)
    const AMOUNT: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0
    ];

    unsafe {
        add_txn_field(
            txn_index,
            sfield::Amount,
            AMOUNT.as_ptr(),
            AMOUNT.len()
        );
    }

    // 4. Emit the transaction
    let result = unsafe { emit_built_txn(txn_index) };
    if result < 0 {
        return result;
    }

    0
}
```

### Adding Transaction Fields

Common transaction fields:

| Field | sfield | Description |
|-------|--------|-------------|
| Destination | `sfield::Destination` | Payment recipient |
| Amount | `sfield::Amount` | Payment amount |
| SendMax | `sfield::SendMax` | Maximum amount to send |
| DestinationTag | `sfield::DestinationTag` | Destination identifier |
| Memos | `sfield::Memos` | Transaction notes |
| LimitAmount | `sfield::LimitAmount` | Trust line limit |
| QualityIn | `sfield::QualityIn` | Trust line quality |

### Transaction Types

Supported transaction types:

- `TT_PAYMENT` - Send XRP or issued currency
- `TT_ACCOUNT_SET` - Modify account settings
- `TT_TRUST_SET` - Create/modify trust line
- `TT_OFFER_CREATE` - Place order in DEX
- `TT_OFFER_CANCEL` - Cancel DEX order

### Adding Memos

**Rust Example:**
```rust
use xrpl_wasm_std::core::submit::inner_objects::build_memo;

// Create memo buffer
let mut memo_buffer = [0u8; 256];

// Build memo
let memo_len = build_memo(
    &mut memo_buffer,
    Some(b"invoice"),           // MemoType
    Some(b"INV-2024-001"),      // MemoData
    Some(b"text/plain")         // MemoFormat
);

// Add to memos array
let mut memos_array = [0u8; 1024];
let mut pos = 0;

// Copy memo
memos_array[pos..pos + memo_len].copy_from_slice(&memo_buffer[..memo_len]);
pos += memo_len;

// Terminate array
memos_array[pos] = 0xF1;  // ARRAY_END marker
pos += 1;

// Add memos field
unsafe {
    add_txn_field(
        txn_index,
        sfield::Memos,
        memos_array.as_ptr(),
        pos
    );
}
```

---

## Host Functions

Host functions are low-level APIs provided by the XRPL runtime to contracts.

### Data Operations

- `get_data(account, key, type, buf, len)` - Read simple data
- `set_data(account, key, type, buf, len)` - Write simple data
- `get_nested_data(account, key1, key2, type, buf, len)` - Read nested data
- `set_nested_data(account, key1, key2, type, buf, len)` - Write nested data
- `get_array_element(account, key, index, type, buf, len)` - Read array element
- `set_array_element(account, key, index, type, buf, len)` - Write array element
- `get_nested_array_element(account, key, index, field, type, buf, len)` - Read nested array element
- `set_nested_array_element(account, key, index, field, type, buf, len)` - Write nested array element

### Parameter Access

- `instance_param(index, type, buf, len)` - Read instance parameter
- `function_param(index, type, buf, len)` - Read function parameter

### Transaction Building

- `build_txn(transaction_type)` - Initialize transaction builder
- `add_txn_field(txn_index, field_code, buf, len)` - Add field to transaction
- `emit_built_txn(txn_index)` - Submit built transaction

### Utility Functions

- `trace(message)` - Log debug message
- `trace_num(message, number)` - Log number
- `trace_data(message, data, repr)` - Log hex data
- `trace_float(message, float)` - Log float value

### Ledger Access

- `ledger_entry(keylet_type, params)` - Read ledger entry
- `ledger_seq()` - Get current ledger sequence

### Account Functions

- `account_info(account)` - Get account details
- `owner_count(account)` - Get number of owned objects

---

## Testing Patterns

### Unit Testing

Test contracts in isolation:

**Test Structure (C++):**
```cpp
void testContractCreate() {
    testcase("contract create");

    using namespace jtx;
    test::jtx::Env env{*this, features};

    auto const alice = Account{"alice"};
    env.fund(XRP(10'000), alice);
    env.close();

    // Create contract
    env(contract::create(alice, wasmHex),
        contract::add_function("myFunc", {}),
        fee(XRP(200)),
        ter(tesSUCCESS));
    env.close();

    // Validate contract exists
    auto const contractKey = keylet::contract(contractHash, alice, seq);
    BEAST_EXPECT(env.le(contractKey));
}
```

### Integration Testing

Test contract interactions:

```cpp
void testContractCall() {
    testcase("contract call");

    // Setup
    auto const alice = Account{"alice"};
    auto const bob = Account{"bob"};
    env.fund(XRP(10'000), alice, bob);

    // Create contract
    auto const [contractAccount, contractHash, _] = setContract(
        env,
        tesSUCCESS,
        contract::create(alice, wasmHex),
        contract::add_function("transfer", {{0, "amount", "UINT32"}}),
        fee(XRP(200))
    );

    // Call contract
    env(contract::call(bob, contractAccount, "transfer"),
        contract::add_param(0, "amount", "UINT32", 1000),
        escrow::comp_allowance(1'000'000),
        ter(tesSUCCESS));
    env.close();

    // Validate results
    auto const contractInfo = env.rpc("contract_info", params);
    // Assert expected state
}
```

### Test Validation

Common validation patterns:

1. **Validate Contract Creation:**
```cpp
auto const [id, sle] = contractKeyAndSle(*env.current(), contractHash, owner, seq);
BEAST_EXPECT(sle);
BEAST_EXPECT(sle->getAccountID(sfContractAccount) == expectedAccount);
```

2. **Validate Contract Source:**
```cpp
auto const [sourceId, sourceSle] = contractSourceKeyAndSle(*env.current(), contractHash);
BEAST_EXPECT(sourceSle);
BEAST_EXPECT(sourceSle->getFieldU64(sfReferenceCount) == expectedCount);
```

3. **Validate Contract Data:**
```cpp
auto const data = getContractData(env, contractAccount, account, "key");
BEAST_EXPECT(data == expectedValue);
```

### Error Testing

Test error conditions:

```cpp
// Test invalid parameter count
env(contract::create(alice, wasmHex),
    contract::add_function("func", {
        /* 33 parameters - too many */
    }),
    ter(temARRAY_TOO_LARGE));

// Test missing required field
env(contract::call(alice, contractAccount, "transfer"),
    // Missing amount parameter
    ter(temMALFORMED));

// Test insufficient computation
env(contract::call(alice, contractAccount, "expensive_operation"),
    escrow::comp_allowance(100),  // Too low
    ter(tecCOMPUTATION_EXCEEDED));
```

---

## Code Examples

### Complete Contract: Simple Token

**Rust Contract:**
```rust
#![cfg_attr(target_arch = "wasm32", no_std)]

use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::data::codec::{get_data, set_data};
use xrpl_wasm_std::core::params::function::get_function_param;

const TOTAL_SUPPLY_KEY: &str = "total_supply";
const BALANCE_PREFIX: &str = "balance_";

#[no_mangle]
pub extern "C" fn initialize() -> i32 {
    let contract = AccountID::contract();

    // Set initial supply
    if set_data::<u64>(&contract, TOTAL_SUPPLY_KEY, 1_000_000).is_err() {
        return -1;
    }

    0
}

#[no_mangle]
pub extern "C" fn transfer() -> i32 {
    // Get function parameters
    let to = match get_function_param::<AccountID>(0) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };

    let amount = match get_function_param::<u64>(1) {
        Ok(amt) => amt,
        Err(_) => return -1,
    };

    let from = AccountID::caller();

    // Get sender balance
    let from_balance = get_data::<u64>(&from, BALANCE_PREFIX)
        .unwrap_or(0);

    if from_balance < amount {
        return -1;  // Insufficient balance
    }

    // Get recipient balance
    let to_balance = get_data::<u64>(&to, BALANCE_PREFIX)
        .unwrap_or(0);

    // Update balances
    set_data::<u64>(&from, BALANCE_PREFIX, from_balance - amount).unwrap();
    set_data::<u64>(&to, BALANCE_PREFIX, to_balance + amount).unwrap();

    0
}

#[no_mangle]
pub extern "C" fn balance_of() -> i32 {
    let account = match get_function_param::<AccountID>(0) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };

    let balance = get_data::<u64>(&account, BALANCE_PREFIX)
        .unwrap_or(0);

    // Return balance via trace for now
    // In production, use proper return mechanism
    0
}
```

### Deployment Script (TypeScript)

```typescript
import {
  Client,
  ContractCreate,
  Wallet,
  xrpToDrops,
} from '@transia/xrpl'
import {
  createContract,
  readWasmFromPath,
} from '@transia/craft-toolkit-ts'

async function deployToken() {
  const client = new Client('wss://alphanet.nerdnest.xyz')
  await client.connect()

  const wallet = Wallet.fromSeed('sEdSK...')

  // Define contract functions
  const initFunction = new iFunction(
    new iFunctionName('initialize'),
    []
  )

  const transferFunction = new iFunction(
    new iFunctionName('transfer'),
    [
      new iFunctionParameter(
        new iParameterFlag(0),
        new iParameterType('ACCOUNT')
      ),
      new iFunctionParameter(
        new iParameterFlag(0),
        new iParameterType('UINT64')
      ),
    ]
  )

  // Create contract
  const createTx: ContractCreate = {
    TransactionType: 'ContractCreate',
    Account: wallet.classicAddress,
    ContractCode: readWasmFromPath('./token.wasm'),
    Functions: [
      initFunction.toXrpl(),
      transferFunction.toXrpl(),
    ],
    Fee: xrpToDrops('200'),
  }

  const { id, account } = await createContract({
    client,
    wallet,
    tx: createTx,
  })

  console.log('Token Contract:', account)
  console.log('Contract ID:', id)

  // Initialize the contract
  await client.submit({
    TransactionType: 'ContractCall',
    Account: wallet.classicAddress,
    ContractAccount: account,
    FunctionName: convertStringToHex('initialize'),
    ComputationAllowance: 1_000_000,
    Fee: '5000',
  })

  await client.disconnect()
}

deployToken()
```

---

## Best Practices

### Security

1. **Validate all inputs** - Never trust function parameters
2. **Check balances** - Verify sufficient funds before transfers
3. **Use safe math** - Prevent overflows and underflows
4. **Implement access control** - Restrict sensitive operations
5. **Emit events** - Log important state changes

### Performance

1. **Minimize storage operations** - Reads and writes are expensive
2. **Batch updates** - Group related changes together
3. **Use appropriate types** - Smaller types use less gas
4. **Avoid loops** - Fixed-iteration operations are more predictable
5. **Cache frequently used data** - Reduce redundant reads

### Design

1. **Keep contracts simple** - Complex logic is harder to audit
2. **Follow conventions** - Use standard patterns
3. **Document thoroughly** - Explain non-obvious logic
4. **Version your contracts** - Plan for upgrades
5. **Test extensively** - Cover edge cases

### Gas Optimization

1. **Computation limits** - Set appropriate `ComputationAllowance`
2. **Minimize memory allocation** - Use stack-based data structures
3. **Optimize data structures** - Choose efficient storage patterns
4. **Reduce function calls** - Inline simple operations
5. **Profile your code** - Identify bottlenecks

---

## Additional Resources

- [XRPL Smart Contracts Specification](https://github.com/ripple/rippled)
- [Craft Toolkit TypeScript SDK](https://github.com/transia/craft-toolkit-ts)
- [XRPL WASM Standard Library](https://github.com/ledger-works/xrpl-wasm-std)
- [Smart Contract Examples](../contracts/)

---

## Glossary

- **WASM**: WebAssembly, a binary instruction format for a stack-based virtual machine
- **ContractSource**: The shared code storage for a contract
- **Contract Instance**: A specific deployment of a contract with its own state
- **Pseudo-Account**: An account created to manage a contract's state
- **Instance Parameter**: Configuration value set at contract deployment
- **Function Parameter**: Runtime value provided when calling a contract function
- **Event**: A notification emitted by a contract
- **Host Function**: A native function provided by the XRPL runtime
- **Computation Allowance**: Maximum gas/computation units for a contract call
- **Keylet**: A unique identifier for a ledger entry

---

## Changelog

- **2025-01-01**: Initial documentation based on xrpld-smart-contracts test suite
