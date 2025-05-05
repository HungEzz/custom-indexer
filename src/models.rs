// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::schema::{cetus_liquidity_events, cetus_swap_events};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = cetus_swap_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CetusSwapEvent {
    pub id: String,
    pub amount_a_in: i64,
    pub amount_a_out: i64,
    pub amount_b_in: i64,
    pub amount_b_out: i64,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug)]
#[diesel(table_name = cetus_liquidity_events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CetusLiquidityEvent {
    pub id: String,
    pub liquidity: i64,
}
