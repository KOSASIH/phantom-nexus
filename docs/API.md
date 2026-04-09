# Phantom Nexus API Reference

## RPC Endpoints

### Chain
```
POST /rpc/v1
```

#### Methods

##### `phnx_getBlockNumber`
Returns the latest block number.
```json
{"jsonrpc":"2.0","method":"phnx_getBlockNumber","params":[],"id":1}
```

##### `phnx_getBlock`
Returns block by number or hash.
```json
{"jsonrpc":"2.0","method":"phnx_getBlock","params":["0x1",true],"id":1}
```

##### `phnx_sendTransaction`
Submit a signed transaction.
```json
{"jsonrpc":"2.0","method":"phnx_sendTransaction","params":[{"from":"0x...","to":"0x...","value":"0x...","data":"0x..."}],"id":1}
```

##### `phnx_getBalance`
Get account balance.
```json
{"jsonrpc":"2.0","method":"phnx_getBalance","params":["0x...","latest"],"id":1}
```

### AI Oracle
```
POST /oracle/v1
```

##### `oracle_predict`
Get AI market prediction.
```json
{"jsonrpc":"2.0","method":"oracle_predict","params":{"asset":"BTC","timeframe":"24h"},"id":1}
```

##### `oracle_dataFeed`
Query real-time data feed.
```json
{"jsonrpc":"2.0","method":"oracle_dataFeed","params":{"feed_id":"price/btc-usd"},"id":1}
```

### DeFi Vaults
```
POST /defi/v1
```

##### `vault_deposit`
Deposit into a Phantom Vault.
```json
{"jsonrpc":"2.0","method":"vault_deposit","params":{"vault_id":"phantom-yield-v1","amount":"1000000000000000000"},"id":1}
```

##### `vault_withdraw`
Withdraw from a Phantom Vault.
```json
{"jsonrpc":"2.0","method":"vault_withdraw","params":{"vault_id":"phantom-yield-v1","shares":"500000000000000000"},"id":1}
```

## WebSocket API

```
ws://node.phantomnexus.io/ws/v1
```

### Subscriptions
- `phnx_subscribe("newBlocks")` — New block headers
- `phnx_subscribe("pendingTx")` — Pending transactions
- `phnx_subscribe("logs", {filter})` — Contract event logs
- `oracle_subscribe("feed", {feed_id})` — Real-time oracle data

## SDK

### JavaScript/TypeScript
```bash
npm install @phantom-nexus/sdk
```

```typescript
import { PhantomNexus } from '@phantom-nexus/sdk';

const nexus = new PhantomNexus('https://rpc.phantomnexus.io');
const balance = await nexus.getBalance('0x...');
```

### Rust
```toml
[dependencies]
phantom-nexus-sdk = "0.1"
```

```rust
use phantom_nexus_sdk::Client;

let client = Client::new("https://rpc.phantomnexus.io");
let block = client.get_latest_block().await?;
```
