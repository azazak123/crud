import { useEffect, useState } from "react";
import { TeachersBorrowing } from "../../model";
import { Button, Modal, Table } from "react-bootstrap";

type ReadonlyBorrowing = TeachersBorrowing & {
  owner: string;
  librarian_name: string;
  book_title: string;
  required_return_date: string | null;
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

  useEffect(() => setShow(showInitial), [showInitial]);

  const handleClose = () => closeWindow();

  useEffect(() => {
    getBorrowings(isTeacher, card).then((borrowings) =>
      setBorrowings(borrowings)
    );
  }, [isTeacher, card]);

  return (
    <>
      <Modal show={show} onHide={handleClose} centered size="xl">
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
                </tr>
              ))}
            </tbody>
          </Table>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
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

export default BorrowingWindow;
