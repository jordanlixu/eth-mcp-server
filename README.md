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
