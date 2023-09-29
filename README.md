# TicTacToe in 3 Smart Contracts Paradigms 🧩

## Introduction 👋

Hello, in this repository, we explore the three fundamental smart contract paradigms: UTXO-based, Account-Based (Stateless and Stateful).

The aim of this project is to provide a comprehensive understanding of these smart contract paradigms and their real-world implications. By scaffolding a familiar game (Tic Tac Toe) in each paradigm, we offer practical insights into the differences, challenges, and advantages of each approach.

## Smart Contract Paradigms

Smart contracts are self-executing, immutable pieces of code that automate and enforce the execution of predefined agreements on a blockchain. These agreements can range from simple transactions to complex decentralized applications (DApps). To better understand the implementations in this project, let's explore three primary smart contract paradigms:

### Account-Based Paradigm

The Account-Based paradigm, exemplified by Ethereum and many other blockchain platforms, operates differently from the UTXO model. 

In this paradigm:

- **State Management**: accounts maintain balances and can execute code. These accounts can be externally owned accounts, controlled by private keys, or contract accounts, controlled by the code of a smart contract.
- **State Transitions**: contracts are executed by sending transactions to their respective addresses. Contracts can store and modify their own state, which is maintained on the blockchain.
- **Flexibility**: account-based systems are highly flexible and can accommodate complex smart contract logic.

Within the Account-Based paradigm, we can further categorize smart contracts into two main types: Stateless and Stateful contracts.

#### Stateful Contracts

Stateful contracts are contracts that maintain and manage their own state. They can store data, update it based on transactions, and execute complex logic.
Stateful contracts are suitable for applications that require persistent storage and complex business logic, such as games, decentralized finance (DeFi) platforms, and more.

#### Stateless Contracts

Stateless contracts are contracts that do not maintain their own state. They execute deterministic functions based solely on their input parameters and the blockchain's current state that is stored in other accounts and passed to contracts as input.

### UTXO Based Paradigm

The UTXO (Unspent Transaction Output) based paradigm is famously associated with Bitcoin and some other cryptocurrencies. 

UTXOs are unspent transaction outputs that are created when a transaction is executed and they are consumed when a new transaction is executed. 

In this paradigm each transaction consumes one or more UTXOs and creates one or more new UTXOs and each UTXO can only be consumed once.

UTXO-based systems are very simple. However, they can be less flexible for complex operations.

Understanding these smart contract paradigms is crucial when developing blockchain applications, as they influence how contracts handle state, transitions, and security. In the following sections, we'll explore how each of these paradigms has been applied to our Tic Tac Toe game, providing real-world examples of their capabilities and limitations.

## Tic Tac Toe 🧩

In this section, we provide an overview of the Tic Tac Toe game implemented in three different smart contract paradigms: UTXO-Based, Account-Based (Stateless), and Account-Based (Stateful). Each implementation serves as a practical demonstration of how these paradigms handle state, transitions, and logic execution.

> Don't worry, we won't bore you with a tutorial on how to play Tic Tac Toe – we trust that you've all mastered it during those 'productive' school days when lessons just couldn't compete with this classic game. 😉

### Account Based (Stateful) Implementation

Account-Based smart contracts, when stateful, have the capability to maintain and modify their own internal state.

### Account Based (Stateless) Implementation

The Account-Based paradigm, in its stateless form, allows for executing smart contract functions without maintaining any internal state.

### UTXO Based Implementation

In the UTXO-Based paradigm, we adapted the Tic Tac Toe game to utilize the unique principles of this model. 

## Contributing 🙌

We welcome contributions to this project! If you're interested, here is an idea: note that while we have implemented Tic Tac Toe game for Ethereum using solidity (Statefull) and for Solana using Rust (Stateless), we currently only have a scaffolded pseudocode for the UTXO-based paradigm.

## Acknowledgments 🙏

This project drew inspiration from the speakers and content of the [4th Scientific School on Blockchain & Distributed Ledger Technologies](https://dlt-school.github.io) held at the University of Cagliari.

## License 📜

This project is licensed under the [MIT License](LICENSE).
