# ProClean Loyalty DApp

**ProClean Loyalty DApp** - Blockchain-Based Decentralized Customer Reward System

## Project Description

ProClean Loyalty DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, transparent, and immutable platform for managing customer loyalty points for vehicle wash and service businesses. The contract ensures that customer service records are stored reliably and rewards are triggered automatically without relying on a vulnerable centralized database.

The system allows staff to log customer visits, automatically track when a customer reaches the threshold for a free service (5 visits), and securely claim the reward. Each customer's status is uniquely tracked within the contract's storage, ensuring data persistence and eliminating point fraud.

## Project Vision

Our vision is to revolutionize customer retention in the automotive service industry by:

- **Decentralizing Loyalty**: Moving point systems from easily manipulated local databases to a global, distributed blockchain.
- **Ensuring Transparency**: Empowering customers and business owners with a clear, immutable record of service history.
- **Automating Rewards**: Providing a trustless mechanism where rewards (free services) are guaranteed by code once conditions are met.
- **Enhancing Security**: Leveraging blockchain technology to prevent unauthorized modifications to customer points.

We envision a future where business-customer relationships are strengthened through transparent, automated, and sovereign digital loyalty assets.

## Key Features

### 1. **Automated Point Tracking**
- Log customer visits with a single function call (`add_service`).
- Automatically increment service counts for unique customer IDs.
- Persistent and immutable point storage on the Stellar blockchain.

### 2. **Smart Reward Logic**
- The contract automatically checks point thresholds (e.g., 5 visits).
- Status automatically updates to grant a "Free Service" when the threshold is reached.
- Eliminates manual calculation and potential human error.

### 3. **Secure Reward Redemption**
- Claim rewards securely using the `claim_reward` function.
- Automatically resets the customer's point counter back to zero upon successful redemption.
- Prevents double-claiming of rewards.

### 4. **Stellar Network Integration**
- Leverages the high speed and near-zero fees of the Stellar network.
- Built using the modern, Rust-based Soroban Smart Contract SDK.
- Highly scalable for growing customer bases and multiple franchise branches.

## Contract Details

- **Network:** Stellar Testnet
- **Contract Address:** `CAZ57U4TEYCGX6Z4KFI3PPJNPKZWI6XBYV7IESJNBEH7NYXXQDADK3PU`

![ProClean Soroban Studio](INSERT_YOUR_SCREENSHOT_HERE.png)
*(Note: Replace INSERT_YOUR_SCREENSHOT_HERE.png with the actual filename of your Soroban Studio screenshot)*

## Future Scope

### Short-Term Enhancements
1. **Customer Dashboard**: A web frontend for customers to check their own points using their wallet address.
2. **Tiered Rewards**: Implementing VIP tiers (Silver, Gold, Platinum) with different reward thresholds.
3. **Multi-Branch Support**: Adding branch IDs to track where the service was provided.

### Medium-Term Development
4. **Tokenized Rewards**: Issuing custom Stellar tokens (e.g., $CLEAN) instead of simple point counters.
5. **NFT Memberships**: Issuing exclusive NFTs for customers who reach 50+ visits.
6. **Partner Integration**: Allowing points to be redeemed at partnered cafes or waiting lounges.

### Long-Term Vision
7. **Cross-Chain Loyalty**: Allowing points to be bridged or recognized across different decentralized networks.
8. **DAO Governance**: Allowing loyal customers to vote on future promotions or services through a DAO.

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network (Testnet)

## Getting Started

Interact with the deployed smart contract using the Stellar CLI:

- `add_service(customer_id)` - Increment points for a specific customer.
- `claim_reward(customer_id)` - Redeem the free service and reset the counter.

---

**ProClean Loyalty DApp** - Trustless Rewards on the Blockchain