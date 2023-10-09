---
sidebar_position: 2
---

# Smart Contract Paradigms 📗

Smart contracts are self-executing, immutable pieces of code that automate and enforce the execution of predefined agreements on a blockchain. These agreements can range from simple transactions to complex decentralized applications (DApps). To better understand the implementations in this project, let's explore three primary smart contract paradigms:

## Account-Based Paradigm

The Account-Based paradigm is exemplified by [Ethereum](https://ethereum.org/en/) and many other blockchain platforms and in this paradigm:

- **State Management**: accounts maintain balances and can execute code. These accounts can be externally owned accounts, controlled by private keys, or contract accounts, controlled by the code of a smart contract.
- **State Transitions**: contracts are executed by sending transactions to their respective addresses. Contracts can store and modify their own state, which is maintained on the blockchain.
- **Flexibility**: account-based systems are highly flexible and can accommodate complex smart contract logic.

Within the Account-Based paradigm, we can further categorize smart contracts into two main types: Stateless and Stateful contracts.

### Stateful Contracts

Stateful contracts maintain and manage their own state. They can store data, update it based on transactions, and execute complex logic. 

In a stateful smart contracts, the contract itself maintains the state of the game and it doesn't rely on external data. Stateful contracts are suitable for applications that require persistent storage and complex business logic, such as games, decentralized finance (DeFi) platforms, and more.

![Stateful example](/img/example_stateful.png)

### Stateless Contracts

Stateless contracts are contracts that do not maintain their own state. Instead, they relie on external accounts to provide data and instructions for their execution.

❗ In stateless contracts, ensuring the security of the system becomes paramount. One security consideration can be the **access controll**: it is crucial to implement robust access control mechanisms to ensure that only authorized entities can modify the data of an account. This often involves verifying the signatures of transactions.

![Stateless example](/img/example_stateless.png)

## UTXO Based Paradigm

![UTXO Transaction Structure](/img/utxo_transacion_structure.PNG)
<p><small>UTXO Transaction Structure. Source: <a href="https://coincodecap.com/utxo">coincodecap.com</a></small></p>

The UTXO (Unspent Transaction Output) paradigm is famously associated with [Bitcoin](https://bitcoin.org/en/) and some other cryptocurrencies. UTXOs are unspent transaction outputs that are created when a transaction is executed and they are consumed when a new transaction is executed. In this paradigm each transaction consumes one or more UTXOs and creates one or more new UTXOs and each UTXO can only be consumed once. UTXO-based systems are very simple. However, they can be less flexible for complex operations.

In the UTXO-based paradigm, the "script" and "witness" are fundamental concepts used to validate transactions. 

- **Script**: a piece of code associated with a UTXO. It defines the conditions that must be met for the UTXO to be spent. In other words, it specifies the rules for how the UTXO can be used in a transaction. 
- **Witness**: provided by the sender of a transaction to satisfy the conditions specified in the script of the UTXO being spent. The witness serves as evidence that the sender has the right to spend the UTXO. It typically includes digital signatures and other data required by the script.

### Witness Constraints

If you want to transfer an UTXO to Bob, you can specify that the witness should be the signature of Alice.

```yaml
# tx1 (Alice)
inputs:
    txA ← ... # txA holds 1:T
    # other inputs
outputs:
    1:T → fun sigA: versig(Bob, rtx, sig)
    # other outputs

# tx2 (Bob)
inputs:
    tx1[0] ← sig_Bob(tx2)
outputs:
    1:T → ...
```

There are also other types of constrains, here some examples:

- enforcing that the script of the `rtx` should be the same as the script of the `ctxo`:
```yaml
# tx
inputs: ...
outputs:
    ... → fun x: rtx[0].script == ctxo[0].script
```
- enforcing that the value of `myVar` of the `rtx` should be the same of the `ctxo`:
```yaml
# tx
inputs: ...
outputs:
    ... → fun x [myVar=3]: rtx[0].myVar == ctxo[0].myVar
```

## Parallelizability

Parallelizability is the ability to execute multiple transactions concurrently. In the context of smart contracts, parallelizability is an important consideration, as it can impact the scalability and efficiency of blockchain systems. 

- **Stateful Contracts**:  **not parallelizable** because they maintain and manage their own state. When multiple transactions attempt to modify the same parts of the state, they can potentially lead to conflicts and race conditions.

- **Stateless Contracts**: **parallelizable**, since they do not maintain their own state and instead rely on external accounts to provide data, they often operate on distinct and isolated parts of the state. 

For example, in [Solana](https://solana.com), for each passed account in an instruction, we need to specify if the account is read-only or writable:

```typescript
const instruction = new TransactionInstruction({
    keys: [
        {pubkey: gameAccount.publicKey, isSigner: true, isWritable: true},         // writable
        {pubkey: gameStateAccount.publicKey, isSigner: false, isWritable: false},  // read-only
    ],
    programId: programId,
    data: // ...,
});
```

- **UTXO-Based Contracts**: also UTXO-based contracts can be highly **parallelizable**. In this paradigm, each transaction consumes specific UTXOs and creates new ones. These UTXOs are  disjoint and do not overlap in terms of state. As a result, multiple transactions can be processed in parallel.
