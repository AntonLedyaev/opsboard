# OpsBoard (WIP)

Mini job orchestration system written in Rust.

This project is a learning exercise to understand Rust by building a simplified job system (job queue + worker + UI in the future).

---

## Description

**OpsBoard** is a lightweight system inspired by tools like Airflow / Celery / Sidekiq, but without DAGs or complex infrastructure.

Current capabilities:

* store jobs
* manage job lifecycle
* execute jobs via a runner
* automatic retry logic
* mark jobs as failed after max retries

At the moment, this is an **in-memory job runner** (no database, no network).


## Current Status

✔ Stage 1 — basic Job model
✔ Stage 2 — state machine + retry
✔ Stage 3 — runner (worker loop)
In progress:
Stage 4 — tests
Stage 5 — HTTP API
Stage 6 — Web UI (OpsBoard)

---

##  Project Goals

The goal is not just to write code, but to:

* understand ownership and borrowing
* design a state machine
* separate responsibilities across modules
* get closer to real-world backend systems

---

##  What’s Covered So Far

* `struct` and `enum`
* `Option` and `Result`
* ownership / borrowing
* mutable references (`&mut`)
* working with collections
* modular project structure
* basic orchestration layer

---

## Roadmap

* [ ] unit tests for Job / Queue / Runner
* [ ] HTTP API (Axum)
* [ ] persistence (SQLite/Postgres)
* [ ] worker as a separate process
* [ ] web UI (React / Leptos / Yew)
* [ ] logging

---

## Notes

This is a learning project, but designed with production-like architecture in mind.

The goal is not to build a full Airflow clone, but to **understand how such systems work internally**.
