use axum::Router;
use sqlx::{Pool, Postgres};

pub mod student;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new().merge(student::routes(db))
}
