# Phantom Nexus Deployment Guide

## Prerequisites

- Rust 1.75+ with `wasm32-unknown-unknown` target
- Node.js 20+
- Python 3.11+
- Docker & Docker Compose

## Development

### Backend (Rust)
```bash
# Build all crates
cargo build --release

# Run tests
cargo test --all

# Run local node
cargo run --bin nexus-node -- --testnet --port 8545
```

### Frontend
```bash
cd frontend
npm install
npm run dev  # http://localhost:3000
```

### Smart Contracts (WASM)
```bash
cargo build --target wasm32-unknown-unknown --release -p contracts
```

## Docker Deployment

```bash
docker-compose up -d
```

## Configuration

### Node Configuration
```toml
# config/node.toml
[network]
port = 8545
max_peers = 256
stealth_mode = true

[consensus]
algorithm = "PhantomBFT"
block_time_ms = 400
min_validators = 4

[ai]
enable_prediction = true
swarm_agents = 1000
model_path = "models/phantom-v1"

[storage]
ipfs_endpoint = "http://localhost:5001"
arweave_gateway = "https://arweave.net"
```

## Network Endpoints

| Network | RPC | WebSocket | Explorer |
|---------|-----|-----------|----------|
| Testnet | `https://testnet-rpc.phantomnexus.io` | `wss://testnet-ws.phantomnexus.io` | `https://testnet.phantomscan.io` |
| Mainnet | `https://rpc.phantomnexus.io` | `wss://ws.phantomnexus.io` | `https://phantomscan.io` |
