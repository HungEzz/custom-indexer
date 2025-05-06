# Cetus Events Indexer

This indexer tracks and stores Cetus protocol events on the Sui blockchain, specifically swap events, add liquidity events, and remove liquidity events. The data is stored in a PostgreSQL database and exposed through a REST API.

## Features

- Tracks Cetus swap events
- Tracks Cetus add liquidity events
- Tracks Cetus remove liquidity events
- Processes blockchain checkpoints in parallel
- Stores processed data in PostgreSQL
- Exposes data through a REST API

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database
- Diesel CLI (for migrations): `cargo install diesel_cli --no-default-features --features postgres`

## Database Setup

1. Create a PostgreSQL database
2. Run diesel migrations:

```bash
diesel migration run
```

## Environment Configuration

Create a `.env` file in the project root directory with the following variables:

```
DATABASE_URL=postgres://username:password@localhost:5432/database_name
REMOTE_STORAGE=https://checkpoints.mainnet.sui.io
BACKFILL_PROGRESS_FILE_PATH=/path/to/backfill_progress/file
CHECKPOINTS_DIR=/path/to/checkpoints/dir

# Cetus event type overrides (uncomment to use custom event types)
# SWAP_EVENT_TYPE=0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent
# ADD_LIQUIDITY_EVENT_TYPE=0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::AddLiquidityEvent
# REMOVE_LIQUIDITY_EVENT_TYPE=0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::RemoveLiquidityEvent

# API server configuration
API_HOST=127.0.0.1
API_PORT=3000
```

## Building and Running

### Build the Project

```bash
cargo build
```

### Run the Indexer

```bash
cargo run --bin suins-indexer
```

### Run the API Server

```bash
cargo run --bin api_server
```

For verbose logging, add the `RUST_LOG` environment variable:

```bash
RUST_LOG=info cargo run --bin api_server
```

## API Endpoints

### Swap Events

- `GET /api/swaps`: Get all swap events with pagination
- `GET /api/swaps?page=1&per_page=10`: Get paginated swap events
- `GET /api/swaps/by_id/{id}`: Get a specific swap event by ID

### Add Liquidity Events

- `GET /api/add_liquidity`: Get all add liquidity events with pagination
- `GET /api/add_liquidity/by_id/{id}`: Get a specific add liquidity event by ID
- `GET /api/add_liquidity/by_pool?id_contains={pool_id}`: Get add liquidity events filtered by pool ID

### Remove Liquidity Events

- `GET /api/remove_liquidity`: Get all remove liquidity events with pagination
- `GET /api/remove_liquidity/by_id/{id}`: Get a specific remove liquidity event by ID
- `GET /api/remove_liquidity/by_pool?id_contains={pool_id}`: Get remove liquidity events filtered by pool ID

## Recent Changes

- Removed `pool` and `position` fields from add liquidity and remove liquidity events
- Added `id_contains` parameter to filter events by ID pattern
- Added pagination support to all endpoints

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
