use crate::error::CatalogDbError;
use catalog::{Clock, IdGenerator, ImportJob};
use chrono::{DateTime, FixedOffset, Utc};
use deadpool_postgres::Pool;
use uuid::Uuid;

use catalog_db_queries::queries::products::import_jobs::{
    claim_next_queued_job, get_import_job_by_id, insert_import_job, list_import_jobs_by_status,
    mark_import_job_failed, mark_import_job_succeeded, reap_stale_running_jobs,
    retry_failed_import_job,
};

#[derive(Clone)]
pub struct PgImportJobs {
    pool: Pool,
}

impl PgImportJobs {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        input_path: &str,
        ids: &dyn IdGenerator,
        clock: &dyn Clock,
    ) -> Result<ImportJob, CatalogDbError> {
        let client = self.pool.get().await?;
        let id = ids.new_id();
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());

        let row = insert_import_job()
            .bind(&client, &id, &input_path, &now_fixed)
            .one()
            .await
            .map_err(CatalogDbError::from)?;

        Ok(row_to_job(
            row.id,
            row.status,
            row.input_path,
            row.attempts,
            row.last_error,
            row.created_at,
            row.updated_at,
        ))
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ImportJob>, CatalogDbError> {
        let client = self.pool.get().await?;
        let row = get_import_job_by_id()
            .bind(&client, &id)
            .opt()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(row.map(|r| {
            row_to_job(
                r.id,
                r.status,
                r.input_path,
                r.attempts,
                r.last_error,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    pub async fn list_by_status(&self, status: &str) -> Result<Vec<ImportJob>, CatalogDbError> {
        let client = self.pool.get().await?;
        let rows = list_import_jobs_by_status()
            .bind(&client, &status)
            .all()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(rows
            .into_iter()
            .map(|r| {
                row_to_job(
                    r.id,
                    r.status,
                    r.input_path,
                    r.attempts,
                    r.last_error,
                    r.created_at,
                    r.updated_at,
                )
            })
            .collect())
    }

    /// queued -> running, attempts += 1 (before the import runs).
    pub async fn claim_next(&self, clock: &dyn Clock) -> Result<Option<ImportJob>, CatalogDbError> {
        let client = self.pool.get().await?;
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        let row = claim_next_queued_job()
            .bind(&client, &now_fixed)
            .opt()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(row.map(|r| {
            row_to_job(
                r.id,
                r.status,
                r.input_path,
                r.attempts,
                r.last_error,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    pub async fn mark_succeeded(&self, id: Uuid, clock: &dyn Clock) -> Result<(), CatalogDbError> {
        let client = self.pool.get().await?;
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        mark_import_job_succeeded()
            .bind(&client, &now_fixed, &id)
            .opt()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(())
    }

    pub async fn mark_failed(
        &self,
        id: Uuid,
        last_error: &Option<String>,
        clock: &dyn Clock,
    ) -> Result<(), CatalogDbError> {
        let client = self.pool.get().await?;
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        mark_import_job_failed()
            .bind(&client, last_error, &now_fixed, &id)
            .opt()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(())
    }

    pub async fn reap_stale(
        &self,
        stale_before: DateTime<Utc>,
        clock: &dyn Clock,
    ) -> Result<Vec<ImportJob>, CatalogDbError> {
        let client = self.pool.get().await?;
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());
        let stale_before_fixed = stale_before.with_timezone(&FixedOffset::east_opt(0).unwrap());
        let rows = reap_stale_running_jobs()
            .bind(&client, &now_fixed, &stale_before_fixed)
            .all()
            .await
            .map_err(CatalogDbError::from)?;
        Ok(rows
            .into_iter()
            .map(|r| {
                row_to_job(
                    r.id,
                    r.status,
                    "".into(),
                    r.attempts,
                    None,
                    now_fixed,
                    now_fixed,
                )
            })
            .collect())
    }
    pub async fn retry(
        &self,
        id: Uuid,
        clock: &dyn Clock,
    ) -> Result<Option<RetryOutcome>, CatalogDbError> {
        let client = self.pool.get().await?;
        let now_fixed = clock
            .now()
            .with_timezone(&FixedOffset::east_opt(0).unwrap());

        let row = retry_failed_import_job()
            .bind(&client, &now_fixed, &id)
            .opt()
            .await
            .map_err(CatalogDbError::from)?;

        Ok(row.map(|r| RetryOutcome {
            id: r.id,
            status: r.status,
            attempts: r.attempts,
        }))
    }
}

fn row_to_job(
    id: Uuid,
    status: String,
    input_path: String,
    attempts: i32,
    last_error: Option<String>,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
) -> ImportJob {
    ImportJob {
        id,
        status,
        input_path,
        attempts,
        last_error,
        created_at: created_at.with_timezone(&Utc),
        updated_at: updated_at.with_timezone(&Utc),
    }
}

pub struct RetryOutcome {
    pub id: Uuid,
    pub status: String,
    pub attempts: i32,
}
