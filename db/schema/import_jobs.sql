create table import_jobs (
    id uuid primary key,
    status text not null,
    input_path text not null,
    attempts integer not null default 0,
    last_error text,
    created_at timestamptz not null,
    updated_at timestamptz not null,

    constraint import_jobs_status_check
        check (status in ('queued', 'running', 'succeeded', 'failed')),
    constraint import_jobs_attempts_nonneg_check
        check (attempts >= 0)
);

create index import_jobs_status_created_at_idx
    on import_jobs (status, created_at);