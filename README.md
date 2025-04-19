# Timelock Claimable Balance Contract

A simple and functional smart contract implementation for the Soroban platform that demonstrates the "timelock" concept and provides a streamlined "Claimable Balance" mechanism.

# Build

stellar contract build 

- ℹ️  Build Summary:
   - Wasm File: target\wasm32-unknown-unknown\release\soroban_timelock.wasm
   - Wasm Hash: 9f14263759db2e33b4bb57d039c9425e4ae516133c209740f7d973e916dc4745
   - Exported Functions: 3 found
     - _
     - claim
     - deposit
- ✅ Build Complete

# Deploy

wasm-opt -Oz -o optimized.wasm target/wasm32-unknown-unknown/release/soroban_timelock.wasm

wasm-validate optimized.wasm

stellar contract deploy --wasm optimized.wasm --source-account alice --network testnet --alias timelock

- ℹ️  Simulating install transaction…
- ℹ️  Signing transaction: 0d2150eb69e8cf1fe4c4c220c805dea8bfbacff6b1050a17744b60b537e334a0
- 🌎 Submitting install transaction…
- ℹ️  Using wasm hash 6e59c0ec6a47fba5d9d41461fc643c229b62de12e03292bfd1d51a996d23146c
- ℹ️  Simulating deploy transaction…
- ℹ️  Transaction hash is 4b8adc00e5d7ead13288530ce2999766af53ea1677a1368155fdf6dc45117169
- 🔗 https://stellar.expert/explorer/testnet/tx/4b8adc00e5d7ead13288530ce2999766af53ea1677a1368155fdf6dc45117169
- ℹ️  Signing transaction: 4b8adc00e5d7ead13288530ce2999766af53ea1677a1368155fdf6dc45117169
- 🌎 Submitting deploy transaction…
- 🔗 https://stellar.expert/explorer/testnet/contract/CCVRMFZMVQ24H2SJJEMD6R7PW5ROD5OC4HZL2GKHHFZ4RK7LTVAXGGRL
- ✅ Deployed!
- CCVRMFZMVQ24H2SJJEMD6R7PW5ROD5OC4HZL2GKHHFZ4RK7LTVAXGGRL

## Overview

This contract allows users to deposit a specified amount of tokens and enables other accounts to claim them either before or after a specific time point. The contract implements a basic timelock mechanism while maintaining a clean and efficient design.

## Features

- Deposit tokens with specified time conditions
- Set multiple claimant addresses (up to 10)
- Two types of time bounds:
  - `Before`: Claimable only before a specified timestamp
  - `After`: Claimable only after a specified timestamp
- Secure authorization checks
- Simple one-time initialization

## Contract Structure

### Data Structures

- **DataKey**: Defines storage keys used in the contract
  - `Init`: Marks the contract as initialized
  - `Balance`: Stores claimable balance information

- **TimeBoundKind**: Specifies the type of time constraint
  - `Before`: Claimable before the timestamp
  - `After`: Claimable after the timestamp

- **TimeBound**: Combines the time bound type and its timestamp
  - `kind`: The type of time bound
  - `timestamp`: The specific timestamp value

- **ClaimableBalance**: Stores all details of the claimable balance
  - `token`: Address of the token
  - `amount`: Token amount
  - `claimants`: List of addresses that can claim the balance
  - `time_bound`: Time condition for claiming

## Technical Details

- Built using the Soroban SDK for Stellar's smart contract platform
- Implemented with `#![no_std]` for efficient operation in blockchain environments
- Uses Rust's type system for safety and reliability

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
