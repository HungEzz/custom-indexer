-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "cetus_swap_events" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "amount_in" INT8 NOT NULL,
    "amount_out" INT8 NOT NULL
);

CREATE TABLE IF NOT EXISTS "cetus_add_liquidity_events" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "liquidity" VARCHAR NOT NULL,
    "after_liquidity" VARCHAR NOT NULL,
    "pool" VARCHAR NOT NULL,
    "position" VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS "cetus_remove_liquidity_events" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "liquidity" VARCHAR NOT NULL,
    "after_liquidity" VARCHAR NOT NULL,
    "pool" VARCHAR NOT NULL,
    "position" VARCHAR NOT NULL
);
