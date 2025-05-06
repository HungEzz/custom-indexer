use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web, http::KeepAlive};
use dotenvy::dotenv;
use std::{env, time::Duration};
use rustls;
use std::net::SocketAddr;

use suins_indexer::api::configure_api;
use suins_indexer::get_connection_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the crypto provider for rustls
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install default crypto provider");
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Load environment variables
    dotenv().ok();
    
    // Get the database connection pool
    let pool = get_connection_pool().await;
    
    // Get the host and port from environment variables or use default
    let host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);
    
    // Parse the address to ensure it's valid
    let socket_addr: SocketAddr = bind_address.parse()
        .expect("Invalid bind address format");
    
    println!("Starting API server at http://{}", bind_address);
    
    // Create and start the HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_api)
    })
    .workers(num_cpus::get()) // Set worker threads to number of available CPU cores
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(75))) // Set keep-alive timeout to 75 seconds
    .shutdown_timeout(30) // Allow 30 seconds for graceful shutdown
    .bind(socket_addr)?
    .run()
    .await
} 