#[macro_use]
mod args;

use std::io;
use std::io::Write;
use args::CurrencyArgs;
use clap::Parser;
use serde::Deserialize;
use reqwest::header::HeaderMap;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Deserialize)]
struct Currency {
    amount:i32,
    from:String,
    to:String
}

#[derive(Debug, Deserialize)]
struct Info {
    rate:f64,
}

#[derive(Debug, Deserialize)]
struct Response {
    result:f64,
    info:Info,
    query:Currency
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = CurrencyArgs::parse();
    let amount:String = args.amount.to_string();
    let to:String = args.to.to_string();
    let from:String = args.from.to_string();

    let api_key= env::var("CURRENCY_API_KEY")?;
    let base_url = env::var("CURRENCY_API_BASE_URL")?;

    
    let mut headers = HeaderMap::new();
    headers.insert("apikey", api_key.parse().unwrap());
    
    let query = vec![
        ("from", from),
        ("to", to),
        ("amount", amount)
    ];

    let client = reqwest::Client::new();
    let resp = client
        .get(base_url)
        .headers(headers)
        .query(&query)
        .send()
        .await?;

    let resp_json:Response = resp.json().await?;
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    writeln!(handle,"conversion {} / rate: {} / amount: {} / from: {}  / to:{}", 
        resp_json.result, resp_json.info.rate, resp_json.query.amount, resp_json.query.from, resp_json.query.to
    )?;
    
    Ok(())
}


