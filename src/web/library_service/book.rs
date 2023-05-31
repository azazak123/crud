use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::BookReadonly;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/book-readonly", get(get_books))
        .with_state(db)
}

async fn get_books(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<BookReadonly>>), (StatusCode, String)> {
    let books = sqlx::query_as!(
        BookReadonly,
        r#"SELECT DISTINCT 
        b.id, b.title, b.release, p.name as publisher, c.name as category, b.student_access, 
        STRING_AGG(a.lastname || ' ' || a.name || ' ' || a.surname , ', ') OVER (PARTITION BY b.id) as "authors!"
        FROM book as b 
        JOIN author_book as ab ON b.id = ab.book_id 
        JOIN author as a ON ab.author_id = a.id 
        JOIN publisher as p ON p.id = b.id 
        JOIN category as c ON c.id = b.category
        ORDER BY b.id ASC"#
    )
    .fetch_all(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to load books from database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(books)))
}
