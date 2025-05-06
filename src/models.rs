// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::{cetus_add_liquidity_events, cetus_remove_liquidity_events, cetus_swap_events};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Serialize)]
#[diesel(table_name = cetus_swap_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CetusSwapEvent {
    pub id: String,
    pub amount_in: i64,
    pub amount_out: i64,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Serialize)]
#[diesel(table_name = cetus_add_liquidity_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CetusAddLiquidityEvent {
    pub id: String,
    pub liquidity: String,
    pub after_liquidity: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Serialize)]
#[diesel(table_name = cetus_remove_liquidity_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CetusRemoveLiquidityEvent {
    pub id: String,
    pub liquidity: String,
    pub after_liquidity: String,
}
