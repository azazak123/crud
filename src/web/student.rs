use axum::extract::Path;
use axum::routing::put;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use color_eyre::eyre::Context;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};

use crate::error::internal_error;
use crate::model::Student;

pub fn routes(db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/student", get(get_students).post(create_student))
        .route("/student/:id", put(update_student).delete(delete_student))
        .with_state(db)
}

async fn get_students(
    State(db): State<Pool<Postgres>>,
) -> Result<(StatusCode, Json<Vec<Student>>), (StatusCode, String)> {
    let students = sqlx::query_as!(Student, r#"SELECT id, name, lastname, surname, age, faculty_curriculum, "group", start_study_date, status as "status: _" FROM student ORDER BY id ASC"#)
        .fetch_all(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to load students from database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(students)))
}

async fn create_student(
    State(db): State<Pool<Postgres>>,
    Json(student): Json<Student>,
) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {
    let inserted_student = sqlx::query_as!(Student,
        r#"INSERT INTO student 
        (name, lastname, surname, age, faculty_curriculum, "group", start_study_date, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, lastname, surname, age, faculty_curriculum, "group", start_study_date, status as "status: _""#,
        student.name,
        student.lastname,
        student.surname,
        student.age,
        student.faculty_curriculum,
        student.group,
        student.start_study_date,
        student.status as _
    )
    .fetch_one(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to add student to database"))
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(inserted_student)))
}

async fn update_student(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(student): Json<Student>,
) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {
    tracing::info!("Student payload: {:?}", student);

    sqlx::query!(
        r#"UPDATE student SET
        name = $1,
        lastname = $2,
        surname = $3, 
        age = $4, 
        faculty_curriculum = $5, 
        "group" = $6, 
        start_study_date = $7, 
        status = $8
        WHERE id = $9"#,
        student.name,
        student.lastname,
        student.surname,
        student.age,
        student.faculty_curriculum,
        student.group,
        student.start_study_date,
        student.status as _,
        id
    )
    .execute(&db)
    .await
    .wrap_err_with(|| eyre!("Unable to update student in database"))
    .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(student)))
}

async fn delete_student(
    State(db): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {
    let deleted_student = sqlx::query_as!(Student, r#"DELETE FROM student WHERE id = $1 
    RETURNING id, name, lastname, surname, age, faculty_curriculum, "group", start_study_date, status as "status: _""#, id)
        .fetch_one(&db)
        .await
        .wrap_err_with(|| eyre!("Unable to update student in database"))
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(deleted_student)))
}
