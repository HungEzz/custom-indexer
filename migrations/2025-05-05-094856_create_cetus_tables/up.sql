-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "cetus_swap_events" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "amount_a_in" INT8 NOT NULL,
    "amount_a_out" INT8 NOT NULL,
    "amount_b_in" INT8 NOT NULL,
    "amount_b_out" INT8 NOT NULL
);

CREATE TABLE IF NOT EXISTS "cetus_liquidity_events" (
    "id" VARCHAR NOT NULL PRIMARY KEY,
    "liquidity" INT8 NOT NULL
);
