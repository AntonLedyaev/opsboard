# OpsBoard

**OpsBoard** is a fullstack Rust project that simulates a lightweight job orchestration system — a mini analogue of tools like Apache Airflow.

It includes:

* a backend API for managing jobs
* a runner that processes jobs with retries and failures
* a SQLite-based persistence layer
* a frontend dashboard built entirely in Rust (Leptos)

The goal of the project is to explore Rust in a real-world-like environment: async backend, state management, database integration, and UI — all in one stack.

<img width="1772" height="1226" alt="image" src="https://github.com/user-attachments/assets/1a9e6884-0921-495d-9c0f-1b6ec7bb69cf" />


---

## Features

### Backend

* Create, list and delete jobs
* Run job queue via API
* Job lifecycle:

  * `Queued → Running → Done / Failed`
* Retry logic with failure threshold
* Simulated execution time (random delays)

### Runner

* Sequential job processing
* Failure simulation based on job name (`fail`, `slow`, etc.)
* Async execution using Tokio
* Updates job state in database

### Persistence

* SQLite database (via `sqlx`)
* Jobs stored with:

  * id
  * name
  * status
  * retry count
  * timestamps

### Frontend (Leptos)

* Dashboard UI (Airflow-inspired)
* Job table with statuses
* Summary cards (Queued / Running / Done / Failed)
* Create / Run / Delete actions
* Auto-refresh (polling)

---

## Tech Stack

**Backend**

* Rust
* Axum
* Tokio
* SQLx (SQLite)

**Frontend**

* Rust
* Leptos
* gloo (HTTP + timers)

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/your-username/opsboard.git
cd opsboard
```

### 2. Run backend

```bash
cargo run --bin backend
```

Backend will start at:

```
http://127.0.0.1:3000
```

### 3. Run frontend

```bash
cd crates/frontend
trunk serve
```

Frontend will be available at:

```
http://127.0.0.1:8080
```

---

## API Overview

| Method | Endpoint  | Description   |
| ------ | --------- | ------------- |
| GET    | `/health` | Health check  |
| GET    | `/jobs`   | Get all jobs  |
| POST   | `/jobs`   | Create a job  |
| POST   | `/run`    | Run job queue |
| POST   | `/delete` | Delete job    |

---

## Job Behavior

Job execution is simulated:

* Jobs run with a random delay
* If job name contains:

  * `fail` → job fails
  * `slow` → longer execution time
* Retry logic is applied automatically
* After max retries → job becomes `Failed`

---

## UI Overview

The frontend dashboard provides:

* **Header section** 
<img width="1212" height="196" alt="image" src="https://github.com/user-attachments/assets/ffc0b64b-f5fc-4cfd-ba6a-17459fecb335" />
* **Quick actions panel** (create job, run queue, refresh)
<img width="1218" height="161" alt="image" src="https://github.com/user-attachments/assets/04c85944-d75c-4ec9-80de-597526fd657b" />
* **Summary cards** with aggregated job states
<img width="1203" height="181" alt="image" src="https://github.com/user-attachments/assets/872bd112-fa84-403e-a85d-3481042009cb" />
* **Jobs table** with actions and status badges
<img width="1209" height="574" alt="image" src="https://github.com/user-attachments/assets/f954702a-df31-4401-839b-bb065f1c45bc" />


---

## Project Structure

```
opsboard/
├── backend/
│   ├── api.rs
│   ├── db.rs
│   ├── job.rs
│   ├── runner.rs
│   └── main.rs
│
├── crates/
│   └── frontend/
│       ├── components/
│       ├── pages/
│       ├── api.rs
│       └── main.rs
```

---

## Learning Goals

This project was built to explore:

* Rust async ecosystem (Tokio, Axum)
* Ownership and borrowing in real applications
* Database integration with SQLx
* Separation of concerns (domain / API / DTO)
* Building frontend UI in Rust (Leptos)
* Fullstack architecture without JavaScript

---

## Limitations

* No authentication
* No real distributed execution
* Runner is single-process
* Polling-based UI (no WebSockets)
* API is not fully RESTful

---

## Future Improvements

* Background workers instead of synchronous `/run`
* WebSocket-based live updates
* Job scheduling (cron-like)
* DAG support (Airflow-style dependencies)
* Better error handling and logging
* Pagination and filtering in UI

---

This is a learning project, not production-ready software.
The focus is on understanding Rust patterns in a fullstack context.
Built as a hands-on Rust learning project.

---
