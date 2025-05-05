// @generated automatically by Diesel CLI.

diesel::table! {
    cetus_liquidity_events (id) {
        id -> Varchar,
        liquidity -> Int8,
    }
}

diesel::table! {
    cetus_swap_events (id) {
        id -> Varchar,
        amount_a_in -> Int8,
        amount_a_out -> Int8,
        amount_b_in -> Int8,
        amount_b_out -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cetus_liquidity_events,
    cetus_swap_events,
);
