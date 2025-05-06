// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use move_core_types::language_storage::StructTag;
use serde::{Deserialize, Serialize};
use sui_types::{
    base_types::{ObjectID},
    full_checkpoint_content::{CheckpointData, CheckpointTransaction},
};

use crate::models::{CetusAddLiquidityEvent, CetusRemoveLiquidityEvent, CetusSwapEvent};

// Define constants for Cetus event types
// Cetus Protocol package ID on Sui Mainnet
const CETUS_SWAP_EVENT_TYPE: &str = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent";
const CETUS_ADD_LIQUIDITY_EVENT_TYPE: &str = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::AddLiquidityEvent";
const CETUS_REMOVE_LIQUIDITY_EVENT_TYPE: &str = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::RemoveLiquidityEvent";

// Struct for SwapEvent - corrected based on binary format from logs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwapEventData {
    pub atob: bool,
    pub pool: ObjectID,
    pub partner: ObjectID,
    pub amount_in: u64,
    pub amount_out: u64,
    pub ref_amount: u64,
    pub fee_amount: u64,
    pub vault_a_amount: u64,
    pub vault_b_amount: u64,
    pub before_sqrt_price: u128,
    pub after_sqrt_price: u128,
    pub steps: u64,
}

// Struct cho AddLiquidityEvent
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddLiquidityEventData {
    pub pool: ObjectID,
    pub position: ObjectID,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub after_liquidity: u128,
    pub amount_a: u64,
    pub amount_b: u64,
}

// Struct cho RemoveLiquidityEvent
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveLiquidityEventData {
    pub pool: ObjectID,
    pub position: ObjectID,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub after_liquidity: u128,
    pub amount_a: u64,
    pub amount_b: u64,
}

pub struct CetusIndexer {
    swap_event_type: StructTag,
    add_liquidity_event_type: StructTag,
    remove_liquidity_event_type: StructTag,
}

impl std::default::Default for CetusIndexer {
    fn default() -> Self {
        Self::new(
            CETUS_SWAP_EVENT_TYPE.to_owned(),
            CETUS_ADD_LIQUIDITY_EVENT_TYPE.to_owned(),
            CETUS_REMOVE_LIQUIDITY_EVENT_TYPE.to_owned(),
        )
    }
}

impl CetusIndexer {
    /// Create a new indexer with custom event types
    pub fn new(swap_event_type: String, add_liquidity_event_type: String, remove_liquidity_event_type: String) -> Self {
        let swap_event_type = StructTag::from_str(&swap_event_type).unwrap();
        let add_liquidity_event_type = StructTag::from_str(&add_liquidity_event_type).unwrap();
        let remove_liquidity_event_type = StructTag::from_str(&remove_liquidity_event_type).unwrap();

        Self {
            swap_event_type,
            add_liquidity_event_type,
            remove_liquidity_event_type,
        }
    }

    /// Process a checkpoint and extract Cetus events
    pub fn process_checkpoint(
        &self,
        data: &CheckpointData,
    ) -> (Vec<CetusSwapEvent>, Vec<CetusAddLiquidityEvent>, Vec<CetusRemoveLiquidityEvent>) {
        let mut swap_events = Vec::new();
        let mut add_liquidity_events = Vec::new();
        let mut remove_liquidity_events = Vec::new();

        // Print checkpoint info
        tracing::info!(
            "Processing checkpoint #{} with {} transactions",
            data.checkpoint_summary.sequence_number,
            data.transactions.len()
        );

        // Iterate through all transactions in the checkpoint
        for transaction in &data.transactions {
            self.process_transaction(
                transaction,
                &mut swap_events,
                &mut add_liquidity_events,
                &mut remove_liquidity_events,
            );
        }

        tracing::info!(
            "Found {} swap events, {} add liquidity events, and {} remove liquidity events in checkpoint #{}",
            swap_events.len(),
            add_liquidity_events.len(),
            remove_liquidity_events.len(),
            data.checkpoint_summary.sequence_number
        );

        (swap_events, add_liquidity_events, remove_liquidity_events)
    }

    fn process_transaction(
        &self,
        transaction: &CheckpointTransaction,
        swap_events: &mut Vec<CetusSwapEvent>,
        add_liquidity_events: &mut Vec<CetusAddLiquidityEvent>,
        remove_liquidity_events: &mut Vec<CetusRemoveLiquidityEvent>,
    ) {
        let tx_digest = transaction.transaction.digest().to_string();
        
        // Extract all events from the transaction, if any
        if let Some(events) = &transaction.events {
            tracing::debug!("Transaction {} has {} events", tx_digest, events.data.len());
            
            // Print all events for debugging
            for (i, event) in events.data.iter().enumerate() {
                tracing::debug!(
                    "Event {}: type={}, package_id={}",
                    i,
                    event.type_,
                    event.package_id
                );
                // Try to print the raw contents as hex for debugging
                let hex_content = hex::encode(&event.contents);
                tracing::debug!("Event {} raw content (hex): {}", i, hex_content);
            }
            
            // Keep track of event counts for each type to create unique IDs
            let mut swap_count = 0;
            let mut add_liquidity_count = 0;
            let mut remove_liquidity_count = 0;
            
            for event in &events.data {
                // Use the event struct type as comparison
                let event_type = &event.type_;
                
                // Generate a base ID for this event
                let base_id = format!("{}-{}", event.package_id, &tx_digest);

                // Check if it's a swap event
                if event_type == &self.swap_event_type {
                    tracing::info!("Found Cetus swap event in tx: {}", tx_digest);
                    match bcs::from_bytes::<SwapEventData>(&event.contents) {
                        Ok(swap_data) => {
                            tracing::info!(
                                "Swap event details: amount_in={}, amount_out={}",
                                swap_data.amount_in,
                                swap_data.amount_out
                            );
                            // Create a unique ID with counter
                            let unique_id = format!("{}-swap-{}", base_id, swap_count);
                            swap_count += 1;
                            
                            swap_events.push(CetusSwapEvent {
                                id: unique_id,
                                amount_in: swap_data.amount_in as i64,
                                amount_out: swap_data.amount_out as i64,
                            });
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize swap event: {}", e);
                            // In ra dữ liệu thô để debug
                            let hex_content = hex::encode(&event.contents);
                            tracing::error!("Raw content (hex): {}", hex_content);
                        }
                    }
                }
                // Check if it's an add liquidity event
                else if event_type == &self.add_liquidity_event_type {
                    tracing::info!("Found Cetus add liquidity event in tx: {}", tx_digest);
                    match bcs::from_bytes::<AddLiquidityEventData>(&event.contents) {
                        Ok(data) => {
                            tracing::info!(
                                "Add liquidity event details: liquidity={}, after_liquidity={}",
                                data.liquidity,
                                data.after_liquidity
                            );
                            
                            // Create a unique ID with counter
                            let unique_id = format!("{}-add-{}", base_id, add_liquidity_count);
                            add_liquidity_count += 1;
                            
                            add_liquidity_events.push(CetusAddLiquidityEvent {
                                id: unique_id,
                                liquidity: data.liquidity.to_string(),
                                after_liquidity: data.after_liquidity.to_string(),
                                pool: data.pool.to_string(),
                                position: data.position.to_string(),
                            });
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize add liquidity event: {}", e);
                            // In ra dữ liệu thô để debug
                            let hex_content = hex::encode(&event.contents);
                            tracing::error!("Raw content (hex): {}", hex_content);
                        }
                    }
                }
                // Check if it's a remove liquidity event
                else if event_type == &self.remove_liquidity_event_type {
                    tracing::info!("Found Cetus remove liquidity event in tx: {}", tx_digest);
                    match bcs::from_bytes::<RemoveLiquidityEventData>(&event.contents) {
                        Ok(data) => {
                            tracing::info!(
                                "Remove liquidity event details: liquidity={}, after_liquidity={}",
                                data.liquidity,
                                data.after_liquidity
                            );
                            
                            // Create a unique ID with counter
                            let unique_id = format!("{}-remove-{}", base_id, remove_liquidity_count);
                            remove_liquidity_count += 1;
                            
                            remove_liquidity_events.push(CetusRemoveLiquidityEvent {
                                id: unique_id,
                                liquidity: data.liquidity.to_string(),
                                after_liquidity: data.after_liquidity.to_string(),
                                pool: data.pool.to_string(),
                                position: data.position.to_string(),
                            });
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize remove liquidity event: {}", e);
                            // In ra dữ liệu thô để debug
                            let hex_content = hex::encode(&event.contents);
                            tracing::error!("Raw content (hex): {}", hex_content);
                        }
                    }
                }
            }
        } else {
            tracing::debug!("Transaction {} has no events", tx_digest);
        }
    }
}
