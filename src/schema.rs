// @generated automatically by Diesel CLI.

diesel::table! {
    cetus_add_liquidity_events (id) {
        id -> Varchar,
        liquidity -> Varchar,
        after_liquidity -> Varchar,
        pool -> Varchar,
        position -> Varchar,
    }
}

diesel::table! {
    cetus_remove_liquidity_events (id) {
        id -> Varchar,
        liquidity -> Varchar,
        after_liquidity -> Varchar,
        pool -> Varchar,
        position -> Varchar,
    }
}

diesel::table! {
    cetus_swap_events (id) {
        id -> Varchar,
        amount_in -> Int8,
        amount_out -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cetus_add_liquidity_events,
    cetus_remove_liquidity_events,
    cetus_swap_events,
);
