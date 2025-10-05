// src/main.rs

mod db;

use ethers::prelude::*;
use ethers::abi::Abi;
use std::sync::Arc;
use eyre::Result;
use std::time::Duration;
use std::env;

#[derive(Debug, Clone)]
struct Token {
    symbol: String,
    address: Address,
    decimals: i32,
}


// Renamed function to be more generic
async fn get_swap_quote(
    dex_contract: &Contract<Provider<Http>>,
    amount_in: U256,
    path: Vec<Address>,
) -> Result<U256> {
    
    // ethers call to get the amounts out for a swap
    let amounts: Vec<U256> = dex_contract
        .method("getAmountsOut", (amount_in, path))?
        .call()
        .await?;

    // we want the last element in the returned array
    Ok(amounts[1])
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok(); 
    println!("\n Arb Bot Starting Up... ");
    println!("--------------------------");

    // updated to call new db function name
    let db_conn = db::init_db()?;

    // Load bot parameters, use defaults if they are not in the .env file
    let trade_amount: f64 = env::var("TRADE_AMOUNT_USDC")
        .unwrap_or("1000.0".to_string())
        .parse()
        .expect("cant parse TRADE_AMOUNT_USDC");
    
    let min_profit: f64 = env::var("MIN_PROFIT_USDC")
        .unwrap_or("5.0".to_string())
        .parse()
        .expect("cant parse MIN_PROFIT_USDC");

    let poll_interval: u64 = env::var("POLL_INTERVAL_SECS")
        .unwrap_or("10".to_string())
        .parse()
        .expect("cant parse POLL_INTERVAL_SECS");


    // make sure the rpc url is set
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let quickswap_addr_str = env::var("DEX_QUICKSWAP").expect("DEX_QUICKSWAP must be set");
    let sushiswap_addr_str = env::var("DEX_SUSHISWAP").expect("DEX_SUSHISWAP must be set");
    
    let usdc = Token {
        symbol: "USDC".to_string(),
        address: env::var("USDC_ADDRESS")?.parse()?,
        decimals: 6,
    };

    // a list of tokens to check
    let mut token_list: Vec<Token> = Vec::new();
    let tokens_str = env::var("TOKENS_TO_MONITOR").expect("TOKENS_TO_MONITOR must be set in .env");
    
    for symbol_str in tokens_str.split(',') {
        let symbol = symbol_str.trim().to_uppercase();
        if symbol.is_empty() { continue; }

        let address_key = format!("{}_ADDRESS", symbol);
        let address: Address = env::var(&address_key)
            .expect(&format!("{} must be set in .env", address_key))
            .parse()?;
        
        // TODO: maybe find a way to get decimals from chain later?
        let decimals = match symbol.as_str() {
            "WETH" | "WMATIC" | "DAI" => 18,
            "USDC" => 6,
            _ => panic!("Decimals for token {} are not defined in the code.", symbol),
        };

        token_list.push(Token { symbol, address, decimals });
    }

    println!("Monitoring {} token pairs.", token_list.len());

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);
    println!("Polygon network connected.");

    let router_abi_json = r#"[{"inputs":[{"internalType":"uint256","name":"amountIn","type":"uint256"},{"internalType":"address[]","name":"path","type":"address[]"}],"name":"getAmountsOut","outputs":[{"internalType":"uint256[]","name":"amounts","type":"uint256[]"}],"stateMutability":"view","type":"function"}]"#;
    let abi: Abi = serde_json::from_str(router_abi_json)?;

    let quickswap_router = Contract::new(quickswap_addr_str.parse::<Address>()?, abi.clone(), client.clone());
    let sushiswap_router = Contract::new(sushiswap_addr_str.parse::<Address>()?, abi, client.clone());

    let trade_amt_wei = U256::from((trade_amount * 10f64.powi(usdc.decimals)) as u128);

    println!("\n--- Settings ---");
    println!(" - Trade Size: {} USDC", trade_amount);
    println!(" - Min Profit: {} USDC", min_profit);
    println!(" - Interval: {} sec\n", poll_interval);


    loop {
        for token in &token_list {
            let pair_name = format!("{}/{}", token.symbol, usdc.symbol);
            
            // CHECK 1: BUY ON QUICKSWAP, SELL ON SUSHISWAP
            let path_to = vec![usdc.address, token.address];
            let tokens_from_qs = match get_swap_quote(&quickswap_router, trade_amt_wei, path_to).await {
                Ok(amount) => amount,
                Err(e) => {
                    println!("Error QuickSwap check for {}: {}. Skipping.", pair_name, e);
                    continue;
                }
            };

            let path_back = vec![token.address, usdc.address];
            let usdc_from_sushi = get_swap_quote(&sushiswap_router, tokens_from_qs, path_back).await?;
            let final_usdc = usdc_from_sushi.as_u128() as f64 / 10f64.powi(usdc.decimals);
            let profit1 = final_usdc - trade_amount;

            // Updated to call new db function name
            db::save_check_to_db(&db_conn, &pair_name, "QuickSwap", "SushiSwap", profit1)?;

            if profit1 > min_profit {
                // Updated to call new db function name
                db::record_profit_trade(&db_conn, "QuickSwap", "SushiSwap", &pair_name, trade_amount, final_usdc, profit1)?;
            } else {
                 println!("qs -> ss | {} | profit: {:.4}", pair_name, profit1);
            }

            // CHECK 2: BUY ON SUSHISWAP, SELL ON QUICKSWAP
            let path_to2 = vec![usdc.address, token.address];
            let tokens_from_sushi = match get_swap_quote(&sushiswap_router, trade_amt_wei, path_to2).await {
                Ok(amount) => amount,
                Err(e) => {
                    println!("Error SushiSwap check for {}: {}. Skipping.", pair_name, e);
                    continue;
                }
            };
            let path_back2 = vec![token.address, usdc.address];
            let usdc_from_qs = get_swap_quote(&quickswap_router, tokens_from_sushi, path_back2).await?;
            let final_usdc2 = usdc_from_qs.as_u128() as f64 / 10f64.powi(usdc.decimals);
            let profit2 = final_usdc2 - trade_amount;

            // Updated to call new db function name
            db::save_check_to_db(&db_conn, &pair_name, "SushiSwap", "QuickSwap", profit2)?;

            if profit2 > min_profit {
                 // Updated to call new db function name
                db::record_profit_trade(&db_conn, "SushiSwap", "QuickSwap", &pair_name, trade_amount, final_usdc2, profit2)?;
            } else {
                 println!("ss -> qs | {} | profit: {:.4}", pair_name, profit2);
            }
        }
        
        println!("--- Cycle done, sleeping. ---");
        tokio::time::sleep(Duration::from_secs(poll_interval)).await;
    }
}