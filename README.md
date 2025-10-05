# Polygon Arbitrage Opportunity Detector

A high-performance bot built in Rust that detects and logs potential arbitrage opportunities on the Polygon (Matic) network. The bot monitors multiple token pairs across QuickSwap and SushiSwap to identify and record price discrepancies for analysis.

---

## Features

-   **Multi-DEX Monitoring**: Checks prices on both QuickSwap and SushiSwap.
-   **Dynamic Multi-Token Support**: Monitors a list of tokens (e.g., WETH, WMATIC, DAI) against a base currency (USDC), configured entirely from the `.env` file.
-   **Fully Configurable**: All critical parameters (RPC URLs, addresses, trade amounts, poll interval) are managed via a `.env` file for easy changes without recompiling.
-   **Comprehensive Database Logging**:
    -   Logs every single price check to a `price_logs` table for detailed historical analysis.
    -   Logs only profitable opportunities to an `opportunities` table for high-signal alerts.
-   **Asynchronous Architecture**: Built with Tokio and `ethers-rs` for efficient, non-blocking network requests to the blockchain.

---

## Technology Stack

-   **Programming Language**: Rust
-   **Blockchain Interaction**: `ethers-rs`
-   **Asynchronous Runtime**: `tokio`
-   **Database**: SQLite (via `rusqlite`)
-   **Configuration Management**: `dotenvy`

---

## Project Structure

Of course. Adding a "Future Scope" section is a great way to show vision for the project. I've taken your ideas and added a few more to create a professional-looking section.

I'll provide the complete, updated README.md content. You can simply replace the entire text of your current README.md file with this new version. I have added the new Future Scope section and also included the Disclaimer section at the end.

Final README.md Content (Copy and Paste This)
Markdown

# Polygon Arbitrage Opportunity Detector

A high-performance bot built in Rust that detects and logs potential arbitrage opportunities on the Polygon (Matic) network. The bot monitors multiple token pairs across QuickSwap and SushiSwap to identify and record price discrepancies for analysis.

---

## Features

-   **Multi-DEX Monitoring**: Checks prices on both QuickSwap and SushiSwap.
-   **Dynamic Multi-Token Support**: Monitors a list of tokens (e.g., WETH, WMATIC, DAI) against a base currency (USDC), configured entirely from the `.env` file.
-   **Fully Configurable**: All critical parameters (RPC URLs, addresses, trade amounts, poll interval) are managed via a `.env` file for easy changes without recompiling.
-   **Comprehensive Database Logging**:
    -   Logs every single price check to a `price_logs` table for detailed historical analysis.
    -   Logs only profitable opportunities to an `opportunities` table for high-signal alerts.
-   **Asynchronous Architecture**: Built with Tokio and `ethers-rs` for efficient, non-blocking network requests to the blockchain.

---

## Technology Stack

-   **Programming Language**: Rust
-   **Blockchain Interaction**: `ethers-rs`
-   **Asynchronous Runtime**: `tokio`
-   **Database**: SQLite (via `rusqlite`)
-   **Configuration Management**: `dotenvy`

---

## Project Structure
.
├── .env          # Your configuration file (must be created)
├── .gitignore    # Specifies files for Git to ignore
├── Cargo.toml    # Project dependencies and metadata
├── README.md     # This file
└── src/
├── main.rs   # Main application entrypoint and arbitrage logic
└── db.rs     # Database setup and logging functions


---

## Setup and Installation

### 1. Clone the Repository
Clone this repository to your local machine.

```bash
git clone [https://github.com/ikuldeepdangi/polygon-arbitrage-detector.git](https://github.com/ikuldeepdangi/polygon-arbitrage-detector.git)
cd polygon-arbitrage-detector/polygon-arbitrage-detector-kuldeep
2. Create the Configuration File
The bot requires a .env file for its configuration. Create a new file named .env in the project root and add the following content.

You must provide your own private Alchemy RPC URL.

Code snippet

# .env

# Your private Alchemy RPC URL for the Polygon Mainnet
RPC_URL="YOUR_ALCHEMY_POLYGON_MAINNET_URL_HERE"

# DEX Router Addresses
DEX_QUICKSWAP="0xa5E0829CaCED8ffDd4De3c43696c57F7D7A678ff"
DEX_SUSHISWAP="0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506"

# --- Token Configuration ---
USDC_ADDRESS="0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
TOKENS_TO_MONITOR="WETH,WMATIC,DAI"
WETH_ADDRESS="0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"
WMATIC_ADDRESS="0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270"
DAI_ADDRESS="0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063"

# --- Bot Parameters (Optional, will use defaults if not set) ---
TRADE_AMOUNT_USDC=1000.0
MIN_PROFIT_USDC=5.0
POLL_INTERVAL_SECS=10
3. Build the Project
Use Cargo to download all dependencies and build an optimized executable.

Bash

cargo build --release
Running the Bot
Once the project is built, you can run it with Cargo:

Bash

cargo run --release
The bot will start, connect to the database and the Polygon network, and begin logging price checks to the console. A database file named arbitrage_log.db3 will be created in the project directory.

Database Schema
The bot creates and uses an SQLite database file (arbitrage_log.db3) with two tables:

opportunities Table
Stores only profitable trades that exceed the MIN_PROFIT_USDC threshold.

Column	Type	Description
id	INTEGER	Primary Key
timestamp	TEXT	ISO 8601 timestamp of the event
buy_dex	TEXT	The DEX where the initial buy occurred
sell_dex	TEXT	The DEX where the subsequent sell occurred
token_pair	TEXT	The token pair involved (e.g., "WETH/USDC")
amount_in	REAL	The initial USDC amount for the trade
amount_out	REAL	The final USDC amount after both swaps
profit	REAL	The calculated profit in USDC

Export to Sheets
price_logs Table
Stores every single price check performed by the bot for historical analysis.

Column	Type	Description
id	INTEGER	Primary Key
timestamp	TEXT	ISO 8601 timestamp of the check
token_pair	TEXT	The token pair involved
buy_dex	TEXT	The simulated buy DEX
sell_dex	TEXT	The simulated sell DEX
profit	REAL	The calculated profit/loss in USDC

Export to Sheets
Future Scope
This project provides a solid foundation for a more advanced arbitrage system. Potential future enhancements include:

Gas Cost Simulation: Implement logic to fetch current gas prices on the Polygon network and subtract the estimated transaction fees from the potential profit for a more accurate simulation.

Real-time Notifications: Integrate services like SendGrid (for email) or a Telegram Bot API to send instant alerts when a profitable opportunity is logged.

Web Dashboard: Develop a web-based dashboard (e.g., using a Rust web framework like Axum or Actix-web) to visualize the data from the price_logs table, showing live price differences and historical trends.

Semi-Automated Execution: Add functionality to securely manage a private key. When an opportunity is found, the system could prompt a user for one-click approval via the dashboard to sign and execute the necessary transactions.

Expanded DEX Support: Integrate more DEX aggregators and routers on Polygon (e.g., 1inch, ParaSwap) to find more arbitrage paths.
