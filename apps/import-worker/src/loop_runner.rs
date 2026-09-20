use crate::import_logic::import_file;
use api::app::AppState;
use chrono::Utc;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_secs(1);
const STALE_RUNNING_AFTER: chrono::Duration = chrono::Duration::minutes(5);
const REAP_EVERY_N_TICKS: u32 = 30;
const RETRY_FAILED_EVERY_N_TICKS: u32 = 60;
pub async fn run(state: AppState) {
    tracing::info!("worker started, polling for queued import jobs");
    let mut tick: u32 = 0;

    loop {
        tick = tick.wrapping_add(1);

        if tick % REAP_EVERY_N_TICKS == 0 {
            reap(&state).await;
        }
        if tick % RETRY_FAILED_EVERY_N_TICKS == 0 {
            auto_retry_failed(&state).await;
        }

        match state.import_jobs.claim_next(state.clock.as_ref()).await {
            Ok(Some(job)) => {
                tracing::info!(job_id = %job.id, attempt = job.attempts, status = %job.status, "claimed import job");
                run_one(&state, &job).await;
            }
            Ok(None) => tokio::time::sleep(POLL_INTERVAL).await,
            Err(e) => {
                tracing::error!(error = %e, "failed to claim next job");
                tokio::time::sleep(POLL_INTERVAL).await;
            }
        }
    }
}

async fn run_one(state: &AppState, job: &catalog::ImportJob) {
    // No HTTP request here, so there's no real RequestId to correlate with —
    // the job's own id fills that role for logging/AppError purposes.
    let request_id = job.id;

    let result = import_file(
        &state.catalog,
        state.ids.as_ref(),
        state.clock.as_ref(),
        &job.input_path,
        request_id,
    )
    .await;

    match result {
        Ok(created) => match state.import_jobs.mark_succeeded(job.id, state.clock.as_ref()).await {
            Ok(_) => tracing::info!(
                job_id = %job.id, attempt = job.attempts, status = "succeeded",
                products_created = created, "import job succeeded"
            ),
            Err(e) => tracing::error!(job_id = %job.id, error = %e, "failed to mark job succeeded"),
        },
        Err(app_err) => {
            // AppError -> safe last_error text. Swap `.to_string()` for
            // whatever accessor AppError actually exposes (e.g. `.message()`)
            // if it isn't a plain Display impl.
            let message = Some(app_err.public_message());
            match state.import_jobs.mark_failed(job.id, &message, state.clock.as_ref()).await {
                Ok(_) => tracing::error!(
                    job_id = %job.id, attempt = job.attempts, status = "failed",
                    error_code = "import_failed", error =%message.as_deref().unwrap_or(""), "import job failed"
                ),
                Err(e) => tracing::error!(job_id = %job.id, error = %e, "failed to mark job failed"),
            }
        }
    }
}

async fn reap(state: &AppState) {
    let stale_before = Utc::now() - STALE_RUNNING_AFTER;
    match state.import_jobs.reap_stale(stale_before, state.clock.as_ref()).await {
        Ok(reaped) => for job in reaped {
            tracing::warn!(
                job_id = %job.id, attempt = job.attempts, status = %job.status,
                error_code = "worker_crash_recovered", "reaped a job stuck in running"
            );
        },
        Err(e) => tracing::error!(error = %e, "failed to reap stale running jobs"),
    }
}

async fn auto_retry_failed(state: &AppState) {
    let failed = match state.import_jobs.list_by_status("failed").await {
        Ok(jobs) => jobs,
        Err(e) => {
            tracing::error!(error = %e, "failed to list failed jobs for auto-retry");
            return;
        }
    };

    for job in failed {
        match state.import_jobs.retry(job.id, state.clock.as_ref()).await {
            Ok(Some(outcome)) => {
                tracing::info!(
                    job_id = %outcome.id, attempts = outcome.attempts, status = %outcome.status,
                    "auto-retried failed job"
                );
            }
            Ok(None) => {
                // attempts already at 3 -- correctly left alone, per contract.
            }
            Err(e) => tracing::error!(job_id = %job.id, error = %e, "failed to auto-retry job"),
        }
    }
}