export enum BookStatus {
  Excellent,
  Good,
  Satisfactory,
  Unsatisfactory,
}

export enum StudentStatus {
  Graduated,
  Expelled,
  Moved,
}

export enum TeacherStatus {
  Fired,
  Moved,
}

export type Student = {
  id: number;
  name: string;
  lastname: string;
  surname: string;
  age: number;
  faculty_curriculum: number;
  group: number;
  start_study_date: Date;
  status: StudentStatus | null;
};

export type Faculty = {
  id: number;
  name: string;
  letter: string;
};

export type Curriculum = {
  id: number;
  name: string;
  letter: string;
};

export type FacultyCurriculum = {
  id: number;
  faculty: number;
  curriculum: number;
};

export type Teacher = {
  id: number;
  name: string;
  lastname: string;
  surname: string;
  age: number;
  faculty: number;
  status: TeacherStatus | null;
};

export type Book = {
  id: number;
  title: string;
  release: Date;
  lisher: number;
  category: number;
  student_access: boolean;
};

export type Category = {
  id: number;
  name: string;
};

export type Author = {
  id: number;
  name: string;
  lastname: string;
  surname: string;
  country: string;
};

export type AuthorBook = {
  id: number;
  author_id: number;
  book_id: number;
  num: number;
};

export type Librarian = {
  id: number;
  name: string;
  lastname: string;
  surname: string;
  age: number;
};

export type Publisher = {
  id: number;
  name: string;
  country: string;
};

export type Country = {
  code: string;
  name: string;
};

export type StudentCard = {
  id: number;
  student: number;
  issue_date: Date;
};

export type TeacherCard = {
  id: number;
  teacher: number;
  issue_date: Date;
};

export type StudentsBorrowing = {
  id: number;
  student_card: number;
  librarian: number;
  book: number;
  book_status_start: BookStatus;
  book_status_finish: BookStatus | null;
  borrow_date: Date;
  return_date: Date | null;
  required_return_date: Date;
};

export type TeachersBorrowing = {
  id: number;
  teacher_card: number;
  librarian: number;
  book: number;
  book_status_start: BookStatus;
  book_status_finish: BookStatus | null;
  borrow_date: Date;
  return_date: Date | null;
};
