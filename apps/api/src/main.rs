mod app;
use app::{create_router, AppState};
mod config;
mod routes;
mod handlers;
mod dto;
mod error;
mod observability;
use config::Config;
use observability::init_tracing;
#[tokio::main]
async fn main() {
  init_tracing();
   let config=Config::new();
   let addr=config.addr();
  let state=AppState::new(config);
  let app=create_router(state);
  let listener=tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address");
  axum::serve(listener,app).await.expect("Failed to start server");
 
}
