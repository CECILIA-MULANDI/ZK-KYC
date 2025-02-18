# ProofMe - Zero-Knowledge KYC Verification

## Introduction

ProofMe is a zero-knowledge (ZK) identity verification system that allows users to prove their identity without revealing sensitive personal information. By leveraging ZK proofs, ProofMe ensures privacy, security, and seamless verification for blockchain-based applications.

## Features

- **Zero-Knowledge Proofs:** Users can verify their identity without exposing private details.
- **Privacy-Preserving KYC:** No personal data is stored or shared, only cryptographic proofs.
- **Blockchain Integration:** Supports smart contract-based verification.
- **User-Friendly Interface:** Simple and intuitive experience for users.
- **Secure and Efficient:** Built using Rust and Solidity for performance and security.

## Installation

### 1. Clone the Repository

```sh
git clone https://github.com/CECILIA-MULANDI/ZK-KYC.git
cd ProofMe
```

### 2. Install Dependencies

Ensure you have the following installed:

- Rust
- Foundry for Solidity testing
- Node.js and npm/yarn for frontend development

## Usage

### 1. Compile the zkVM Program

Navigate to the zk program directory and compile:

```sh
cd proofsystems/program
cargo prove build
```

### 2. Execute Proof Generation

Run the program with the RISC-V runtime:

```sh
cd script
RUST_LOG=info cargo run --release -- --execute
```

If execution succeeds, proof generation should also succeed!

### 3. Generate ZK Proofs

Run proof generation:

```sh
cd script
RUST_LOG=info cargo run --release -- --prove
```

## Smart Contract Verification

ProofMe integrates with Solidity smart contracts for on-chain verification. The generated proofs can be submitted to a smart contract for validation.
