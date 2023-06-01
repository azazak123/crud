use axum::extract::Path;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::BorrowingReadonly;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/borrowing-readonly/:is_teacher/:card", get(get_borrowings))
        .with_state(db)
}

async fn get_borrowings(
    State(db): State<Pool<Postgres>>,
    Path((is_teacher, card)): Path<(bool, i32)>,
) -> Result<(StatusCode, Json<Vec<BorrowingReadonly>>), (StatusCode, String)> {
    let borrowings = if is_teacher {
        sqlx::query_as!(
            BorrowingReadonly,
            r#"SELECT tb.id, 
            t.lastname || ' ' || t.name || ' ' || t.surname as "owner!",
            tb.teacher_card as card, 
            l.lastname || ' ' || l.name || ' ' || l.surname as "librarian_name!",
            l.id as librarian,
            b.title as book_title, b.id as book,
            tb.book_status_start as "book_status_start:_", 
            tb.book_status_finish as "book_status_finish:_", 
            tb.borrow_date, tb.return_date, 
            null as "required_return_date:_"
            FROM teachers_borrowing as tb 
            JOIN librarian as l ON tb.librarian=l.id 
            JOIN book as b ON tb.book=b.id 
            JOIN teacher_card as tc ON tc.id=tb.teacher_card 
            JOIN teacher as t ON tc.teacher=t.id WHERE tb.teacher_card=$1
            ORDER BY tb.id ASC"#,
            card
        )
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load teachers borrowings from database"))
    } else {
        sqlx::query_as!(
            BorrowingReadonly,
            r#"SELECT sb.id, 
            s.lastname || ' ' || s.name || ' ' || s.surname as "owner!",
            sb.student_card as card, 
            l.lastname || ' ' || l.name || ' ' || l.surname as "librarian_name!",
            l.id as librarian,
            b.title as book_title, b.id as book,
            sb.book_status_start as "book_status_start:_", 
            sb.book_status_finish as "book_status_finish:_", 
            sb.borrow_date, sb.return_date, 
            sb.required_return_date as "required_return_date:_"
            FROM students_borrowing as sb 
            JOIN librarian as l ON sb.librarian=l.id 
            JOIN book as b ON sb.book=b.id 
            JOIN student_card as sc ON sc.id=sb.student_card 
            JOIN student as s ON sc.student=s.id WHERE sb.student_card=$1
            ORDER BY sb.id ASC"#,
            card
        )
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load students borrowings from database"))
    }
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(borrowings)))
}
