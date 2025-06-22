# InfraStaking – Liquid Staking Vault Powered by DePIN Rewards

**InfraStaking** is a unified liquid staking system and enterprise-grade platform built on **Stellar Soroban**. It enables users and institutions to contribute **digital resources** (e.g., bandwidth, GPU, storage) to **DePINs** (Decentralized Physical Infrastructure Networks), earn **reward tokens**, and seamlessly **stake** those tokens into a **liquid vault**—all while maintaining liquidity and composability via **vault tokens (vTokens)**.

## 🚀 What InfraStaking Enables

InfraStaking provides a full pipeline from **digital resource contribution** to **liquid staking**, combining:

1. **DePIN Resource Contribution**
   Users supply physical or digital infrastructure (e.g., GPU, bandwidth) to DePIN protocols like Flux or Render.

2. **Earning Reward Tokens**
   DePINs distribute **reward tokens** (e.g., \$FLUX, \$RENDER) to users based on resource uptime, quality, and availability.

3. **Using Rewards as Base Tokens**
   These earned tokens become the **base tokens** for the **Staking Vault**. Instead of selling or idling them, users stake these tokens.

4. **Receiving Vault Tokens (vTokens)**
   For every staked base token, users receive a liquid **vToken** representing their position, allowing them to stay liquid and compounding.

5. **Restaking & Composability**
   vTokens can be traded, used in DeFi protocols, or restaked in secondary layers like Benqi or Suzaku for additional yield.

### ⚡ **Built on Stellar**

- Fast transaction processing
- Low fees for all operations
- Robust smart contract platform (Soroban)
- Enterprise-grade security and compliance

## 🏗️ Key Features

### 🔧 Smart Contract Vault

- Secure vault for staking any base token
- Auto-deploys a vault token (vToken)
- Tracks total deposits and ensures withdrawal logic

### 💧 Vault Tokens (vTokens)

- Liquid representation of staked assets
- Usable in DeFi, lending, or restaking layers
- Redeemable at any time

### 🔄 Reward-Powered Yield Pipeline

- Base tokens come from DePIN earnings
- Stake rewards instead of selling
- Compound yield across multiple layers

### 🏢 Remote Enterprise Access

- Enterprises can stake, manage, and restake via InfraStaking’s remote terminal
- Zero Web3 complexity, built-in compliance

## 🧬 How It Works (Step by Step)

1. **Provide Digital Resources**

   - Plug GPUs, bandwidth, or nodes into DePIN protocols (e.g., Flux, Render)

2. **Earn Reward Tokens**

   - Receive tokens like \$FLUX or \$RENDER based on contribution

3. **Stake Rewards in Vault**

   - Deposit reward tokens into the StakingVault smart contract

4. **Receive vTokens**

   - For each deposit, get liquid vTokens back (e.g., 1000 \$FLUX → 1000 vFLUX)

5. **Use vTokens Freely**

   - Trade, lend, or stake vTokens elsewhere while still earning

6. **Withdraw Original Tokens**

   - Burn vTokens anytime to get back your base tokens from the vault

## 🔭 Technical Overview

### Smart Contracts

- **Vault Contract (`staking-vault`)**

  - Manages staking, withdrawals, vault token minting

- **Token Contract (`token`)**

  - Used for both base token and vault token (mint, burn, transfer)

## 📦 File Structure

```tree
.
├── Cargo.toml                   # Main project configuration
├── Makefile                     # Build automation scripts
├── rust-toolchain.toml          # Rust version requirements
├── README.md                    # This documentation file
└── contracts/                   # Smart contract implementations
    ├── staking-vault/           # Main staking vault contract
    │   ├── Cargo.toml           # Vault contract dependencies
    │   ├── Makefile             # Vault build commands
    │   └── src/
    │       ├── lib.rs           # Main vault contract logic
    │       ├── token.rs         # Vault token creation utilities
    │       └── test.rs          # Comprehensive test suite
    └── token/                   # Standard token contract
        ├── Cargo.toml           # Token contract dependencies
        ├── Makefile             # Token build commands
        └── src/
            ├── lib.rs           # Token contract entry point
            ├── contract.rs      # Core token functionality
            ├── admin.rs         # Admin management
            ├── allowance.rs     # Token allowance handling
            ├── balance.rs       # Balance management
            ├── metadata.rs      # Token metadata (name, symbol, etc.)
            ├── storage_types.rs # Data structure definitions
            └── test.rs          # Token contract tests
```

## 🧩 InfraStaking Platform: Enterprise-Grade Remote Terminal

Beyond smart contracts, InfraStaking includes a fully automated backend for institutions:

- **Secure Remote Access:** Stake via web terminal with MFA and audit logs
- **Zero DevOps Needed:** No smart contract knowledge required
- **Dynamic Optimization:** Reallocates vTokens based on protocol yield rates
- **Compliance Ready:** Supports eERC and Avacy standards for privacy and regulation
