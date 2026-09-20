--! insert_import_job (id, input_path, now) : (id, status, input_path, attempts, last_error?, created_at, updated_at)

INSERT INTO import_jobs (

    id,

    status,

    input_path,

    attempts,

    created_at,

    updated_at

)

VALUES (

    :id,

    'queued',

    :input_path,

    0,

    :now,

    :now

)

RETURNING

    id,

    status,

    input_path,

    attempts,

    last_error,

    created_at,

    updated_at;


--! get_import_job_by_id (id) : (id, status, input_path, attempts, last_error?, created_at, updated_at)

SELECT

    id,

    status,

    input_path,

    attempts,

    last_error,

    created_at,

    updated_at

FROM import_jobs

WHERE id = :id;


--! list_import_jobs_by_status (status) : (id, status, input_path, attempts, last_error?, created_at, updated_at)

SELECT

    id,

    status,

    input_path,

    attempts,

    last_error,

    created_at,

    updated_at

FROM import_jobs

WHERE status = :status

ORDER BY updated_at DESC;


--! claim_next_queued_job (now) : (id, status, input_path, attempts, last_error?, created_at, updated_at)

UPDATE import_jobs

SET

    status = 'running',

    attempts = attempts + 1,

    updated_at = :now

WHERE id = (

    SELECT id

    FROM import_jobs

    WHERE status = 'queued'

      AND attempts < 3

    ORDER BY created_at

    LIMIT 1

    FOR UPDATE SKIP LOCKED

)

RETURNING

    id,

    status,

    input_path,

    attempts,

    last_error,

    created_at,

    updated_at;


--! mark_import_job_succeeded (id, now) : (id, status)

UPDATE import_jobs

SET

    status = 'succeeded',

    last_error = NULL,

    updated_at = :now

WHERE id = :id

  AND status = 'running'

RETURNING

    id,

    status;


--! mark_import_job_failed (id, last_error?, now) : (id, status)

UPDATE import_jobs

SET

    status = 'failed',

    last_error = :last_error,

    updated_at = :now

WHERE id = :id

  AND status = 'running'

RETURNING

    id,

    status;


--! retry_failed_import_job (id, now) : (id, status, attempts)

UPDATE import_jobs

SET

    status = 'queued',

    updated_at = :now

WHERE id = :id

  AND status = 'failed'

  AND attempts < 3

RETURNING

    id,

    status,

    attempts;


--! reap_stale_running_jobs (stale_before, now) : (id, status, attempts)

UPDATE import_jobs

SET

    status = 'failed',

    last_error = 'worker crashed or timed out while running',

    updated_at = :now

WHERE status = 'running'

  AND updated_at < :stale_before

RETURNING

    id,

    status,

    attempts;