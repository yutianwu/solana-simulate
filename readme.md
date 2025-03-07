# Solana Simulator

## Overview

The Solana Simulator is a tool for testing and simulating Solana programs in a local environment with limited accounts. It allows developers to simulate transactions and observe their effects without a full node, making it ideal for testing and simulation purposes.

For example, if you want to simulate a transaction like the quote for a Raydium swap, you can use the Solana Simulator to simulate the request without a full node.

## Developer Guide

The simulator is implemented in Rust and provides a straightforward API for simulating Solana transactions. Here's how to use it in your code:

### Setting Up the Simulator

As shown in `src/main.rs`, you can create a simulator instance with custom configuration:

```rust
let config = SimulatorConfig {
    accounts_path: PathBuf::from("./accounts.json"),
};

let simulator = Simulator::new(config);
```

The `accounts.json` file should contain the accounts that you want to use in the simulation. You can refer the `accounts.json` file in the example for the format.

### Creating Transactions

The simulator accepts standard Solana transactions. You can create them as follows:

1. Create a signer keypair:
```rust
let signer = Keypair::from_bytes(&keypair_bytes).unwrap();
```

2. Create instructions:
```rust
let instruction = Instruction::new_with_bytes(
    program_id,  
    &instruction_data,  
    vec![  
        AccountMeta::new(account1, true),  
        AccountMeta::new(account2, false),  
        AccountMeta::new_readonly(account3, false),  
    ],
);
```

3. Create and sign a transaction:
```rust
let message = Message::new(&[instruction1, instruction2], Some(&signer.pubkey()));
let transaction = Transaction::new(
    &[&signer],
    message,
    Hash::default(),
);
```

4. Convert to a sanitized transaction:
```rust
let sanitized_transaction = SanitizedTransaction::try_create(
    transaction.into(),
    MessageHash::Compute,
    None,
    simulator.clone(),
    &HashSet::new(),
).unwrap();
```

### Running Simulation

Execute the transaction simulation:

```rust
let simulation_result = simulator.simulate_transaction_unchecked(
    &sanitized_transaction,
    true,  
);
```
