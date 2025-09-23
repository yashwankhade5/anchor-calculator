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
```markdown
 1) calculator
       Is initialized!:
     Error: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.       
      at AnchorError.parse (node_modules/@coral-xyz/anchor/src/error.ts:136:14)
      at translateError (node_modules/@coral-xyz/anchor/src/error.ts:277:35)
      at MethodsBuilder.rpc [as _rpcFn] (node_modules/@coral-xyz/anchor/src/program/namespace/rpc.ts:35:29)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

  2) calculator
       double:
     Error: Account does not exist or has no data G84G55L4EzqMooQJcrWfdUcmZeRPDBon9BfV2GiWHo6i
      at AccountClient.fetch (node_modules/@coral-xyz/anchor/src/program/namespace/account.ts:168:13)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

  3) calculator
       add :
     Error: Account does not exist or has no data G84G55L4EzqMooQJcrWfdUcmZeRPDBon9BfV2GiWHo6i
      at AccountClient.fetch (node_modules/@coral-xyz/anchor/src/program/namespace/account.ts:168:13)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

  4) calculator
       half:
     Error: Account does not exist or has no data G84G55L4EzqMooQJcrWfdUcmZeRPDBon9BfV2GiWHo6i
      at AccountClient.fetch (node_modules/@coral-xyz/anchor/src/program/namespace/account.ts:168:13)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

  5) calculator
       subtract :
     Error: Account does not exist or has no data G84G55L4EzqMooQJcrWfdUcmZeRPDBon9BfV2GiWHo6i
      at AccountClient.fetch (node_modules/@coral-xyz/anchor/src/program/namespace/account.ts:168:13)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

      error Command failed with exit code 5.
      info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

#### Follow these steps:

   1. Copy the Program ID that appears in the mismatch error.it is prsent in error like "program Id : <programID>"

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

## License

This project is open source and available under the MIT License.



