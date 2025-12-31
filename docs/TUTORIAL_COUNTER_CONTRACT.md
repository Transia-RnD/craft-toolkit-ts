# Tutorial: Building Your First Smart Contract - Counter

## Introduction

This tutorial will guide you through building a simple but complete smart contract: a **Counter** that demonstrates all the core concepts of XRPL smart contracts.

### What You'll Learn

- Setting up a contract with instance parameters
- Defining functions with function parameters
- Storing and retrieving contract data
- Emitting events
- Building and emitting transactions from a contract
- Deploying and testing your contract

### What We'll Build

A counter contract that:
- Stores a counter value per account
- Has an initial maximum value (instance parameter)
- Can increment the counter (with custom step size)
- Can decrement the counter
- Can reset the counter
- Emits events when the counter changes
- Sends XRP rewards when milestones are reached

---

## Prerequisites

### Tools You Need

1. **Rust** (with wasm32 target)
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **xrpl-wasm-std** library
   ```toml
   [dependencies]
   xrpl-wasm-std = { git = "https://github.com/ledger-works/xrpl-wasm-std" }
   ```

3. **Node.js** and **craft-toolkit-ts**
   ```bash
   npm install @transia/craft-toolkit-ts @transia/xrpl
   ```

---

## Part 1: Contract Design

### Data Model

```
Contract State (per account):
{
  "counter": u32,          // Current counter value
  "total_increments": u64, // Total times incremented
  "last_updated": u64      // Last update timestamp
}
```

### Parameters

**Instance Parameters** (set at deployment):
1. `max_value` (UINT32) - Maximum counter value
2. `reward_amount` (AMOUNT) - XRP to send when reaching max

**Function Parameters**:
- `increment(step: UINT32)` - How much to increment
- `reset()` - No parameters

### Functions

1. `increment(step)` - Increase counter by step
2. `decrement()` - Decrease counter by 1
3. `reset()` - Reset counter to 0
4. `get_counter()` - Return current value

---

## Part 2: Writing the Contract

### Create Project Structure

```bash
cargo new --lib counter_contract
cd counter_contract
```

### Update Cargo.toml

```toml
[package]
name = "counter_contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
xrpl-wasm-std = { git = "https://github.com/ledger-works/xrpl-wasm-std" }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Reduce number of codegen units
panic = "abort"     # Abort on panic
strip = true        # Strip symbols
```

### Write the Contract (src/lib.rs)

```rust
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::{
    core::{
        data::codec::{get_data, set_data},
        types::account_id::AccountID,
        params::{
            instance::get_instance_param,
            function::get_function_param,
        },
        current_tx::contract_call::get_current_contract_call,
        transaction_types::TT_PAYMENT,
    },
    host::{
        trace::{trace, trace_num},
        build_txn,
        add_txn_field,
        emit_built_txn,
    },
    sfield,
};
use xrpl_wasm_std::core::event::codec_v2::{
    EventBuffer,
    event_add_u32,
    event_add_u64,
    event_add_str,
};

// Data keys
const COUNTER_KEY: &str = "counter";
const TOTAL_INCREMENTS_KEY: &str = "total_increments";

// ============================================================================
// Helper Functions
// ============================================================================

fn get_counter(account: &AccountID) -> u32 {
    get_data::<u32>(account, COUNTER_KEY).unwrap_or(0)
}

fn set_counter(account: &AccountID, value: u32) -> Result<(), i32> {
    set_data::<u32>(account, COUNTER_KEY, value)
}

fn get_total_increments(account: &AccountID) -> u64 {
    get_data::<u64>(account, TOTAL_INCREMENTS_KEY).unwrap_or(0)
}

fn increment_total(account: &AccountID) -> Result<(), i32> {
    let total = get_total_increments(account);
    set_data::<u64>(account, TOTAL_INCREMENTS_KEY, total + 1)
}

fn emit_counter_event(
    event_name: &str,
    account: &AccountID,
    old_value: u32,
    new_value: u32,
    total: u64
) -> Result<(), i32> {
    let mut event = EventBuffer::new();

    event_add_str(&mut event, "action", event_name)
        .map_err(|_| -1)?;
    event_add_u32(&mut event, "old_value", old_value)
        .map_err(|_| -1)?;
    event_add_u32(&mut event, "new_value", new_value)
        .map_err(|_| -1)?;
    event_add_u64(&mut event, "total_increments", total)
        .map_err(|_| -1)?;

    event.emit("counter_changed").map_err(|_| -1)?;

    Ok(())
}

// ============================================================================
// Contract Functions
// ============================================================================

/// Increment the counter by a specified step
/// Parameters:
///   - 0: step (UINT32) - Amount to increment by
#[no_mangle]
pub extern "C" fn increment() -> i32 {
    let _ = trace("=== INCREMENT Function ===");

    // Get the caller's account
    let contract_call = get_current_contract_call();
    let caller = match contract_call.get_account() {
        Ok(acc) => acc,
        Err(_) => {
            let _ = trace("Error: Could not get caller account");
            return -1;
        }
    };

    // Get function parameter: step
    let step = match get_function_param::<u32>(0) {
        Ok(s) => s,
        Err(_) => {
            let _ = trace("Error: Could not get step parameter");
            return -1;
        }
    };
    let _ = trace_num("Step value:", step as i64);

    // Get instance parameter: max_value
    let max_value = match get_instance_param::<u32>(0) {
        Ok(m) => m,
        Err(_) => {
            let _ = trace("Error: Could not get max_value parameter");
            return -1;
        }
    };
    let _ = trace_num("Max value:", max_value as i64);

    // Get current counter
    let old_counter = get_counter(&caller);
    let _ = trace_num("Old counter:", old_counter as i64);

    // Check if increment would exceed max
    let new_counter = if old_counter.saturating_add(step) > max_value {
        let _ = trace("Counter would exceed max, capping at max");
        max_value
    } else {
        old_counter + step
    };
    let _ = trace_num("New counter:", new_counter as i64);

    // Update counter
    if let Err(e) = set_counter(&caller, new_counter) {
        let _ = trace("Error: Could not set counter");
        return e;
    }

    // Update total increments
    if let Err(e) = increment_total(&caller) {
        let _ = trace("Error: Could not update total increments");
        return e;
    }

    let total = get_total_increments(&caller);

    // Emit event
    if let Err(e) = emit_counter_event(
        "increment",
        &caller,
        old_counter,
        new_counter,
        total
    ) {
        let _ = trace("Error: Could not emit event");
        return e;
    }

    // If we hit the max, send reward
    if new_counter == max_value {
        let _ = trace("MAX REACHED! Sending reward...");

        // Get reward amount from instance parameter
        // Note: For simplicity, we're hardcoding 100 drops here
        // In a real contract, you'd get this from instance parameter 1
        let reward_drops: [u8; 8] = [
            0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64 // 100 drops
        ];

        // Build payment transaction
        let txn_index = unsafe { build_txn(TT_PAYMENT) };
        if txn_index < 0 {
            let _ = trace("Error: Could not build transaction");
            return txn_index;
        }

        // Add destination (send to caller)
        let mut dest_buf = [0u8; 21];
        dest_buf[0] = 0x14; // AccountID type marker
        dest_buf[1..21].copy_from_slice(&caller.0);

        unsafe {
            if add_txn_field(
                txn_index,
                sfield::Destination,
                dest_buf.as_ptr(),
                dest_buf.len()
            ) < 0 {
                let _ = trace("Error: Could not add destination");
                return -1;
            }

            if add_txn_field(
                txn_index,
                sfield::Amount,
                reward_drops.as_ptr(),
                reward_drops.len()
            ) < 0 {
                let _ = trace("Error: Could not add amount");
                return -1;
            }

            // Emit the transaction
            let result = emit_built_txn(txn_index);
            if result < 0 {
                let _ = trace("Error: Could not emit transaction");
                return result;
            }
        }

        let _ = trace("Reward sent successfully!");
    }

    let _ = trace("Increment completed successfully");
    0
}

/// Decrement the counter by 1
#[no_mangle]
pub extern "C" fn decrement() -> i32 {
    let _ = trace("=== DECREMENT Function ===");

    // Get the caller's account
    let contract_call = get_current_contract_call();
    let caller = match contract_call.get_account() {
        Ok(acc) => acc,
        Err(_) => {
            let _ = trace("Error: Could not get caller account");
            return -1;
        }
    };

    // Get current counter
    let old_counter = get_counter(&caller);
    let _ = trace_num("Old counter:", old_counter as i64);

    // Decrement (with underflow protection)
    let new_counter = old_counter.saturating_sub(1);
    let _ = trace_num("New counter:", new_counter as i64);

    // Update counter
    if let Err(e) = set_counter(&caller, new_counter) {
        let _ = trace("Error: Could not set counter");
        return e;
    }

    let total = get_total_increments(&caller);

    // Emit event
    if let Err(e) = emit_counter_event(
        "decrement",
        &caller,
        old_counter,
        new_counter,
        total
    ) {
        let _ = trace("Error: Could not emit event");
        return e;
    }

    let _ = trace("Decrement completed successfully");
    0
}

/// Reset the counter to 0
#[no_mangle]
pub extern "C" fn reset() -> i32 {
    let _ = trace("=== RESET Function ===");

    // Get the caller's account
    let contract_call = get_current_contract_call();
    let caller = match contract_call.get_account() {
        Ok(acc) => acc,
        Err(_) => {
            let _ = trace("Error: Could not get caller account");
            return -1;
        }
    };

    // Get current counter
    let old_counter = get_counter(&caller);
    let _ = trace_num("Old counter:", old_counter as i64);

    // Reset to 0
    let new_counter = 0;

    // Update counter
    if let Err(e) = set_counter(&caller, new_counter) {
        let _ = trace("Error: Could not set counter");
        return e;
    }

    // Don't reset total increments - keep history

    let total = get_total_increments(&caller);

    // Emit event
    if let Err(e) = emit_counter_event(
        "reset",
        &caller,
        old_counter,
        new_counter,
        total
    ) {
        let _ = trace("Error: Could not emit event");
        return e;
    }

    let _ = trace("Reset completed successfully");
    0
}
```

### Build the Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM will be at:
```
target/wasm32-unknown-unknown/release/counter_contract.wasm
```

---

## Part 3: Deployment Script

Create `deploy-counter.ts`:

```typescript
import {
  Client,
  ContractCall,
  ContractCreate,
  convertStringToHex,
  Wallet,
  xrpToDrops,
} from '@transia/xrpl'
import {
  createContract,
  CreateContractParams,
  readWasmFromPath,
  Xrpld,
} from '@transia/craft-toolkit-ts'
import {
  iInstanceParameter,
  iParameterFlag,
  iParameterType,
  iInstanceParameterValue,
  iParameterValue,
  iFunction,
  iFunctionName,
  iFunctionParameter,
  iParameter,
} from '@transia/craft-toolkit-ts'

class CounterContract {
  private client: Client
  private wallet: Wallet
  private contractAccount: string | null = null

  constructor(wallet: Wallet) {
    this.client = new Client('wss://alphanet.nerdnest.xyz')
    this.wallet = wallet
  }

  async connect() {
    await this.client.connect()
    console.log('✓ Connected to XRPL')
  }

  async disconnect() {
    await this.client.disconnect()
    console.log('✓ Disconnected')
  }

  async deploy() {
    console.log('\n📦 Deploying Counter Contract...')

    // Define instance parameter 1: max_value (UINT32)
    const maxValueParam = new iInstanceParameter(
      new iParameterFlag(0),
      new iParameterType('UINT32')
    )

    const maxValueParamValue = new iInstanceParameterValue(
      maxValueParam.flag,
      maxValueParam.type,
      new iParameterValue(100)  // Max counter value = 100
    )

    // Define instance parameter 2: reward_amount (AMOUNT)
    const rewardAmountParam = new iInstanceParameter(
      new iParameterFlag(65536), // tfSendAmount
      new iParameterType('AMOUNT')
    )

    const rewardAmountParamValue = new iInstanceParameterValue(
      rewardAmountParam.flag,
      rewardAmountParam.type,
      new iParameterValue(xrpToDrops('1'))  // 1 XRP reward
    )

    // Define functions
    const incrementFunc = new iFunction(
      new iFunctionName('increment'),
      [
        new iFunctionParameter(
          new iParameterFlag(0),
          new iParameterType('UINT32')  // step parameter
        )
      ]
    )

    const decrementFunc = new iFunction(
      new iFunctionName('decrement'),
      []  // No parameters
    )

    const resetFunc = new iFunction(
      new iFunctionName('reset'),
      []  // No parameters
    )

    // Create the contract
    const createTx: ContractCreate = {
      TransactionType: 'ContractCreate',
      Account: this.wallet.classicAddress,
      ContractCode: readWasmFromPath(
        './target/wasm32-unknown-unknown/release/counter_contract.wasm'
      ),
      InstanceParameters: [
        maxValueParam.toXrpl(),
        rewardAmountParam.toXrpl(),
      ],
      InstanceParameterValues: [
        maxValueParamValue.toXrpl(),
        rewardAmountParamValue.toXrpl(),
      ],
      Functions: [
        incrementFunc.toXrpl(),
        decrementFunc.toXrpl(),
        resetFunc.toXrpl(),
      ],
      Fee: xrpToDrops('200'),
    }

    const { account } = await createContract({
      client: this.client,
      wallet: this.wallet,
      tx: createTx,
    } as CreateContractParams) as any

    this.contractAccount = account
    console.log(`✓ Contract deployed at: ${account}`)

    return account
  }

  async increment(step: number) {
    if (!this.contractAccount) {
      throw new Error('Contract not deployed')
    }

    console.log(`\n➕ Incrementing counter by ${step}...`)

    const param = new iParameter(
      new iParameterFlag(0),
      new iParameterType('UINT32'),
      new iParameterValue(step)
    )

    const callTx: ContractCall = {
      TransactionType: 'ContractCall',
      Account: this.wallet.classicAddress,
      ContractAccount: this.contractAccount,
      FunctionName: convertStringToHex('increment'),
      Parameters: [param.toXrpl()],
      ComputationAllowance: 1000000,
      Fee: '5000',
    }

    const result = await Xrpld.submit(this.client, {
      wallet: this.wallet,
      tx: callTx,
    })

    console.log('✓ Increment successful')
    console.log('   Result:', result.result.meta.TransactionResult)
  }

  async decrement() {
    if (!this.contractAccount) {
      throw new Error('Contract not deployed')
    }

    console.log('\n➖ Decrementing counter...')

    const callTx: ContractCall = {
      TransactionType: 'ContractCall',
      Account: this.wallet.classicAddress,
      ContractAccount: this.contractAccount,
      FunctionName: convertStringToHex('decrement'),
      ComputationAllowance: 1000000,
      Fee: '5000',
    }

    const result = await Xrpld.submit(this.client, {
      wallet: this.wallet,
      tx: callTx,
    })

    console.log('✓ Decrement successful')
    console.log('   Result:', result.result.meta.TransactionResult)
  }

  async reset() {
    if (!this.contractAccount) {
      throw new Error('Contract not deployed')
    }

    console.log('\n🔄 Resetting counter...')

    const callTx: ContractCall = {
      TransactionType: 'ContractCall',
      Account: this.wallet.classicAddress,
      ContractAccount: this.contractAccount,
      FunctionName: convertStringToHex('reset'),
      ComputationAllowance: 1000000,
      Fee: '5000',
    }

    const result = await Xrpld.submit(this.client, {
      wallet: this.wallet,
      tx: callTx,
    })

    console.log('✓ Reset successful')
    console.log('   Result:', result.result.meta.TransactionResult)
  }
}

// Main execution
async function main() {
  console.log('🚀 Counter Contract Demo\n')

  // Create wallet (replace with your seed)
  const wallet = Wallet.fromSeed('sEdSScfGpPRUnEBcUjR2aFdcnkhDhx8')
  console.log(`Wallet: ${wallet.classicAddress}`)

  const counter = new CounterContract(wallet)

  try {
    // Connect
    await counter.connect()

    // Deploy
    await counter.deploy()

    // Interact with contract
    await counter.increment(5)    // Increment by 5
    await counter.increment(10)   // Increment by 10
    await counter.decrement()     // Decrement by 1
    await counter.increment(85)   // Should hit max (100) and trigger reward
    await counter.reset()         // Reset to 0

    console.log('\n✅ All operations completed!')

  } catch (error) {
    console.error('❌ Error:', error)
  } finally {
    await counter.disconnect()
  }
}

main()
```

---

## Part 4: Testing

### Run the Deployment

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Deploy and test
npx ts-node deploy-counter.ts
```

### Expected Output

```
🚀 Counter Contract Demo

Wallet: rPEPPER7kfTD9w2To4CQk6UCfuHM9c6GDY
✓ Connected to XRPL

📦 Deploying Counter Contract...
✓ Contract deployed at: rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh

➕ Incrementing counter by 5...
✓ Increment successful
   Result: tesSUCCESS

➕ Incrementing counter by 10...
✓ Increment successful
   Result: tesSUCCESS

➖ Decrementing counter...
✓ Decrement successful
   Result: tesSUCCESS

➕ Incrementing counter by 85...
✓ Increment successful
   Result: tesSUCCESS
   💰 Reward payment emitted!

🔄 Resetting counter...
✓ Reset successful
   Result: tesSUCCESS

✅ All operations completed!
✓ Disconnected
```

---

## Part 5: Homework Exercises

Now that you've built the counter, try these exercises:

### Exercise 1: Add Multiplier

Add a `multiply` function that multiplies the counter by a given factor.

**Requirements:**
- Function parameter: `factor` (UINT32)
- Should check if result exceeds max_value
- Emit an event with old and new values

### Exercise 2: Add History

Store the last 5 counter values in an array.

**Requirements:**
- Use `set_array_element` to store values
- Add a `get_history` function that emits an event with all historical values

### Exercise 3: Add Access Control

Only allow the contract owner to reset the counter.

**Requirements:**
- Store owner address as instance parameter
- Check caller against owner in `reset` function
- Return error if caller is not owner

### Exercise 4: Add Leaderboard

Track the highest counter value achieved and who achieved it.

**Requirements:**
- Store: `highest_value` (UINT32), `highest_holder` (ACCOUNT)
- Update these when counter reaches new high
- Add `get_leaderboard` function that emits this data

### Exercise 5: Add Cost to Increment

Make users pay 1 XRP per increment call.

**Requirements:**
- Require `Amount` field in ContractCall transaction
- Validate amount >= 1 XRP
- Store collected fees in contract data
- Add `withdraw_fees` function for owner

---

## Key Takeaways

1. **Instance Parameters** are set at deployment and configure the contract
2. **Function Parameters** are provided at runtime when calling functions
3. **Data Storage** uses account-based key-value storage
4. **Events** notify external systems of state changes
5. **Transaction Emission** allows contracts to send XRP or create transactions
6. **Computation Allowance** limits gas usage per call

---

## Next Steps

1. Study the [Smart Contracts Documentation](./SMART_CONTRACTS.md)
2. Explore more complex data patterns (nested objects, arrays)
3. Learn about cross-contract calls
4. Implement error handling and recovery
5. Optimize gas usage

---

## Troubleshooting

### Build Errors

```bash
# Make sure wasm target is installed
rustup target add wasm32-unknown-unknown

# Clean and rebuild
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

### Deployment Errors

- **temMALFORMED**: Check that all required fields are present
- **temARRAY_TOO_LARGE**: Too many functions or parameters
- **tecINSUFFICIENT_FUNDS**: Need more XRP for fee

### Runtime Errors

- **tecCOMPUTATION_EXCEEDED**: Increase ComputationAllowance
- **INTERNAL_ERROR (-1)**: Check trace logs for details

---

## Additional Resources

- [XRPL Smart Contracts Specification](https://github.com/ripple/rippled)
- [xrpl-wasm-std Documentation](https://github.com/ledger-works/xrpl-wasm-std)
- [More Contract Examples](../contracts/)

Happy coding! 🎉
