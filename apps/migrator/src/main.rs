use refinery::embed_migrations;
use std::env;
use tokio_postgres::NoTls;

embed_migrations!("../../db/migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load local .env if available.
    // In deployment, environment variables can be injected directly.

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "Missing required environment variable: DATABASE_URL")?;

    println!("Starting database migrations...");

    let (mut client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Keep the PostgreSQL connection alive.
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("PostgreSQL connection error: {error}");
        }
    });

    // Run pending migrations normally.
    let report = migrations::runner().run_async(&mut client).await?;

    println!("Database migrations completed.");

    println!("Applied migrations: {}", report.applied_migrations().len());

    for migration in report.applied_migrations() {
        println!("  V{}__{}", migration.version(), migration.name());
    }

    Ok(())
}
