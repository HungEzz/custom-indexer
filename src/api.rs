use actix_web::{web, HttpResponse, Responder, Error};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::models::{CetusSwapEvent, CetusAddLiquidityEvent, CetusRemoveLiquidityEvent};
use crate::schema::{cetus_swap_events, cetus_add_liquidity_events, cetus_remove_liquidity_events};
use crate::PgConnectionPool;

// Query parameters for pagination
#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    per_page: Option<i64>,
}

// Query parameters for pool filtering
#[derive(Deserialize)]
pub struct PoolFilterParams {
    id_contains: String,
    page: Option<i64>,
    per_page: Option<i64>,
}

// Response format for GET /api/swaps
#[derive(Serialize)]
pub struct SwapsResponse {
    swaps: Vec<CetusSwapEvent>,
    total: i64,
    page: i64,
    per_page: i64,
}

// Response format for GET /api/add_liquidity
#[derive(Serialize)]
pub struct AddLiquidityResponse {
    events: Vec<CetusAddLiquidityEvent>,
    total: i64,
    page: i64,
    per_page: i64,
}

// Response format for GET /api/remove_liquidity
#[derive(Serialize)]
pub struct RemoveLiquidityResponse {
    events: Vec<CetusRemoveLiquidityEvent>,
    total: i64,
    page: i64,
    per_page: i64,
}

// Statistics response
#[derive(Serialize)]
pub struct StatsResponse {
    total_swaps: i64,
    total_add_liquidity: i64,
    total_remove_liquidity: i64,
}

// Time range filter
#[derive(Deserialize)]
pub struct TimeRangeParams {
    start_date: Option<String>, // ISO 8601 format: YYYY-MM-DD
    end_date: Option<String>,   // ISO 8601 format: YYYY-MM-DD
    page: Option<i64>,
    per_page: Option<i64>,
}

// Volume statistics response
#[derive(Serialize)]
pub struct VolumeStatsResponse {
    total_volume_in: i64,
    total_volume_out: i64,
    pool_stats: Vec<PoolVolumeStats>,
}

// Pool volume statistics
#[derive(Serialize)]
pub struct PoolVolumeStats {
    pool_id: String,
    volume_in: i64,
    volume_out: i64,
    swap_count: i64,
}

// GET /api/swaps - Get all swap events with pagination
pub async fn get_swaps(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count
    let total = cetus_swap_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting swap events");

    // Get paginated swaps
    let swaps = cetus_swap_events::table
        .order_by(cetus_swap_events::id.desc())
        .limit(per_page)
        .offset(offset)
        .load::<CetusSwapEvent>(&mut conn)
        .await
        .expect("Error loading swap events");

    Ok(HttpResponse::Ok().json(SwapsResponse {
        swaps,
        total,
        page,
        per_page,
    }))
}

// GET /api/add_liquidity - Get all add liquidity events with pagination
pub async fn get_add_liquidity(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count
    let total = cetus_add_liquidity_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting add liquidity events");

    // Get paginated events
    let events = cetus_add_liquidity_events::table
        .order_by(cetus_add_liquidity_events::id.desc())
        .limit(per_page)
        .offset(offset)
        .load::<CetusAddLiquidityEvent>(&mut conn)
        .await
        .expect("Error loading add liquidity events");

    Ok(HttpResponse::Ok().json(AddLiquidityResponse {
        events,
        total,
        page,
        per_page,
    }))
}

// GET /api/remove_liquidity - Get all remove liquidity events with pagination
pub async fn get_remove_liquidity(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count
    let total = cetus_remove_liquidity_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting remove liquidity events");

    // Get paginated events
    let events = cetus_remove_liquidity_events::table
        .order_by(cetus_remove_liquidity_events::id.desc())
        .limit(per_page)
        .offset(offset)
        .load::<CetusRemoveLiquidityEvent>(&mut conn)
        .await
        .expect("Error loading remove liquidity events");

    Ok(HttpResponse::Ok().json(RemoveLiquidityResponse {
        events,
        total,
        page,
        per_page,
    }))
}

// GET /api/swaps/by_pool - Get swap events for a specific pool
pub async fn get_swaps_by_pool(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PoolFilterParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    let id_filter = &query.id_contains;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count for this filter
    let total = diesel::QueryDsl::filter(
        cetus_swap_events::table,
        cetus_swap_events::id.like(format!("%{}%", id_filter))
    )
    .count()
    .get_result::<i64>(&mut conn)
    .await
    .expect("Error counting swap events for filter");

    // Get paginated swaps for this filter
    let swaps = diesel::QueryDsl::filter(
        cetus_swap_events::table,
        cetus_swap_events::id.like(format!("%{}%", id_filter))
    )
    .order_by(cetus_swap_events::id.desc())
    .limit(per_page)
    .offset(offset)
    .load::<CetusSwapEvent>(&mut conn)
    .await
    .expect("Error loading swap events for filter");

    Ok(HttpResponse::Ok().json(SwapsResponse {
        swaps,
        total,
        page,
        per_page,
    }))
}

// GET /api/add_liquidity/by_pool - Get add liquidity events for a specific pool
pub async fn get_add_liquidity_by_pool(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PoolFilterParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    let id_filter = &query.id_contains;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count for this filter
    let total = diesel::QueryDsl::filter(
        cetus_add_liquidity_events::table,
        cetus_add_liquidity_events::id.like(format!("%{}%", id_filter))
    )
    .count()
    .get_result::<i64>(&mut conn)
    .await
    .expect("Error counting add liquidity events for filter");

    // Get paginated events for this filter
    let events = diesel::QueryDsl::filter(
        cetus_add_liquidity_events::table,
        cetus_add_liquidity_events::id.like(format!("%{}%", id_filter))
    )
    .order_by(cetus_add_liquidity_events::id.desc())
    .limit(per_page)
    .offset(offset)
    .load::<CetusAddLiquidityEvent>(&mut conn)
    .await
    .expect("Error loading add liquidity events for filter");

    Ok(HttpResponse::Ok().json(AddLiquidityResponse {
        events,
        total,
        page,
        per_page,
    }))
}

// GET /api/remove_liquidity/by_pool - Get remove liquidity events for a specific pool
pub async fn get_remove_liquidity_by_pool(
    pool: web::Data<PgConnectionPool>,
    query: web::Query<PoolFilterParams>,
) -> Result<impl Responder, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    let offset = (page - 1) * per_page;
    let id_filter = &query.id_contains;

    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get total count for this filter
    let total = diesel::QueryDsl::filter(
        cetus_remove_liquidity_events::table,
        cetus_remove_liquidity_events::id.like(format!("%{}%", id_filter))
    )
    .count()
    .get_result::<i64>(&mut conn)
    .await
    .expect("Error counting remove liquidity events for filter");

    // Get paginated events for this filter
    let events = diesel::QueryDsl::filter(
        cetus_remove_liquidity_events::table,
        cetus_remove_liquidity_events::id.like(format!("%{}%", id_filter))
    )
    .order_by(cetus_remove_liquidity_events::id.desc())
    .limit(per_page)
    .offset(offset)
    .load::<CetusRemoveLiquidityEvent>(&mut conn)
    .await
    .expect("Error loading remove liquidity events for filter");

    Ok(HttpResponse::Ok().json(RemoveLiquidityResponse {
        events,
        total,
        page,
        per_page,
    }))
}

// GET /api/stats - Get overall statistics
pub async fn get_stats(
    pool: web::Data<PgConnectionPool>,
) -> Result<impl Responder, Error> {
    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Get counts for each event type
    let total_swaps = cetus_swap_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting swap events");

    let total_add_liquidity = cetus_add_liquidity_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting add liquidity events");

    let total_remove_liquidity = cetus_remove_liquidity_events::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .expect("Error counting remove liquidity events");

    Ok(HttpResponse::Ok().json(StatsResponse {
        total_swaps,
        total_add_liquidity,
        total_remove_liquidity,
    }))
}

// GET /api/volume - Get volume statistics
pub async fn get_volume_stats(
    pool: web::Data<PgConnectionPool>,
    _query: web::Query<TimeRangeParams>,
) -> Result<impl Responder, Error> {
    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Calculate total volume (simplified, in a production app you might want to use SQL aggregations)
    let swaps = cetus_swap_events::table
        .load::<CetusSwapEvent>(&mut conn)
        .await
        .expect("Error loading swap events");

    let mut total_volume_in = 0;
    let mut total_volume_out = 0;
    let mut pool_volumes: std::collections::HashMap<String, (i64, i64, i64)> = std::collections::HashMap::new();

    for swap in &swaps {
        total_volume_in += swap.amount_in;
        total_volume_out += swap.amount_out;
        
        // Try to extract pool ID from the event ID
        // This assumes the pool ID is in the event ID
        if let Some(pool_part) = swap.id.split('-').nth(1) {
            let pool_id = pool_part.to_string();
            let entry = pool_volumes.entry(pool_id).or_insert((0, 0, 0));
            entry.0 += swap.amount_in;    // volume in
            entry.1 += swap.amount_out;   // volume out
            entry.2 += 1;                // swap count
        }
    }

    // Convert pool volumes hashmap to a vector of PoolVolumeStats
    let mut pool_stats = pool_volumes
        .into_iter()
        .map(|(pool_id, (vol_in, vol_out, count))| {
            PoolVolumeStats {
                pool_id,
                volume_in: vol_in,
                volume_out: vol_out,
                swap_count: count,
            }
        })
        .collect::<Vec<_>>();
    
    // Sort by volume_in descending
    pool_stats.sort_by(|a, b| b.volume_in.cmp(&a.volume_in));

    Ok(HttpResponse::Ok().json(VolumeStatsResponse {
        total_volume_in,
        total_volume_out,
        pool_stats,
    }))
}

// GET /api/health - Simple health check endpoint
pub async fn health_check() -> impl Responder {
    #[derive(Serialize)]
    struct HealthResponse {
        status: String,
        message: String,
        version: String,
    }
    
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        message: "API server is running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// GET / - Root route that shows available API endpoints
pub async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(r#"
        <html>
            <head>
                <title>Cetus Indexer API</title>
                <style>
                    body { font-family: system-ui, -apple-system, Arial, sans-serif; line-height: 1.6; margin: 20px; }
                    h1 { color: #333; }
                    h2 { color: #555; margin-top: 30px; }
                    ul { list-style-type: none; padding-left: 20px; }
                    li { margin: 8px 0; }
                    a { color: #0366d6; text-decoration: none; }
                    a:hover { text-decoration: underline; }
                    .endpoint { background: #f6f8fa; padding: 4px 8px; border-radius: 4px; font-family: monospace; }
                </style>
            </head>
            <body>
                <h1>Cetus Indexer API</h1>
                <p>Welcome to the Cetus Indexer API. Below are the available endpoints:</p>

                <h2>Basic Endpoints</h2>
                <ul>
                    <li><a href="/api/swaps" class="endpoint">GET /api/swaps</a> - Get all swap events with pagination</li>
                    <li><a href="/api/add_liquidity" class="endpoint">GET /api/add_liquidity</a> - Get all add liquidity events with pagination</li>
                    <li><a href="/api/remove_liquidity" class="endpoint">GET /api/remove_liquidity</a> - Get all remove liquidity events with pagination</li>
                </ul>

                <h2>Pool-Specific Endpoints</h2>
                <ul>
                    <li><span class="endpoint">GET /api/swaps/by_pool?id_contains={pool_id}</span> - Get swap events for a specific pool</li>
                    <li><span class="endpoint">GET /api/add_liquidity/by_pool?id_contains={pool_id}</span> - Get add liquidity events for a specific pool</li>
                    <li><span class="endpoint">GET /api/remove_liquidity/by_pool?id_contains={pool_id}</span> - Get remove liquidity events for a specific pool</li>
                </ul>

                <h2>Analytics Endpoints</h2>
                <ul>
                    <li><a href="/api/stats" class="endpoint">GET /api/stats</a> - Get overall statistics</li>
                    <li><a href="/api/volume" class="endpoint">GET /api/volume</a> - Get volume statistics</li>
                </ul>

                <h2>Utility Endpoints</h2>
                <ul>
                    <li><a href="/api/health" class="endpoint">GET /api/health</a> - API health check</li>
                </ul>

                <p>For all list endpoints, you can use <code>page</code> and <code>per_page</code> query parameters for pagination.</p>
            </body>
        </html>
    "#)
}

// Configure API routes
pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg
        // Root route
        .route("/", web::get().to(index))
        .service(
            web::scope("/api")
                // Basic event endpoints
                .route("/swaps", web::get().to(get_swaps))
                .route("/add_liquidity", web::get().to(get_add_liquidity))
                .route("/remove_liquidity", web::get().to(get_remove_liquidity))
                
                // Pool-specific endpoints
                .route("/swaps/by_pool", web::get().to(get_swaps_by_pool))
                .route("/add_liquidity/by_pool", web::get().to(get_add_liquidity_by_pool))
                .route("/remove_liquidity/by_pool", web::get().to(get_remove_liquidity_by_pool))
                
                // Statistics and volume endpoints
                .route("/stats", web::get().to(get_stats))
                .route("/volume", web::get().to(get_volume_stats))
                
                // Health check
                .route("/health", web::get().to(health_check))
        );
} 