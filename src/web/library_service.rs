use axum::Router;
use sqlx::{Pool, Postgres};

pub mod student;
pub mod teacher;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(student::routes(db.clone()))
        .merge(teacher::routes(db))
}
