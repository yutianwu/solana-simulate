use {
    reqwest::{Client, header::HeaderMap},
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    std::{fs, time::Duration},
    tokio::time,
};

#[derive(Debug, Serialize, Deserialize)]
struct AccountInfo {
    data: Vec<Value>,
    executable: bool,
    lamports: u64,
    owner: String,
    #[serde(rename = "rentEpoch")]
    rent_epoch: u64,
    space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountResponse {
    accounts: Vec<AccountEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountEntry {
    pubkey: String,
    account: AccountInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct RPCRequest {
    jsonrpc: String,
    id: i32,
    method: String,
    params: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RPCResponse {
    jsonrpc: String,
    result: RPCResult,
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct RPCResult {
    context: Context,
    value: AccountInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    #[serde(rename = "apiVersion")]
    api_version: String,
    slot: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let accounts = vec![

        "H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ",
        "A3TiDsQgQFKSLXcj51Jiigm4Fd4F27GGrsXAsHaXh3E1",
        "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
        "EvFmWAGp82Kfenmh8xFzSBGYChtmWXmqqTK9QSWW9BqB",
        "A9M4vMERK54sEpGefBVnvxJhJRa9U6tUGbkYgYbjci1B",
        "Cg1sa7AgfqVTQYREXGv4KwB9qBq5ymNddGTd1CdShjxZ",
        "So11111111111111111111111111111111111111112",
        "SysvarRent111111111111111111111111111111111",
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
        "A7ZG7ByDi8DpzT9Ab7CiXhvgYTJQmaDPJkMDoPitaCQV",

        // "H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ",
        // "D7zxvjRy4jmzM4YAQ37RvyBLPZL2Km7Tbc4Pu4SQZ8Pz",
        // "Hxw77h9fEx598afiiZunwHaX3vYu9UskDk9EpPNZp1mG",
        // // "ComputeBudget111111111111111111111111111111",
        // // "11111111111111111111111111111111",
        // "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        // "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        // "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE",
        // "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
        // "CtXfPzz36dH5Ws4UYKZvrQ1Xqzn42ecDW6y8NKuiN8nD",
        // "SysvarRent111111111111111111111111111111111",
        // "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT",
        // "DLZSeiq2xjikgwcniQB6B89uodkbQHrTcco6mJu9UNuK",
        // "F25nyBGmRtMYBC7hw7kqmHWzCkwCzBBGWGpzdg8rHxXV",
        // "23wrJvTfDLdsqjM17pZ3PEDRJ6BFWJea92Y4FHELsHim",
        // "AJjVJZzuh1Wvbz7ZcrYRr579K3KR18WSb9L5mECJLxS9",
        // "DuBdX4KesVdEmoXgNerTWCo9acYQbM7rb4HYar83TUC3",

        // "H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ",
        // "4kqUgKUi4iuZQWFnqTW4weoRrvwq67FcdCVoSKcWmSzJ",
        // "59YuGWPunbchD2mbi9U7qvjWQKQReGeepn4ZSr9zz9Li",
        // "5rCf1DM8LjKTw4YqhnoLcngyZYeNnQqztScTogYHAS6",
        // "CoaxzEh8p5YyGLcj36Eo3cUThVJxeKCs7qvLAGDYwBcz",
        // "Ehkf9XQLVnY8HV6jbbDU25fTxF1qQ3NuScWfawSb79pu",
        // "EYj9xKw6ZszwpyNibHY7JD5o3QgTVrSdcBp1fMJhrR9o",
        // "11111111111111111111111111111111",
        // "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        // "ComputeBudget111111111111111111111111111111",
        // "D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6",
        // "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        // "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
        // "So11111111111111111111111111111111111111112",
        // "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        // "HZcJwcJ2njPDxZtpPoKnF8v2w9QAx2rS7TdJPSRkbEhu",

        // "H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ",
        // "3fmSwzBY2TYStZwtdzBmGkdAHcTThsuowSKLVCgqLMXx",
        // "4kdxjt8pKEW4qV4ji4HANixwswDJw3Egn8L4x2BEWQqT",
        // "6vwm1hbRj5u8ELuiyLiAmVPnLiQ77XiHjZzpGyrYhQXv",
        // "AY4mkttNs3vhS5YaPuUY5X54WfuzKoD7HgbPx79DqSUY",
        // "EUjny9fc82MedgDfC5WM9dC9FFXh5BDrtTuDMAa7RwMs",
        // "ZJJc42SVCwyb6ZiW6ivVB51jUmEyZb3pLrFhgFAvFdq",
        // "11111111111111111111111111111111",
        // "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh",
        // "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        // "ComputeBudget111111111111111111111111111111",
        // "D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6",
        // "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
        // "So11111111111111111111111111111111111111112",
        // "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        //
        // "H7GCUaJMUgdQiNYyoQTTmwG4fSYMV8W8ECmATZ2kyNTJ",
        // "4kqUgKUi4iuZQWFnqTW4weoRrvwq67FcdCVoSKcWmSzJ",
        // "59YuGWPunbchD2mbi9U7qvjWQKQReGeepn4ZSr9zz9Li",
        // "5rCf1DM8LjKTw4YqhnoLcngyZYeNnQqztScTogYHAS6",
        // "CoaxzEh8p5YyGLcj36Eo3cUThVJxeKCs7qvLAGDYwBcz",
        // "Ehkf9XQLVnY8HV6jbbDU25fTxF1qQ3NuScWfawSb79pu",
        // "EYj9xKw6ZszwpyNibHY7JD5o3QgTVrSdcBp1fMJhrR9o",
        // "11111111111111111111111111111111",
        // "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        // "ComputeBudget111111111111111111111111111111",
        // "D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6",
        // "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        // "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
        // "So11111111111111111111111111111111111111112",
        // "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        // "HZcJwcJ2njPDxZtpPoKnF8v2w9QAx2rS7TdJPSRkbEhu",
        // "SysvarRent111111111111111111111111111111111"

    ];

    let mut response = AccountResponse {
        accounts: Vec::new(),
    };

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    for acc in accounts {
        time::sleep(Duration::from_secs(1)).await;

        let rpc_req = RPCRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getAccountInfo".to_string(),
            params: vec![
                json!(acc),
                json!({
                    "encoding": "base64"
                }),
            ],
        };

        let res = client
            .post("https://api.mainnet-beta.solana.com")
            .headers(headers.clone())
            .json(&rpc_req)
            .send()
            .await?;
        println!("{}", acc);
        let body = res.text().await?;
        println!("{}", body);

        let rpc_resp: RPCResponse = serde_json::from_str(&body)?;

        response.accounts.push(AccountEntry {
            pubkey: acc.to_string(),
            account: rpc_resp.result.value,
        });
    }

    let json_data = serde_json::to_string_pretty(&response)?;
    fs::write("accounts.json", json_data)?;

    println!("Account information has been written to accounts.json");
    Ok(())
} 