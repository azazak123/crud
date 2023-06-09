use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "book_status", rename_all = "snake_case")]
pub enum BookStatus {
    Excellent,
    Good,
    Satisfactory,
    Unsatisfactory,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "student_status", rename_all = "snake_case")]
pub enum StudentStatus {
    Graduated,
    Expelled,
    Moved,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "teacher_status", rename_all = "snake_case")]
pub enum TeacherStatus {
    Fired,
    Moved,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub surname: String,
    pub age: i16,
    pub faculty_curriculum: i32,
    pub group: i16,
    pub start_study_date: NaiveDate,
    pub status: Option<StudentStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Faculty {
    pub id: i32,
    pub name: String,
    pub letter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Curriculum {
    pub id: i32,
    pub name: String,
    pub letter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacultyCurriculum {
    pub id: i32,
    pub faculty: i32,
    pub curriculum: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub surname: String,
    pub age: i16,
    pub faculty: i32,
    pub status: Option<TeacherStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub release: NaiveDate,
    pub publisher: i32,
    pub category: i32,
    pub student_access: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub surname: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorBook {
    pub id: i32,
    pub author_id: i32,
    pub book_id: i32,
    pub num: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Librarian {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub surname: String,
    pub age: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentCard {
    pub id: i32,
    pub student: i32,
    pub issue_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherCard {
    pub id: i32,
    pub teacher: i32,
    pub issue_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentsBorrowing {
    pub id: i32,
    pub student_card: i32,
    pub librarian: i32,
    pub book: i32,
    pub book_status_start: BookStatus,
    pub book_status_finish: Option<BookStatus>,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub required_return_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeachersBorrowing {
    pub id: i32,
    pub teacher_card: i32,
    pub librarian: i32,
    pub book: i32,
    pub book_status_start: BookStatus,
    pub book_status_finish: Option<BookStatus>,
    pub borrow_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
}
