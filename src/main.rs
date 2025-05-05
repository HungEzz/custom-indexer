// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use async_trait::async_trait;
use diesel::{dsl::sql, ExpressionMethods};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use dotenvy::dotenv;
use mysten_service::metrics::start_basic_prometheus_server;
use prometheus::Registry;
use std::env;
use std::path::PathBuf;
use sui_data_ingestion_core::{
    DataIngestionMetrics, FileProgressStore, IndexerExecutor, ReaderOptions, Worker, WorkerPool,
};
use sui_types::full_checkpoint_content::CheckpointData;
use tokio::sync::oneshot;
use tracing::info;

use suins_indexer::{
    get_connection_pool,
    indexer::CetusIndexer,
    models::{CetusLiquidityEvent, CetusSwapEvent},
    schema::{cetus_liquidity_events, cetus_swap_events},
    PgConnectionPool,
};

struct CetusIndexerWorker {
    pg_pool: PgConnectionPool,
    indexer: CetusIndexer,
}

impl CetusIndexerWorker {
    /// Inserts or updates Cetus events in the database
    async fn commit_to_db(
        &self,
        swap_events: &[CetusSwapEvent],
        liquidity_events: &[CetusLiquidityEvent],
    ) -> Result<()> {
        if swap_events.is_empty() && liquidity_events.is_empty() {
            return Ok(());
        }

        let mut connection = self.pg_pool.get().await.unwrap();

        connection
            .transaction::<_, anyhow::Error, _>(|conn| {
                async move {
                    if !swap_events.is_empty() {
                        diesel::insert_into(cetus_swap_events::table)
                            .values(swap_events)
                            .on_conflict(cetus_swap_events::id)
                            .do_update()
                            .set((
                                cetus_swap_events::amount_a_in.eq(sql("excluded.amount_a_in")),
                                cetus_swap_events::amount_a_out.eq(sql("excluded.amount_a_out")),
                                cetus_swap_events::amount_b_in.eq(sql("excluded.amount_b_in")),
                                cetus_swap_events::amount_b_out.eq(sql("excluded.amount_b_out")),
                            ))
                            .execute(conn)
                            .await
                            .unwrap_or_else(|_| {
                                panic!("Failed to process swap events: {:?}", swap_events)
                            });
                    }

                    if !liquidity_events.is_empty() {
                        diesel::insert_into(cetus_liquidity_events::table)
                            .values(liquidity_events)
                            .on_conflict(cetus_liquidity_events::id)
                            .do_update()
                            .set((
                                cetus_liquidity_events::liquidity.eq(sql("excluded.liquidity")),
                            ))
                            .execute(conn)
                            .await
                            .unwrap_or_else(|_| {
                                panic!("Failed to process liquidity events: {:?}", liquidity_events)
                            });
                    }

                    Ok(())
                }
                .scope_boxed()
            })
            .await
    }
}

#[async_trait]
impl Worker for CetusIndexerWorker {
    type Result = ();
    async fn process_checkpoint(&self, checkpoint: &CheckpointData) -> Result<()> {
        let checkpoint_seq_number = checkpoint.checkpoint_summary.sequence_number;
        let (swap_events, liquidity_events) = self.indexer.process_checkpoint(checkpoint);

        // Log progress every 1000 checkpoints
        if checkpoint_seq_number % 1000 == 0 {
            info!("Checkpoint sequence number: {}", checkpoint_seq_number);
        }
        self.commit_to_db(&swap_events, &liquidity_events).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = mysten_service::logging::init();
    dotenv().ok();
    
    // Load configuration from environment variables
    let remote_storage = env::var("REMOTE_STORAGE").ok();
    let swap_event_type = env::var("SWAP_EVENT_TYPE").ok();
    let liquidity_event_type = env::var("LIQUIDITY_EVENT_TYPE").ok();
    let backfill_progress_file_path = env::var("BACKFILL_PROGRESS_FILE_PATH")
        .unwrap_or("./backfill_progress/backfill_progress".to_string());
    let checkpoints_dir = env::var("CHECKPOINTS_DIR").unwrap_or("./checkpoints".to_string());

    println!("Starting Cetus indexer with checkpoints dir: {}", checkpoints_dir);

    // Setup exit signal, progress tracking, and metrics
    let (_exit_sender, exit_receiver) = oneshot::channel();
    let progress_store = FileProgressStore::new(PathBuf::from(backfill_progress_file_path));
    let registry: Registry = start_basic_prometheus_server();
    mysten_metrics::init_metrics(&registry);
    let metrics = DataIngestionMetrics::new(&registry);
    let mut executor = IndexerExecutor::new(progress_store, 1, metrics);

    // Initialize the Cetus indexer with event type configuration
    let indexer_setup = if let (Some(swap_event_type), Some(liquidity_event_type)) =
        (swap_event_type, liquidity_event_type)
    {
        CetusIndexer::new(swap_event_type, liquidity_event_type)
    } else {
        CetusIndexer::default()
    };

    // Setup and register the worker pool
    let worker_pool = WorkerPool::new(
        CetusIndexerWorker {
            pg_pool: get_connection_pool().await,
            indexer: indexer_setup,
        },
        "cetus_indexing".to_string(), // Task name used as key in progress store
        100,                          // Concurrency level
    );
    executor.register(worker_pool).await?;

    // Start processing checkpoints
    executor
        .run(
            PathBuf::from(checkpoints_dir),
            remote_storage,
            vec![],                       // AWS credentials (empty)
            ReaderOptions::default(),
            exit_receiver,
        )
        .await?;
    
    drop(_guard);
    Ok(())
}
