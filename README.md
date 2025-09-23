# Anchor Calculator

This is a simple **calculator contract** built on Solana using **Anchor**.  
It supports basic operations like initializing a number, adding, subtracting, doubling, and halving.  

A **TypeScript client** is also included to interact with the contract.

## Features

- Initialize a number
- Add a number
- Subtract a number
- Double the current number
- Halve the current number

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) installed
- [Yarn](https://classic.yarnpkg.com/lang/en/docs/install/) installed
- Solana CLI installed
- Anchor installed

### Installation & Running

1. Clone the repository:

```bash
git clone https://github.com/yashwankhade5/anchor-calculator.git
cd anchor-calculator
```
2. Install dependencies:

```bash 
yarn install
```
3. Run the tests to deploy and interact with the contract locally:
```bash
anchor test
```
### Handling declare_id mismatch
If you see an error like:
```text
declare_id mismatch
```

#### Follow these steps:

   1. Copy the Program ID that appears in the mismatch error.

   2. Replace the declare_id! in your Rust code with the new program ID:

```bash 
   declare_id!("NEW_PROGRAM_ID_HERE");
```
   3. Update the Anchor.toml file under [programs.localnet]:

```bash 
[programs.localnet]
calculator = "NEW_PROGRAM_ID_HERE"
```
   4. Run the tests again:
   
```bash 
anchor test

```

## Using the Client

The included TypeScript client allows you to interact with the calculator contract.
You can call methods like init, add, sub, double, and halve through the client.