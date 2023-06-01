import { useEffect, useState } from "react";
import { Book, BookStatus, Librarian, TeachersBorrowing } from "../../model";
import {
  Button,
  Col,
  Container,
  Dropdown,
  Form,
  Modal,
  Row,
  Table,
} from "react-bootstrap";

type ReadonlyBorrowing = Omit<TeachersBorrowing, "teacher_card"> & {
  owner: string;
  librarian_name: string;
  book_title: string;
  required_return_date: string | null;
  card: number;
};

type Props = {
  showInitial: boolean;
  card: number;
  isTeacher: boolean;
  owner: string;
  closeWindow: () => void;
  librarian: Librarian;
};

function BorrowingWindow({
  showInitial,
  card,
  isTeacher,
  owner,
  closeWindow,
  librarian,
}: Props) {
  const [borrowings, setBorrowings] = useState<ReadonlyBorrowing[]>([]);
  const [show, setShow] = useState(showInitial);
  const [books, setBooks] = useState<Book[]>([]);
  const [newBorrowing, setNewBorrowing] = useState<Partial<ReadonlyBorrowing>>(
    isTeacher
      ? {
          book_status_start: BookStatus.Excellent,
        }
      : {
          book_status_start: BookStatus.Excellent,
          required_return_date: new Date().toISOString().split("T")[0],
        }
  );
  const [update, setUpdate] = useState({});

  useEffect(() => setShow(showInitial), [showInitial]);

  useEffect(() => {
    getBorrowings(isTeacher, card).then((borrowings) =>
      setBorrowings(borrowings)
    );
  }, [isTeacher, card, update]);

  useEffect(() => {
    getBooks().then((books) => setBooks(books));
  }, [isTeacher, card, update]);

  return (
    <>
      <Modal show={show} onHide={closeWindow} centered size="xl">
        <Modal.Header closeButton>
          <Modal.Title>{owner}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Table>
            <thead>
              <tr>
                <th className="p-3">Librarian</th>
                <th className="p-3">Book</th>
                <th className="p-3">Book initial status</th>
                <th className="p-3">Book finish status</th>
                <th className="p-3">Borrow date</th>
                <th className="p-3">Return date</th>
                <th className="p-3">Required return date</th>
                <th className="p-3">Return book</th>
              </tr>
            </thead>
            <tbody>
              {borrowings.map((borrowing, i) => (
                <tr key={i}>
                  <td className="p-3">{borrowing.librarian_name}</td>
                  <td className="p-3">{borrowing.book_title}</td>
                  <td className="p-3">{borrowing.book_status_start}</td>
                  <td className="p-3">{borrowing.book_status_finish}</td>
                  <td className="p-3">{borrowing.borrow_date}</td>
                  <td className="p-3">{borrowing.return_date}</td>
                  <td className="p-3">{borrowing.required_return_date}</td>
                  {!borrowing.return_date ? (
                    <td className="p-3">
                      <Dropdown>
                        <Dropdown.Toggle variant="warning" id="dropdown-basic">
                          Return
                        </Dropdown.Toggle>

                        <Dropdown.Menu>
                          <Dropdown.Item
                            onClick={() =>
                              returnBook(
                                isTeacher,
                                borrowing,
                                BookStatus.Excellent
                              ).then(() => setUpdate({}))
                            }
                          >
                            Excellent
                          </Dropdown.Item>
                          <Dropdown.Item
                            onClick={() =>
                              returnBook(
                                isTeacher,
                                borrowing,
                                BookStatus.Good
                              ).then(() => setUpdate({}))
                            }
                          >
                            Good
                          </Dropdown.Item>
                          <Dropdown.Item
                            onClick={() =>
                              returnBook(
                                isTeacher,
                                borrowing,
                                BookStatus.Satisfactory
                              ).then(() => setUpdate({}))
                            }
                          >
                            Satisfactory
                          </Dropdown.Item>
                          <Dropdown.Item
                            onClick={() =>
                              returnBook(
                                isTeacher,
                                borrowing,
                                BookStatus.Unsatisfactory
                              ).then(() => setUpdate({}))
                            }
                          >
                            Unsatisfactory
                          </Dropdown.Item>
                        </Dropdown.Menu>
                      </Dropdown>
                    </td>
                  ) : (
                    ""
                  )}
                </tr>
              ))}
            </tbody>
          </Table>

          <Container>
            <Row>
              <Col>
                {" "}
                <Form.Select
                  onChange={(e) =>
                    setNewBorrowing({
                      ...newBorrowing,
                      book: books[e.target.value as unknown as number].id,
                    })
                  }
                >
                  <option selected={true} disabled={true}>
                    Choose book
                  </option>
                  {books.map((book, i) => (
                    <option key={i} value={i}>
                      {book.title}
                    </option>
                  ))}
                </Form.Select>
              </Col>
              <Col>
                {" "}
                <Form.Select
                  value={newBorrowing.book_status_start}
                  onChange={(e) =>
                    setNewBorrowing({
                      ...newBorrowing,
                      book_status_start: e.target
                        .value as unknown as BookStatus,
                    })
                  }
                >
                  <option value={BookStatus.Excellent}>Excellent</option>
                  <option value={BookStatus.Good}>Good</option>
                  <option value={BookStatus.Satisfactory}>Satisfactory</option>
                  <option value={BookStatus.Unsatisfactory}>
                    Unsatisfactory
                  </option>
                </Form.Select>
              </Col>
              <Col>
                {!isTeacher ? (
                  <Form.Control
                    type="text"
                    value={newBorrowing.required_return_date as string}
                    onChange={(e) =>
                      setNewBorrowing({
                        ...newBorrowing,
                        required_return_date: e.target.value,
                      })
                    }
                  />
                ) : (
                  ""
                )}
              </Col>
              <Col>
                <Button
                  variant="primary"
                  onClick={() =>
                    createBorrowing(
                      isTeacher,
                      card,
                      newBorrowing.book as number,
                      newBorrowing.book_status_start as BookStatus,
                      newBorrowing.required_return_date as string,
                      librarian.id
                    ).then(() => setUpdate({}))
                  }
                >
                  Borrow
                </Button>
              </Col>
            </Row>
          </Container>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={closeWindow}>
            Close
          </Button>
        </Modal.Footer>
      </Modal>
    </>
  );
}

async function getBorrowings(
  isTeacher: boolean,
  card: number
): Promise<ReadonlyBorrowing[]> {
  const borrowingsRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/borrowing-readonly/${isTeacher}/${card}`
  );
  const borrowings = (await borrowingsRes.json()) as ReadonlyBorrowing[];

  return borrowings;
}

async function returnBook(
  isTeacher: boolean,
  borrowing: ReadonlyBorrowing,
  finishStatus: BookStatus
): Promise<void> {
  if (isTeacher) {
    await fetch(
      `${import.meta.env.VITE_SERVER_URL}/teachers-borrowing/${borrowing.id}`,
      {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          id: borrowing.id,
          teacher_card: borrowing.card,
          librarian: borrowing.librarian,
          book: borrowing.book,
          book_status_start: borrowing.book_status_start,
          book_status_finish: finishStatus,
          borrow_date: borrowing.borrow_date,
          return_date: new Date().toISOString().split("T")[0],
        }),
      }
    );
  } else {
    await fetch(
      `${import.meta.env.VITE_SERVER_URL}/students-borrowing/${borrowing.id}`,
      {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          id: borrowing.id,
          student_card: borrowing.card,
          librarian: borrowing.librarian,
          book: borrowing.book,
          book_status_start: borrowing.book_status_start,
          book_status_finish: finishStatus,
          borrow_date: borrowing.borrow_date,
          return_date: new Date().toISOString().split("T")[0],
          required_return_date: borrowing.required_return_date,
        }),
      }
    );
  }
}

async function createBorrowing(
  isTeacher: boolean,
  card: number,
  book: number,
  startStatus: BookStatus,
  requiredReturnDate: string,
  librarian: number
): Promise<void> {
  if (isTeacher) {
    await fetch(`${import.meta.env.VITE_SERVER_URL}/teachers-borrowing`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        id: 0,
        teacher_card: card,
        librarian: librarian,
        book: book,
        book_status_start: startStatus,
        book_status_finish: null,
        borrow_date: new Date().toISOString().split("T")[0],
        return_date: null,
      }),
    });
  } else {
    await fetch(`${import.meta.env.VITE_SERVER_URL}/students-borrowing`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        id: 0,
        student_card: card,
        librarian: librarian,
        book: book,
        book_status_start: startStatus,
        book_status_finish: null,
        borrow_date: new Date().toISOString().split("T")[0],
        return_date: null,
        required_return_date: requiredReturnDate,
      }),
    });
  }
}

async function getBooks(): Promise<Book[]> {
  const booksRes = await fetch(`${import.meta.env.VITE_SERVER_URL}/book`);
  const books = (await booksRes.json()) as Book[];

  return books;
}

export default BorrowingWindow;
