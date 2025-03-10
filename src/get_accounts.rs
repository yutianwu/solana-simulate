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
        "14ryLxgtBbjF6RvdkPb8z4c3R46Dj5WprCVAGtW7EzpN",
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