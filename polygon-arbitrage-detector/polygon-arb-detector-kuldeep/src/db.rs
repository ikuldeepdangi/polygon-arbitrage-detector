// src/db.rs

use rusqlite::{Connection, params, Result};
use chrono::Utc;

// db file name, maybe move this to config later?
const DB_FILENAME: &str = "arbitrage_log.db3";

// sets up the database connection and makes sure the tables are their
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_FILENAME)?;

    // Create the opportunities table for profitable trades
    // and the price_logs for every check we make
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            buy_dex TEXT NOT NULL, sell_dex TEXT NOT NULL,
            token_pair TEXT NOT NULL,
            amount_in REAL NOT NULL, amount_out REAL NOT NULL, profit REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS price_logs (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            token_pair TEXT NOT NULL,
            buy_dex TEXT NOT NULL,
            sell_dex TEXT NOT NULL,
            profit REAL NOT NULL
        );",
    )?;


    println!("DB connected ok. Tables are ready.");
    Ok(conn)
}


// log a profitable trade to the db
pub fn record_profit_trade(
    db_connection: &Connection,
    buy_dex: &str,
    sell_dex: &str,
    token_pair: &str,
    amount_in: f64,
    amount_out: f64,
    profit_amt: f64 
) -> Result<()> {
    
    let now_timestamp = Utc::now().to_rfc3339();

    db_connection.execute(
        "INSERT INTO opportunities (timestamp, buy_dex, sell_dex, token_pair, amount_in, amount_out, profit)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![now_timestamp, buy_dex, sell_dex, token_pair, amount_in, amount_out, profit_amt],
    )?;

    println!(">>>> PROFIT! logged for {} <<<<", token_pair);
    Ok(())
}


// save the result of a price check
pub fn save_check_to_db(
    c: &Connection, 
    token_pair: &str,
    buy_dex: &str,
    sell_dex: &str,
    profit: f64
) -> Result<()> {
    
    // just insert it, dont need to print anything for this one its too noisy
    c.execute(
        "INSERT INTO price_logs (timestamp, token_pair, buy_dex, sell_dex, profit)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![Utc::now().to_rfc3339(), token_pair, buy_dex, sell_dex, profit],
    )?;

    Ok(())
}