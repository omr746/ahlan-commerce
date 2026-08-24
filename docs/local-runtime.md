# Local Runtime Debug Notes

This document explains how to run and debug the Ahlan Commerce application locally using `mprocs`.

## Starting the Local Runtime

The local process board runs the API and PostgreSQL processes together:

```bash
make start
```

This opens `mprocs` with separate processes for:

* API
* PostgreSQL

Instead of opening multiple terminals, both processes can be monitored from one view.

## API Logs

The API process runs the local API using:

```bash
make run
```

To inspect API logs, select the **api** process inside `mprocs`.

This is useful for checking:

* Application startup
* HTTP requests
* `tracing` output
* Database connection errors
* API errors and panics

## PostgreSQL Logs

The PostgreSQL process uses the database log command:

```bash
make db-logs
```

To inspect database logs, select the **postgres** process inside `mprocs`.

These logs are useful for checking:

* PostgreSQL startup
* Database connection problems
* Migration-related issues
* PostgreSQL errors

## Stopping the Local Runtime

When finished, stop the local process board with:

```bash
make stop
```

Depending on the Makefile configuration, this stops the local processes without representing a production deployment shutdown.

## Local Workflow vs Production

`mprocs` is a **local development workflow tool**.

It provides a convenient way to run and monitor multiple development processes from one terminal interface.

It does **not** represent how the application should be deployed to production.

The production environment would use dedicated process management and deployment infrastructure rather than relying on a developer's local `mprocs` session.

The purpose of `mprocs` in Ahlan Commerce is therefore to make the local development workflow easier:

```text
make start
    │
    └── mprocs
         ├── API
         │    └── API logs
         │
         └── PostgreSQL
              └── DB logs
```
