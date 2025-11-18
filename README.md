# MCP Ethereum Tooling Server (Rust)

This project implements a **Model Context Protocol (MCP) server** in Rust that provides Ethereum-related tooling for AI agents.
It exposes balance queries, token price lookup, and Uniswap swap simulation through a simple MCP interface.

The goal is to demonstrate **practical Rust engineering, modular design, and correct usage of Ethereum RPC**.

---

## 1. Overview

The server acts as an MCP-compatible backend.
When an AI agent calls a tool, the server performs on-chain queries via Ethereum RPC.

```
MCP Client → MCP Server → Ethereum RPC → Uniswap Contracts
```

### Architecture Diagram

```
       ┌──────────────────────┐
       │      AI Agent        │
       │  (decision making)   │
       └─────────┬────────────┘
                 │  MCP Request
                 ▼
       ┌──────────────────────-┐
       │      MCP Server       │
       │   (ServerHandler)     │
       │──────────────────────-│
       │ - call_tool()         │
       │ - list_tools()        │
       └─────────┬────────────-┘
                 │
       ┌─────────┼────────────------------─┐
       ▼         ▼                         ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│ BalanceModule │ │  PriceModule  │ │  SwapModule   │
│  get_balance  │ │   get_price   │ │ simulate_swap │
└───────────────┘ └───────────────┘ └───────────────┘
                 │
                 ▼
        ┌─────────────────┐
        │ Ethereum Node   │
        │  (ethers.rs)    │
        └─────────────────┘
```

> **Note:** The server does **not** execute real transactions.
> Swap functionality uses `eth_call` to simulate execution safely.

---

## 2. Provided Tools

### `get_balance`

Queries:

* ETH balance
* ERC20 balance (using ABI + decimals)

### `get_token_price`

* Fetches token price using on-chain Uniswap pool data
* **Note:** External price sources are not included

### `swap_tokens`

* Constructs a Uniswap V2 or V3 swap call and simulates it using `eth_call`
* Returns expected output amount and gas estimate
* **No transaction is broadcast**

---

## 3. Requirements

### Dependencies

* Rust (tokio async runtime)
* `ethers-rs`
* `serde` / `serde_json`
* `tracing` (logging)
* MCP Rust SDK (`rmcp`)

### Environment

Create a `.env` file:

```bash
INFURA_URL=
WALLET_ADDRESS=
USDC=
```

> The private key is only used for constructing simulation transactions, **not broadcasted**.

---

## 4. Running

### Build

```bash
cargo build
```

### Start MCP Server

```bash
cargo run
```

* Starts the MCP server (`main.rs`)
* Runs on Sepolia testnet with real wallets
* ChatGPT or other clients can send requests directly

### Run Test Client (Local Simulation)

```bash
cargo run --bin test_client
```

* Simulates MCP host/client locally
* Sends requests to the running MCP server to test endpoints and responses
* Useful for development or automated tests without deploying the server
* All transactions occur on Sepolia testnet — safe, no production infrastructure required

---

## 5. Example MCP Tool Calls

### Request: get_balance

```json
{
  "method": "call_tool",
  "params": {
    "name": "get_balance",
    "arguments": {
      "address": "0xYourWalletAddress",
      "token": "0xUSDCContractAddress"
    }
  }
}
```

### Response: get_balance

```json
{
  "result": {
    "eth_balance": "0.52",
    "tokens": [
      {
        "symbol": "USDC",
        "balance": "123.45"
      }
    ]
  }
}
```

### Request: get_price (ETH/USD)

```json
{
  "method": "call_tool",
  "params": {
    "name": "get_price",
    "arguments": {
      "token": null
    }
  }
}
```

### Response: get_price (ETH/USD)

```json
{
  "result": {
    "price": "1850.23"
  }
}
```

### Request: swap_tokens

```json
{
  "method": "call_tool",
  "params": {
    "name": "swap_tokens",
    "arguments": {
      "from_token": "ETH",
      "to_token": "USDC",
      "amount_in": "0.001",
      "slippage": 0.5
    }
  }
}
```

### Response: swap_tokens

```json
{
  "result": {
    "amount_out": "1.82",
    "estimated_gas": 21000
  }
}
```

---

## 6. Design Notes

* Uses a single shared Ethereum provider to reduce redundant connections
* Works with real wallets on Sepolia testnet; simulated/test transactions only
* Can be deployed locally or embedded in external clients
* Modular design: Balance / Price / Swap modules can be extended easily

---
