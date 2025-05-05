# Cetus Indexer

This indexer is used to track and store Cetus protocol events on the Sui blockchain, specifically swap and liquidity events. The data is stored in a PostgreSQL database for efficient querying and analysis.

## Features

- Tracks Cetus swap events
- Tracks Cetus liquidity events
- Processes blockchain checkpoints in parallel
- Stores processed data in PostgreSQL

## Setting up locally

1. Copy `.env.sample` to `.env` and fill the variables for DB connection.
2. Make sure to set the following environment variables:

- `DATABASE_URL`: PostgreSQL connection string
- `BACKFILL_PROGRESS_FILE_PATH`: Path to track progress (format: `{ "cetus_indexing": <starting_checkpoint> }`)
- `CHECKPOINTS_DIR`: Directory to store checkpoint data (ensure it exists)
- `SWAP_EVENT_TYPE`: (Optional) Custom swap event type
- `LIQUIDITY_EVENT_TYPE`: (Optional) Custom liquidity event type

By default, the indexer tracks Cetus Protocol events on Sui Mainnet.

## Building and Running

```bash
# Build the project
cargo build

# Run the indexer
cargo run
```
