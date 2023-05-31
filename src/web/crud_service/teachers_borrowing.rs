use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::TeachersBorrowing;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/teachers-borrowing",
            get(get_teachers_borrowings).post(create_teachers_borrowing),
        )
        .route(
            "/teachers-borrowing/:id",
            put(update_teachers_borrowing).delete(delete_teachers_borrowing),
        )
        .with_state(db)
}

async fn get_teachers_borrowings(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<TeachersBorrowing>>), (StatusCode, String)> {
    let teachers_borrowings = sqlx::query_as!(
        TeachersBorrowing,
        r#"SELECT id, teacher_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date 
        FROM teachers_borrowing ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load teachers_borrowings from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teachers_borrowings)))
}

async fn create_teachers_borrowing(
    State(db): State<Pool<Postgres>>,
    Json(teachers_borrowing): Json<TeachersBorrowing>,
) -> Result<(StatusCode, Json<TeachersBorrowing>), (StatusCode, String)> {
    let inserted_teachers_borrowing = sqlx::query_as!(TeachersBorrowing,
        r#"INSERT INTO teachers_borrowing 
        (teacher_card, librarian, book,
        book_status_start, book_status_finish, borrow_date, return_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, teacher_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date "#,
        teachers_borrowing.teacher_card,
        teachers_borrowing.librarian,
        teachers_borrowing.book,
        teachers_borrowing.book_status_start as _,
        teachers_borrowing.book_status_finish as _,
        teachers_borrowing.borrow_date,
        teachers_borrowing.return_date,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add teachers_borrowing to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_teachers_borrowing)))
}

async fn update_teachers_borrowing(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(teachers_borrowing): Json<TeachersBorrowing>,
) -> Result<(StatusCode, Json<TeachersBorrowing>), (StatusCode, String)> {
    tracing::info!("TeachersBorrowing payload: {:?}", teachers_borrowing);

    sqlx::query!(
        r#"UPDATE teachers_borrowing SET
        teacher_card = $1,
        librarian = $2,
        book = $3, 
        book_status_start = $4, 
        book_status_finish = $5, 
        borrow_date = $6, 
        return_date = $7
        WHERE id = $8"#,
        teachers_borrowing.teacher_card,
        teachers_borrowing.librarian,
        teachers_borrowing.book,
        teachers_borrowing.book_status_start as _,
        teachers_borrowing.book_status_finish as _,
        teachers_borrowing.borrow_date,
        teachers_borrowing.return_date,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update teachers_borrowing in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(teachers_borrowing)))
}

async fn delete_teachers_borrowing(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<TeachersBorrowing>), (StatusCode, String)> {
    let deleted_teachers_borrowing = sqlx::query_as!(
        TeachersBorrowing,
        r#"DELETE FROM teachers_borrowing WHERE id = $1 
        RETURNING id, teacher_card, librarian, book,
        book_status_start as "book_status_start: _", book_status_finish as "book_status_finish: _", borrow_date, return_date"#, id)
        .fetch_one(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to update teachers_borrowing in database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_teachers_borrowing)))
}
