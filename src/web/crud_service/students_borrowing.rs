use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::StudentsBorrowing;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/students-borrowing",
            get(get_students_borrowings).post(create_students_borrowing),
        )
        .route(
            "/students-borrowing/:id",
            put(update_students_borrowing).delete(delete_students_borrowing),
        )
        .with_state(db)
}

async fn get_students_borrowings(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<StudentsBorrowing>>), (StatusCode, String)> {
    let students_borrowings = sqlx::query_as!(StudentsBorrowing, 
        r#"SELECT id, student_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date, required_return_date
        FROM students_borrowing ORDER BY id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load students_borrowings from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(students_borrowings)))
}

async fn create_students_borrowing(
    State(db): State<Pool<Postgres>>,
    Json(students_borrowing): Json<StudentsBorrowing>,
) -> Result<(StatusCode, Json<StudentsBorrowing>), (StatusCode, String)> {
    let inserted_students_borrowing = sqlx::query_as!(StudentsBorrowing,
        r#"INSERT INTO students_borrowing 
        (student_card, librarian, book,
        book_status_start, book_status_finish, borrow_date, return_date, required_return_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, student_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date, required_return_date"#,
        students_borrowing.student_card,
        students_borrowing.librarian,
        students_borrowing.book,
        students_borrowing.book_status_start as _,
        students_borrowing.book_status_finish as _,
        students_borrowing.borrow_date,
        students_borrowing.return_date,
        students_borrowing.required_return_date,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add students_borrowing to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_students_borrowing)))
}

async fn update_students_borrowing(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(students_borrowing): Json<StudentsBorrowing>,
) -> Result<(StatusCode, Json<StudentsBorrowing>), (StatusCode, String)> {
    tracing::info!("StudentsBorrowing payload: {:?}", students_borrowing);

    sqlx::query!(
        r#"UPDATE students_borrowing SET
        student_card = $1,
        librarian = $2,
        book = $3, 
        book_status_start = $4, 
        book_status_finish = $5, 
        borrow_date = $6, 
        return_date = $7, 
        required_return_date = $8
        WHERE id = $9"#,
        students_borrowing.student_card,
        students_borrowing.librarian,
        students_borrowing.book,
        students_borrowing.book_status_start as _,
        students_borrowing.book_status_finish as _,
        students_borrowing.borrow_date,
        students_borrowing.return_date,
        students_borrowing.required_return_date,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update students_borrowing in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(students_borrowing)))
}

async fn delete_students_borrowing(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<StudentsBorrowing>), (StatusCode, String)> {
    let deleted_students_borrowing = sqlx::query_as!(StudentsBorrowing, r#"DELETE FROM students_borrowing WHERE id = $1 
        RETURNING id, student_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date, required_return_date"#, id)
        .fetch_one(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to update students_borrowing in database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_students_borrowing)))
}
