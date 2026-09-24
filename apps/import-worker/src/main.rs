use api::app::AppState;
use api::config::Config;
use api::observability::init_tracing;
use import_worker::loop_runner::run;

#[tokio::main]
async fn main() -> Result<(), String> {
    init_tracing();
    let config = Config::new()?;

    let state = AppState::new(config)
        .await
        .expect("failed to connect to Postgres - check DATABASE_URL");

    run(state).await;
    Ok(())
}
