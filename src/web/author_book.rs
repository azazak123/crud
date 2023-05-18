use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::AuthorBook;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/author-book",
            get(get_author_books).post(create_author_book),
        )
        .route(
            "/author-book/:id",
            put(update_author_book).delete(delete_author_book),
        )
        .with_state(db)
}

async fn get_author_books(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<AuthorBook>>), (StatusCode, String)> {
    let author_books = sqlx::query_as!(
        AuthorBook,
        r#"SELECT id, author_id, book_id, num FROM author_book ORDER BY id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load author_books from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(author_books)))
}

async fn create_author_book(
    State(db): State<Pool<Postgres>>,
    Json(author_book): Json<AuthorBook>,
) -> Result<(StatusCode, Json<AuthorBook>), (StatusCode, String)> {
    let inserted_author_book = sqlx::query_as!(
        AuthorBook,
        r#"INSERT INTO author_book 
        (author_id, book_id, num)
        VALUES ($1, $2, $3)
        RETURNING id, author_id, book_id, num"#,
        author_book.author_id,
        author_book.book_id,
        author_book.num
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add author_book to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_author_book)))
}

async fn update_author_book(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(author_book): Json<AuthorBook>,
) -> Result<(StatusCode, Json<AuthorBook>), (StatusCode, String)> {
    tracing::info!("AuthorBook payload: {:?}", author_book);

    sqlx::query!(
        r#"UPDATE author_book SET
        author_id = $1,
        book_id = $2,
        num = $3
        WHERE id = $4"#,
        author_book.author_id,
        author_book.book_id,
        author_book.num,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update author_book in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(author_book)))
}

async fn delete_author_book(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<AuthorBook>), (StatusCode, String)> {
    let deleted_author_book = sqlx::query_as!(
        AuthorBook,
        r#"DELETE FROM author_book WHERE id = $1 
        RETURNING id, author_id, book_id, num"#,
        id
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update author_book in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_author_book)))
}
