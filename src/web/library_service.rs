use axum::Router;
use sqlx::{Pool, Postgres};

pub mod book;
pub mod borrowing;
pub mod card;
pub mod student;
pub mod teacher;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(student::routes(db.clone()))
        .merge(teacher::routes(db.clone()))
        .merge(card::routes(db.clone()))
        .merge(borrowing::routes(db.clone()))
        .merge(book::routes(db))
}
