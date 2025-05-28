# Overengineered Rust TODO API 🦀

> **Warning:** Dangerously overengineered.

This repo is a playground in overengineering: two separate Rust services communicating over RabbitMQ, managed with Docker Compose.

Each "time-consuming" operation (create, delete, toggle) is handled by a background worker. The API Gateway service handles HTTP requests and enqueues jobs for the worker service to process.

Yes, it’s all completely unnecessary for a simple TODO app. But it's a great showcase of an async event-driven system architecture in Rust.

#### Demo:
>[https://overengineered-todos.frangiadakis.com](https://overengineered-todos.frangiadakis.com)
---

## 📂 Project Structure

```
├── docker-compose.yml
├── .env                   # DB & RabbitMQ URLs
├── migrations/            # SQLx migrations (CREATE tables)
├── api-gateway/           # Gateway service
├── todo-processor/        # Background worker service
└── ui/                    # A very simple React UI (optional)

```

---

## 🚀 Quickstart

### 1. Clone & configure

```bash
git clone https://github.com/mitsosf/overengineered-todo-rust.git
cd overengineered-todo-rust
cp .env.example .env
# Adjust .env if needed (e.g. host ports)
```

### 2. Fire up Docker

```bash
docker compose up --build -d
```

This brings up:

* **Postgres**
* **RabbitMQ**
* **api-gateway** on `localhost:8080`
* **todo-processor** as a hidden worker container
* **UI** on `localhost:6967` (optional)

### 3. Run migrations

Ensure `DATABASE_URL` in your shell matches `.env` and then:

```bash
export DATABASE_URL=$(grep DATABASE_URL .env | cut -d '=' -f2)
sqlx migrate run
```

---

## 🔌 API Endpoints

> All write operations return a `job_id` - poll `/jobs/{job_id}` until status is `completed`.

| Method | Path                     | Description                                       |
|--------|--------------------------|---------------------------------------------------|
| GET    | `/todos?page=1&limit=10` | List all TODOs (paginated)                        |
| GET    | `/todos/{id}`            | Fetch one TODO by ID                              |
| POST   | `/todos`                 | Create a TODO (enqueue job)                       |
| DELETE | `/todos/{id}`            | Delete a TODO (enqueue job)                       |
| POST   | `/todos/{id}/toggle`     | Flip `completed` status (enqueue job)             |
| GET    | `/jobs/{job_id}`         | Check job status (`pending`/`completed`/`failed`) |

### Polling Example (JS)

```js
async function pollJob(id) {
    let status;
    do {
        const res = await fetch(`/jobs/${id}`);
        status = (await res.json()).status;
        await new Promise(r => setTimeout(r, 500));
    } while (status === 'pending');
    return status;
}
```

---

## 🎩 Overengineered Tech Stack

* **Async everywhere** with Tokio + Actix + Lapin
* **Decoupled services**—just to create a single note
* **RabbitMQ tasks** instead of simple DB calls

> *Because why call the DB directly when you can publish to a queue?* 🤷‍♀️

---

## 🛠️ Development tips

* **SQLx CLI**:

  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

---

## 👋 License & Credits

* Feel free to copy, adapt, or shamelessly plagiarize. This is a portfolio toy, not production code.

Always remember - in real world scenarios: Keep It Simple, Stupid 🦀🚀
