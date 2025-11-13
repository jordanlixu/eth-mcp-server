### Architecture Description

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

---

### ASCII Diagram

```
+-------------+          +---------------+
| MCP Client  | <------> | MCP Server    |
| (AI Agent)  |          | (Your Rust App) |
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

