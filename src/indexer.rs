// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use move_core_types::language_storage::StructTag;
use serde::{Deserialize, Serialize};
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    full_checkpoint_content::{CheckpointData, CheckpointTransaction},
};

use crate::models::{CetusLiquidityEvent, CetusSwapEvent};

// Define constants for Cetus event types
// Cetus Protocol package ID on Sui Mainnet
const CETUS_SWAP_EVENT_TYPE: &str = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent";
const CETUS_LIQUIDITY_EVENT_TYPE: &str = "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::LiquidityEvent";

// Define structs that match the on-chain event structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwapEventData {
    pub sender: SuiAddress,
    pub pool_id: ObjectID,
    pub amount_a_in: u64,
    pub amount_a_out: u64,
    pub amount_b_in: u64,
    pub amount_b_out: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiquidityEventData {
    pub sender: SuiAddress,
    pub pool_id: ObjectID,
    pub is_add_liquidity: bool,
    pub liquidity: u64,
    pub amount_a: u64,
    pub amount_b: u64,
}

pub struct CetusIndexer {
    swap_event_type: StructTag,
    liquidity_event_type: StructTag,
}

impl std::default::Default for CetusIndexer {
    fn default() -> Self {
        Self::new(
            CETUS_SWAP_EVENT_TYPE.to_owned(),
            CETUS_LIQUIDITY_EVENT_TYPE.to_owned(),
        )
    }
}

impl CetusIndexer {
    /// Create a new indexer with custom event types
    pub fn new(swap_event_type: String, liquidity_event_type: String) -> Self {
        let swap_event_type = StructTag::from_str(&swap_event_type).unwrap();
        let liquidity_event_type = StructTag::from_str(&liquidity_event_type).unwrap();

        Self {
            swap_event_type,
            liquidity_event_type,
        }
    }

    /// Process a checkpoint and extract Cetus swap and liquidity events
    pub fn process_checkpoint(
        &self,
        data: &CheckpointData,
    ) -> (Vec<CetusSwapEvent>, Vec<CetusLiquidityEvent>) {
        let mut swap_events = Vec::new();
        let mut liquidity_events = Vec::new();

        // Iterate through all transactions in the checkpoint
        for transaction in &data.transactions {
            self.process_transaction(
                transaction,
                &mut swap_events,
                &mut liquidity_events,
            );
        }

        (swap_events, liquidity_events)
    }

    fn process_transaction(
        &self,
        transaction: &CheckpointTransaction,
        swap_events: &mut Vec<CetusSwapEvent>,
        liquidity_events: &mut Vec<CetusLiquidityEvent>,
    ) {
        // Extract all events from the transaction, if any
        if let Some(events) = &transaction.events {
            for event in &events.data {
                // Use the event struct type as comparison
                let event_type = &event.type_;
                let tx_digest = transaction.transaction.digest().to_string();
                // Generate a unique ID for this event
                let event_id = format!("{}-{}", event.package_id, &tx_digest);

                // Check if it's a Cetus swap event
                if event_type == &self.swap_event_type {
                    if let Ok(swap_data) = bcs::from_bytes::<SwapEventData>(&event.contents) {
                        swap_events.push(CetusSwapEvent {
                            id: format!("{}-{}", event_id, tx_digest),
                            amount_a_in: swap_data.amount_a_in as i64,
                            amount_a_out: swap_data.amount_a_out as i64,
                            amount_b_in: swap_data.amount_b_in as i64,
                            amount_b_out: swap_data.amount_b_out as i64,
                        });
                    }
                }
                // Check if it's a Cetus liquidity event
                else if event_type == &self.liquidity_event_type {
                    if let Ok(liquidity_data) = bcs::from_bytes::<LiquidityEventData>(&event.contents) {
                        liquidity_events.push(CetusLiquidityEvent {
                            id: format!("{}-{}", event_id, tx_digest),
                            liquidity: liquidity_data.liquidity as i64,
                        });
                    }
                }
            }
        }
    }
}
