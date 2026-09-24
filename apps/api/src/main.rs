
use api::app::{create_router, AppState};
use api::config::Config;
use api::observability::init_tracing;
#[tokio::main]
async fn main()-> Result<(), String> {
  init_tracing();
   let config=Config::new()?;
   let addr=config.addr();
     let state = AppState::new(config)
        .await
        .expect("failed to connect to Postgres - check DATABASE_URL");

  let app=create_router(state);
  let listener=tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address");
  axum::serve(listener,app).await.expect("Failed to start server");
 Ok(())
}
