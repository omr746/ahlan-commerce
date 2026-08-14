mod catalog;
mod app;
use app::{create_router, AppState};
#[tokio::main]
async fn main() {
  let state=AppState::new();
  let app=create_router(state);
  let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind to address");
  axum::serve(listener,app).await.expect("Failed to start server");
 
}
