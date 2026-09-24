-- Create "import_jobs" table
CREATE TABLE "public"."import_jobs" (
  "id" uuid NOT NULL,
  "status" text NOT NULL,
  "input_path" text NOT NULL,
  "attempts" integer NOT NULL DEFAULT 0,
  "last_error" text NULL,
  "created_at" timestamptz NOT NULL,
  "updated_at" timestamptz NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "import_jobs_attempts_nonneg_check" CHECK (attempts >= 0),
  CONSTRAINT "import_jobs_status_check" CHECK (status = ANY (ARRAY['queued'::text, 'running'::text, 'succeeded'::text, 'failed'::text]))
);
-- Create index "import_jobs_status_created_at_idx" to table: "import_jobs"
CREATE INDEX "import_jobs_status_created_at_idx" ON "public"."import_jobs" ("status", "created_at");
