
---

### **1. Architecture Overview**

1. **MCP Client (AI Agent)**

   * Your client or AI agent that initiates requests and receives responses.

2. **MCP Server (Rust App)**

   * Receives requests from the MCP Client.
   * Internally, it makes RPC calls to the Ethereum node.

3. **Ethereum Node (RPC) – Infura / Alchemy**

   * The server interacts with the Ethereum blockchain through RPC.
   * Provides access to on-chain data such as blocks, transactions, and smart contract states.

4. **Uniswap V2/V3 Smart Contract**

   * The actual on-chain contract.
   * The server can query or interact with it via RPC (e.g., get prices, perform swaps, check liquidity).

**Overall Flow:**
**MCP Client ↔ MCP Server → Ethereum Node → Uniswap Contract**

#### ASCII Diagram:

```
+-------------+          +---------------+
| MCP Client  | <------> | MCP Server    |
| (AI Agent)  |          | (Rust App)    |
+-------------+          +---------------+
                                |
                                | RPC Call
                                v
                        +-------------------+
                        | Ethereum Node (RPC) |
                        | Infura / Alchemy    |
                        +-------------------+
                                |
                                v
                         +--------------+
                         | Uniswap V2/V3 |
                         | Smart Contract |
                         +--------------+
```

---

### **2. MCP Server Flow**

```
           ┌──────────────────────┐
           │      AI Agent        │
           │  (decision making)   │
           └─────────┬────────────┘
                     │  MCP Request
                     ▼
           ┌──────────────────────┐
           │      MCP Server       │
           │  (ServerHandler)     │
           │──────────────────────│
           │ - call_tool()         │
           │ - list_tools()        │
           └───────┬──────────────┘
                   │
       ┌───────────┼─────────────┐
       ▼           ▼             ▼
┌────────────┐ ┌────────────┐ ┌────────────┐
│ BalanceModule│ │ PriceModule │ │ SwapModule │
│  get_balance │ │  get_price │ │simulate_swap│
└────────────┘ └────────────┘ └────────────┘
                   │
                   ▼
          ┌─────────────────┐
          │ Ethereum Node   │
          │  (ethers.rs)    │
          └─────────────────┘
```

**Explanation:**

* **AI Agent**: Sends requests to the MCP Server to retrieve data or execute operations.
* **MCP Server**: Handles requests according to the `ServerHandler` trait and manages context.
* **Tool Modules**: `BalanceModule`, `PriceModule`, and `SwapModule` provide specific functionalities.
* **Ethereum Node**: The underlying blockchain data source; the tool modules interact with it via the provider (`ethers.rs`).

---


