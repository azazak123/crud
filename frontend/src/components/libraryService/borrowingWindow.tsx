import { useEffect, useState } from "react";
import { BookStatus, TeachersBorrowing } from "../../model";
import { Button, Dropdown, Modal, Table } from "react-bootstrap";

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
};

function BorrowingWindow({
  showInitial,
  card,
  isTeacher,
  owner,
  closeWindow,
}: Props) {
  const [borrowings, setBorrowings] = useState<ReadonlyBorrowing[]>([]);
  const [show, setShow] = useState(showInitial);
  const [update, setUpdate] = useState({});

  useEffect(() => setShow(showInitial), [showInitial]);

  useEffect(() => {
    getBorrowings(isTeacher, card).then((borrowings) =>
      setBorrowings(borrowings)
    );
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

export default BorrowingWindow;
