use actix_web::{get, post, delete, web, HttpResponse, Error};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
use sqlx::{FromRow, PgPool};
use lapin::Channel;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, FromRow)]
struct Todo { pub id: Uuid, pub title: String, pub completed: bool }

#[derive(Serialize, FromRow)]
struct JobStatus { id: Uuid, status: String }


#[derive(Deserialize)]
struct Pagination {
    page: i64,
    limit: i64,
}
impl Pagination {
    fn offset(&self) -> i64 {
        (self.page.saturating_sub(1)) * self.limit
    }
}

#[get("/todos")]
async fn list(
    db: web::Data<PgPool>,
    web::Query(p): web::Query<Pagination>,
) -> Result<HttpResponse, Error> {
    let page = p.page;
    let limit = p.limit;
    if page < 1 || limit < 1 || limit > 100 {
        return Err(ErrorInternalServerError("Invalid pagination parameters"));
    }

    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
        .bind(limit)
        .bind(p.offset())
        .fetch_all(db.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(todos))
}

#[get("/todos/{id}")]
async fn get_by_id(
    db: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE id = $1"
    )
        .bind(id)
        .fetch_one(db.get_ref())
        .await
        .map_err(|_| ErrorNotFound("Not found"))?;
    Ok(HttpResponse::Ok().json(todo))
}

#[derive(Deserialize)]
struct CreatePayload { title: String }

#[post("/todos")]
async fn create(
    mq: web::Data<Channel>,
    payload: web::Json<CreatePayload>,
) -> anyhow::Result<HttpResponse, Error> {
    let job_id = Uuid::new_v4();
    let msg = serde_json::json!({
        "job_id": job_id,
        "operation": "create",
        "title": payload.title,
    });
    mq.basic_publish(
        "", "todo_tasks", Default::default(),
        &*serde_json::to_vec(&msg)?, Default::default()
    ).await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Accepted().json(JobStatus { id: job_id, status: "pending".into() }))
}

#[post("/todos/{id}/toggle")]
async fn toggle(
    mq: web::Data<Channel>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let todo_id = path.into_inner();

    let job_id = Uuid::new_v4();
    let msg = serde_json::json!({
        "job_id": job_id,
        "operation": "toggle",
        "todo_id": todo_id,
    });
    mq.basic_publish(
        "", "todo_tasks", Default::default(),
        &*serde_json::to_vec(&msg)?, Default::default()
    )
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Accepted().json(JobStatus { id: job_id, status: "pending".into() }))
}

#[delete("/todos/{id}")]
async fn delete(
    mq: web::Data<Channel>,
    path: web::Path<Uuid>,
) -> anyhow::Result<HttpResponse, Error> {
    let job_id = Uuid::new_v4();
    let msg = serde_json::json!({
        "job_id": job_id,
        "operation": "delete",
        "todo_id": path.into_inner(),
    });
    mq.basic_publish(
        "", "todo_tasks", Default::default(),
        &*serde_json::to_vec(&msg)?, Default::default()
    ).await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Accepted().json(JobStatus { id: job_id, status: "pending".into() }))
}

#[get("/jobs/{id}")]
async fn job_status(db: web::Data<PgPool>, path: web::Path<Uuid>) -> anyhow::Result<HttpResponse, Error> {
    let id = path.into_inner();
    let rec = sqlx::query_as::<_, JobStatus>(
        "SELECT id, status FROM jobs WHERE id=$1"
    )
        .bind(id)
        .fetch_one(db.get_ref())
        .await
        .map_err(|_| ErrorNotFound("Not found"))?;
    
    Ok(HttpResponse::Ok().json(rec))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list)
        .service(get_by_id)
        .service(create)
        .service(toggle)
        .service(delete)
        .service(job_status);
}