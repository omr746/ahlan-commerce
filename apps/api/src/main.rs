mod app;
use app::{create_router, AppState};
mod config;
mod routes;
mod handlers;
mod dto;
use config::Config;
#[tokio::main]
async fn main() {
   let config=Config::new();
   let addr=config.addr();
  let state=AppState::new(config);
  let app=create_router(state);
  let listener=tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address");
  axum::serve(listener,app).await.expect("Failed to start server");
 
}
