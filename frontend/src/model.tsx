export enum BookStatus {
  Excellent = "Excellent",
  Good = "Good",
  Satisfactory = "Satisfactory",
  Unsatisfactory = "Unsatisfactory",
}

export enum StudentStatus {
  Graduated = "Graduated",
  Expelled = "Expelled",
  Moved = "Moved",
}

export enum TeacherStatus {
  Fired = "Fired",
  Moved = "Moved",
}

export type Student = {
  id: number;
  name: string;
  lastname: string;
  surname: string;
  age: number;
  faculty_curriculum: number;
  group: number;
  start_study_date: string;
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
  release: string;
  publisher: number;
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
  issue_date: string;
};

export type TeacherCard = {
  id: number;
  teacher: number;
  issue_date: string;
};

export type StudentsBorrowing = {
  id: number;
  student_card: number;
  librarian: number;
  book: number;
  book_status_start: BookStatus;
  book_status_finish: BookStatus | null;
  borrow_date: string;
  return_date: string | null;
  required_return_date: string;
};

export type TeachersBorrowing = {
  id: number;
  teacher_card: number;
  librarian: number;
  book: number;
  book_status_start: BookStatus;
  book_status_finish: BookStatus | null;
  borrow_date: string;
  return_date: string | null;
};

export type Entity =
  | Student
  | Faculty
  | Curriculum
  | FacultyCurriculum
  | Teacher
  | Book
  | Category
  | Author
  | AuthorBook
  | Librarian
  | Publisher
  | Country
  | StudentCard
  | TeacherCard
  | StudentsBorrowing
  | TeachersBorrowing;

export const getKeys = Object.keys as <T extends object>(
  obj: T
) => Array<keyof T>;

export type PrimaryKey<T extends Entity> = T extends {
  id: NonNullable<unknown>;
}
  ? Extract<keyof T, "id">
  : T extends Country
  ? Extract<keyof T, "code">
  : never;

export type Table =
  | "student"
  | "faculty"
  | "curriculum"
  | "faculty_curriculum"
  | "teacher"
  | "book"
  | "category"
  | "author"
  | "author_book"
  | "librarian"
  | "publisher"
  | "country"
  | "student_card"
  | "teacher_card"
  | "students_borrowing"
  | "teachers_borrowing";

export type TablePrimaryKey<T extends Table> = T extends "country"
  ? "code"
  : "id";

export function tableToPrimaryKey<T extends Table>(
  table: T
): TablePrimaryKey<T> {
  if (table === "country") return "code" as TablePrimaryKey<T>;

  return "id" as TablePrimaryKey<T>;
}

export const defaultModel: Record<Table, Entity> = {
  student: {
    id: 0,
    name: "",
    lastname: "",
    surname: "",
    age: 0,
    faculty_curriculum: 0,
    group: 0,
    start_study_date: new Date().toISOString().split("T")[0],
    status: StudentStatus.Graduated,
  },
  faculty: {
    id: 0,
    name: "",
    letter: "",
  },
  curriculum: {
    id: 0,
    name: "",
    letter: "",
  },
  faculty_curriculum: {
    id: 0,
    faculty: 0,
    curriculum: 0,
  },
  teacher: {
    id: 0,
    name: "",
    lastname: "",
    surname: "",
    age: 0,
    faculty: 0,
    status: TeacherStatus.Moved,
  },
  book: {
    id: 0,
    title: "",
    release: new Date().toISOString().split("T")[0],
    publisher: 0,
    category: 0,
    student_access: false,
  },
  category: {
    id: 0,
    name: "",
  },
  author: {
    id: 0,
    name: "",
    lastname: "",
    surname: "",
    country: "",
  },
  author_book: {
    id: 0,
    author_id: 0,
    book_id: 0,
    num: 0,
  },
  librarian: {
    id: 0,
    name: "",
    lastname: "",
    surname: "",
    age: 0,
  },
  publisher: {
    id: 0,
    name: "",
    country: "",
  },
  country: {
    code: "",
    name: "",
  },
  student_card: {
    id: 0,
    student: 0,
    issue_date: new Date().toISOString().split("T")[0],
  },
  teacher_card: {
    id: 0,
    teacher: 0,
    issue_date: new Date().toISOString().split("T")[0],
  },
  students_borrowing: {
    id: 0,
    student_card: 0,
    librarian: 0,
    book: 0,
    book_status_start: BookStatus.Excellent,
    book_status_finish: BookStatus.Excellent,
    borrow_date: new Date().toISOString().split("T")[0],
    return_date: new Date().toISOString().split("T")[0],
    required_return_date: new Date().toISOString().split("T")[0],
  },
  teachers_borrowing: {
    id: 0,
    teacher_card: 0,
    librarian: 0,
    book: 0,
    book_status_start: BookStatus.Excellent,
    book_status_finish: BookStatus.Excellent,
    borrow_date: new Date().toISOString().split("T")[0],
    return_date: new Date().toISOString().split("T")[0],
  },
};
