time_lock
=========

# 1. Introduction
A simple smart contract written with ink! to store coins in a contract for a minimum time. 

# 2. What can I use this for?
- Test swap
- Test behaviors on a machine when there is little memory available

# 3. Installation

## Getting started with Substrate and ink!

Follow this tutorial to install the prequisites to run your own Substrate local development chain, and  get started with ink!: https://substrate.dev/substrate-contracts-workshop/#/0/setup

## Clone the contract to your desired directory

Run the clone command in your terminal to copy the contract files: 
```
$git clone https://github.com/albertov19/time_lock
```

Open the terminal in the local directory where the files are, and run the following commands to build the contract and generate its metadata:
```
cargo +nightly contract build
cargo +nightly contract generate-metadata
```
The .wasm and .json files are copied to a /target/ directory

## Polkadot UI

With a local Substrate Node running, head over to the Polkadot UI and connect it to your local node following this tutorial: https://substrate.dev/substrate-contracts-workshop/#/0/running-a-substrate-node

# 4. Running

In the Polkadot UI:
1.  Upload the .wasm and .json files in the Contract section. 
2. Deploy the contract to the local chain. 
3. Execute the functions as needed

# 5. License

This coded is unaudited, use at your own risk. But if you make millions out of it, share them with me.
