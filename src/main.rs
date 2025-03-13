use {
    solana_sdk::{
        bs58,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::Signer,
        transaction::{MessageHash, SanitizedTransaction, Transaction},
    },
    solana_simulate::{Simulator, SimulatorConfig},
    std::{
        collections::HashSet,
        path::PathBuf,
        str::FromStr,
    },
};

fn simulate_raydium_v4() {
    let config = SimulatorConfig {
        accounts_path: PathBuf::from("./accounts_raydium.json"),
    };

    let simulator = Simulator::new(config);

    let signer = Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap();

    // Create instruction data
    let instruction1_data = bs58::decode("3ipZX7g9NBXycb5v9QjqWwuhh8PxV9WL3HbJRPdURtmm5W1r5t7QtWMbGWB7mQgB8itRgPTMomJoFW7k4WhmYdYLDyWW5WMHN9M2TPGB2xFoTt3tkD87ECGUXNUzp7WskoNcjTtM9nVZMxZDcAGN1GAD82P9vhnSsQKiE5Kh2").into_vec().unwrap();
    let instruction1 = Instruction::new_with_bytes(
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        &instruction1_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("JBxmvDYWetwnND8z1ppEuVWpXjpds77J2DgR2hD4Qmhg").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("JBxmvDYWetwnND8z1ppEuVWpXjpds77J2DgR2hD4Qmhg").unwrap(), false),
        ],
    );

    let instruction2_data = bs58::decode("2").into_vec().unwrap();
    let instruction2 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction2_data,
        vec![
            AccountMeta::new(Pubkey::from_str("JBxmvDYWetwnND8z1ppEuVWpXjpds77J2DgR2hD4Qmhg").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new_readonly(Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap(), false),
        ],
    );

    let instruction3_data = bs58::decode("6FL8fBmJqzqeUnA28wVdrto").into_vec().unwrap();
    let instruction3 = Instruction::new_with_bytes(
        Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap(),
        &instruction3_data,
        vec![
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("A3TiDsQgQFKSLXcj51Jiigm4Fd4F27GGrsXAsHaXh3E1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("EvFmWAGp82Kfenmh8xFzSBGYChtmWXmqqTK9QSWW9BqB").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("A9M4vMERK54sEpGefBVnvxJhJRa9U6tUGbkYgYbjci1B").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("EvFmWAGp82Kfenmh8xFzSBGYChtmWXmqqTK9QSWW9BqB").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("A9M4vMERK54sEpGefBVnvxJhJRa9U6tUGbkYgYbjci1B").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("JBxmvDYWetwnND8z1ppEuVWpXjpds77J2DgR2hD4Qmhg").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("Cg1sa7AgfqVTQYREXGv4KwB9qBq5ymNddGTd1CdShjxZ").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
        ],
    );

    let instruction4_data = bs58::decode("A").into_vec().unwrap();
    let instruction4 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction4_data,
        vec![
            AccountMeta::new(Pubkey::from_str("JBxmvDYWetwnND8z1ppEuVWpXjpds77J2DgR2hD4Qmhg").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
        ],
    );

    // Create transaction
    let message = Message::new(&[instruction1, instruction2,
        instruction3, instruction4], Some(&signer));
    let transaction = Transaction::new_unsigned(
        message,
    );

    // Convert to sanitized transaction
    let sanitized_transaction = SanitizedTransaction::try_create(
        transaction.into(),
        MessageHash::Compute,
        None,
        simulator.clone(),
        &HashSet::new(),
    ).unwrap();

    // Execute transaction simulation
    let simulation_result = simulator.simulate_transaction_unchecked(
        &sanitized_transaction,
        true, // Enable CPI recording
    );

    println!("Simulation logs:");
    for log in  &simulation_result.logs {
        println!("{}", log);
    }

    println!("Simulation result: {:?}", simulation_result.result);
}

fn simulate_orca() {
    let config = SimulatorConfig {
        accounts_path: PathBuf::from("./accounts_orca.json"),
    };

    let simulator = Simulator::new(config);

    let signer = Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap();

    // Create instruction data
    let instruction1_data = bs58::decode("GCfmkK").into_vec().unwrap();
    let instruction1 = Instruction::new_with_bytes(
        Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        &instruction1_data,
        vec![
        ],
    );

    let instruction2_data = bs58::decode("3wUYjApMaGbh").into_vec().unwrap();
    let instruction2 = Instruction::new_with_bytes(
        Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        &instruction2_data,
        vec![
        ],
    );

    let instruction3_data = bs58::decode("3Bxs43ZMjSRQLs6o").into_vec().unwrap();
    let instruction3 = Instruction::new_with_bytes(
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        &instruction3_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT").unwrap(), false),
        ],
    );

    let instruction4_data = bs58::decode("11112mvZGVczwuVuZpoTBF5GSTiWahkvMBdTt2wW3w8AoiX1bGgkBdGYtcYKkBwpY7iKAp").into_vec().unwrap();
    let instruction4 = Instruction::new_with_bytes(
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        &instruction4_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("CUefFboq3aSzQXSWDyBTaReigqUFtFD8d7Z3jmt6Gw43").unwrap(), true),
        ],
    );

    let instruction5_data = bs58::decode("2").into_vec().unwrap();
    let instruction5 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction5_data,
        vec![
            AccountMeta::new(Pubkey::from_str("CUefFboq3aSzQXSWDyBTaReigqUFtFD8d7Z3jmt6Gw43").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap(), false),
        ],
    );

    let instruction6_data = bs58::decode("2").into_vec().unwrap();
    let instruction6 = Instruction::new_with_bytes(
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        &instruction6_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("D7zxvjRy4jmzM4YAQ37RvyBLPZL2Km7Tbc4Pu4SQZ8Pz").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new_readonly(Pubkey::from_str("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("11111111111111111111111111111111").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
        ],
    );

    let instruction7_data = bs58::decode("59p8WydnSZtTGJ1JGxC6BdyeP8VgSoJBdrQZdKaUQ6Wg1R5cx6mmWh7fqz").into_vec().unwrap();
    let instruction7 = Instruction::new_with_bytes(
        Pubkey::from_str("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc").unwrap(),
        &instruction7_data,
        vec![
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("Hxw77h9fEx598afiiZunwHaX3vYu9UskDk9EpPNZp1mG").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("CUefFboq3aSzQXSWDyBTaReigqUFtFD8d7Z3jmt6Gw43").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("DLZSeiq2xjikgwcniQB6B89uodkbQHrTcco6mJu9UNuK").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("D7zxvjRy4jmzM4YAQ37RvyBLPZL2Km7Tbc4Pu4SQZ8Pz").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("F25nyBGmRtMYBC7hw7kqmHWzCkwCzBBGWGpzdg8rHxXV").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("23wrJvTfDLdsqjM17pZ3PEDRJ6BFWJea92Y4FHELsHim").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("AJjVJZzuh1Wvbz7ZcrYRr579K3KR18WSb9L5mECJLxS9").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("DuBdX4KesVdEmoXgNerTWCo9acYQbM7rb4HYar83TUC3").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("JfxLQasRhePQSP1gLt67xYFXdZCXH4vg1yXA3RxuoBU").unwrap(), false),
        ],
    );

    let instruction8_data = bs58::decode("A").into_vec().unwrap();
    let instruction8 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction8_data,
        vec![
            AccountMeta::new(Pubkey::from_str("CUefFboq3aSzQXSWDyBTaReigqUFtFD8d7Z3jmt6Gw43").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
        ],
    );

    // Create transaction
    let message = Message::new(&[instruction1, instruction2,
        instruction3, instruction4, instruction5, instruction6, instruction7, instruction8], Some(&signer));
    let transaction = Transaction::new_unsigned(
        message,
    );

    // Convert to sanitized transaction
    let sanitized_transaction = SanitizedTransaction::try_create(
        transaction.into(),
        MessageHash::Compute,
        None,
        simulator.clone(),
        &HashSet::new(),
    ).unwrap();

    // Execute transaction simulation
    let simulation_result = simulator.simulate_transaction_unchecked(
        &sanitized_transaction,
        true, // Enable CPI recording
    );

    println!("Simulation logs:");
    for log in  &simulation_result.logs {
        println!("{}", log);
    }

    println!("Simulation result: {:?}", simulation_result.result);
}

fn simulate_meteora() {
    let config = SimulatorConfig {
        accounts_path: PathBuf::from("./accounts_meteora.json"),
    };

    let simulator = Simulator::new(config);

    let signer = Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap();

    // Create instruction data
    let instruction1_data = bs58::decode("LUE2QF").into_vec().unwrap();
    let instruction1 = Instruction::new_with_bytes(
        Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        &instruction1_data,
        vec![
        ],
    );

    let instruction2_data = bs58::decode("2").into_vec().unwrap();
    let instruction2 = Instruction::new_with_bytes(
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        &instruction2_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("11111111111111111111111111111111").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),

        ],
    );

    let instruction3_data = bs58::decode("2").into_vec().unwrap();
    let instruction3 = Instruction::new_with_bytes(
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        &instruction3_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("ZJJc42SVCwyb6ZiW6ivVB51jUmEyZb3pLrFhgFAvFdq").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("11111111111111111111111111111111").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),

        ],
    );

    let instruction4_data = bs58::decode("3Bxs4Bc3VYuGVB19").into_vec().unwrap();
    let instruction4 = Instruction::new_with_bytes(
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        &instruction4_data,
        vec![
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN").unwrap(), false),
        ],
    );

    let instruction5_data = bs58::decode("J").into_vec().unwrap();
    let instruction5 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction5_data,
        vec![
            AccountMeta::new(Pubkey::from_str("14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN").unwrap(), false),
        ],
    );

    let instruction6_data = bs58::decode("PgQWtn8oziwxishHjCFfU28PgTi2bgPfu").into_vec().unwrap();
    let instruction6 = Instruction::new_with_bytes(
        Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap(),
        &instruction6_data,
        vec![
            AccountMeta::new(Pubkey::from_str("4kdxjt8pKEW4qV4ji4HANixwswDJw3Egn8L4x2BEWQqT").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("6vwm1hbRj5u8ELuiyLiAmVPnLiQ77XiHjZzpGyrYhQXv").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("3fmSwzBY2TYStZwtdzBmGkdAHcTThsuowSKLVCgqLMXx").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("ZJJc42SVCwyb6ZiW6ivVB51jUmEyZb3pLrFhgFAvFdq").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("AY4mkttNs3vhS5YaPuUY5X54WfuzKoD7HgbPx79DqSUY").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6").unwrap(), false),
            AccountMeta::new_readonly(Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("EUjny9fc82MedgDfC5WM9dC9FFXh5BDrtTuDMAa7RwMs").unwrap(), false),
        ],
    );

    let instruction7_data = bs58::decode("A").into_vec().unwrap();
    let instruction7 = Instruction::new_with_bytes(
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        &instruction7_data,
        vec![
            AccountMeta::new(Pubkey::from_str("14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN").unwrap(), false),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
            AccountMeta::new(Pubkey::from_str("H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ").unwrap(), true),
        ],
    );

    let instruction8_data = bs58::decode("3tGNFMqHiozw").into_vec().unwrap();
    let instruction8 = Instruction::new_with_bytes(
        Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        &instruction8_data,
        vec![
        ],
    );

    // Create transaction
    let message = Message::new(&[instruction1, instruction2,
        instruction3, instruction4, instruction5, instruction6, instruction7, instruction8], Some(&signer));
    let transaction = Transaction::new_unsigned(
        message,
    );

    // Convert to sanitized transaction
    let sanitized_transaction = SanitizedTransaction::try_create(
        transaction.into(),
        MessageHash::Compute,
        None,
        simulator.clone(),
        &HashSet::new(),
    ).unwrap();

    // Execute transaction simulation
    let simulation_result = simulator.simulate_transaction_unchecked(
        &sanitized_transaction,
        true, // Enable CPI recording
    );

    println!("Simulation logs:");
    for log in  &simulation_result.logs {
        println!("{}", log);
    }

    println!("Simulation result: {:?}", simulation_result.result);
}

fn main() {
    simulate_orca();

    simulate_meteora();

    simulate_raydium_v4();
}