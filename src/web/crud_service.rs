use axum::Router;
use sqlx::{Pool, Postgres};

pub mod author;
pub mod author_book;
pub mod book;
pub mod category;
pub mod country;
pub mod curriculum;
pub mod faculty;
pub mod faculty_curriculum;
pub mod librarian;
pub mod publisher;
pub mod student;
pub mod student_card;
pub mod students_borrowing;
pub mod teacher;
pub mod teacher_card;
pub mod teachers_borrowing;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(student::routes(db.clone()))
        .merge(author::routes(db.clone()))
        .merge(book::routes(db.clone()))
        .merge(category::routes(db.clone()))
        .merge(author_book::routes(db.clone()))
        .merge(teachers_borrowing::routes(db.clone()))
        .merge(teacher_card::routes(db.clone()))
        .merge(teacher::routes(db.clone()))
        .merge(curriculum::routes(db.clone()))
        .merge(faculty::routes(db.clone()))
        .merge(faculty_curriculum::routes(db.clone()))
        .merge(publisher::routes(db.clone()))
        .merge(librarian::routes(db.clone()))
        .merge(student_card::routes(db.clone()))
        .merge(students_borrowing::routes(db.clone()))
        .merge(country::routes(db))
}
