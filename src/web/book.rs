use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Book;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/book", get(get_books).post(create_book))
        .route("/book/:id", put(update_book).delete(delete_book))
        .with_state(db)
}

async fn get_books(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let books = sqlx::query_as!(Book, r#"SELECT id, title, release, publisher, category, student_access FROM book ORDER BY id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load books from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(books)))
}

async fn create_book(
    State(db): State<Pool<Postgres>>,
    Json(book): Json<Book>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    let inserted_book = sqlx::query_as!(
        Book,
        r#"INSERT INTO book 
        (title, release, publisher, category, student_access)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, release, publisher, category, student_access"#,
        book.title,
        book.release,
        book.publisher,
        book.category,
        book.student_access,
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add book to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_book)))
}

async fn update_book(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(book): Json<Book>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    tracing::info!("Book payload: {:?}", book);

    sqlx::query!(
        r#"UPDATE book SET
        title = $1,
        release = $2,
        publisher = $3, 
        category = $4, 
        student_access = $5
        WHERE id = $6"#,
        book.title,
        book.release,
        book.publisher,
        book.category,
        book.student_access,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update book in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(book)))
}

async fn delete_book(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    let deleted_book = sqlx::query_as!(
        Book,
        r#"DELETE FROM book WHERE id = $1 
    RETURNING id, title, release, publisher, category, student_access"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update book in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_book)))
}
