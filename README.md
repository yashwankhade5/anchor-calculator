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

(chor-calculator/target/debug/deps/calculator-a380ed6673537925)
Deploying cluster: http://127.0.0.1:8899
Upgrade authority: /home/toor/.config/solana/id.json
Deploying program "calculator"...
Program path: /mnt/d/harkirat anchor contract/week_38/New folder/anchor-calculator/target/deploy/calculator.so...
Program Id: AaK6vJkBcHU613578v32Z9aZwa7uVyoeTxbmqhc7fLB7                                

Signature: 56brPgy27BitAR6EJQhkShYX22EwsCt5t7YKn1LZySBbYqPTJpisDHEHUhFmVfYuNKEzGHZZXGxpXhszsYKq8tk2

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/mnt/d/harkirat anchor contract/week_38/New folder/anchor-calculator/Anchor.toml"

yarn run v1.22.22
$ '/mnt/d/harkirat anchor contract/week_38/New folder/anchor-calculator/node_modules/.bin/ts-mocha' -p ./tsconfig.json -t 1000000 'tests/**/*.ts'
(node:11306) [MODULE_TYPELESS_PACKAGE_JSON] Warning: Module type of file:///mnt/d/harkirat%20anchor%20contract/week_38/New%20folder/anchor-calculator/tests/calculator.ts is not specified and it doesn't parse as CommonJS.
Reparsing as ES module because module syntax was detected. This incurs a performance overhead.
To eliminate this warning, add "type": "module" to /mnt/d/harkirat anchor contract/week_38/New folder/anchor-calculator/package.json.
(Use `node --trace-warnings ...` to show where the warning was created)


  calculator
    1) Is initialized!
    2) double
    3) add
    4) half
    5) subtract 


  0 passing (200ms)
  5 failing

  1) calculator
       Is initialized!:
     Error: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.       
      at AnchorError.parse (node_modules/@coral-xyz/anchor/src/error.ts:136:14)
      at translateError (node_modules/@coral-xyz/anchor/src/error.ts:277:35)
      at MethodsBuilder.rpc [as _rpcFn] (node_modules/@coral-xyz/anchor/src/program/namespace/rpc.ts:35:29)
      at processTicksAndRejections (node:internal/process/task_queues:105:5)

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



