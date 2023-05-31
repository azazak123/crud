import { useEffect, useState } from "react";
import { Book } from "../../model";
import { Table } from "react-bootstrap";

type ReadonlyBook = Book & {
  category: string;
  publisher: string;
  authors: string;
};

function BookTable() {
  const [books, setBooks] = useState<ReadonlyBook[]>([]);

  useEffect(() => {
    getBooks().then((books) => setBooks(books));
  }, []);

  return (
    <>
      <Table>
        <thead>
          <tr>
            <th className="p-3">Title</th>
            <th className="p-3">Release</th>
            <th className="p-3">Publisher</th>
            <th className="p-3">Category</th>
            <th className="p-3">Student access</th>
            <th className="p-3">Authors</th>
          </tr>
        </thead>
        <tbody>
          {books.map((book, i) => (
            <tr key={i}>
              <td className="p-3">{book.title}</td>
              <td className="p-3">{book.release}</td>
              <td className="p-3">{book.publisher}</td>
              <td className="p-3">{book.category}</td>
              <td className="p-3">{book.student_access.toString()}</td>
              <td className="p-3">{book.authors}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getBooks(): Promise<ReadonlyBook[]> {
  const booksRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/book-readonly`
  );
  const books = (await booksRes.json()) as ReadonlyBook[];

  return books;
}

export default BookTable;
