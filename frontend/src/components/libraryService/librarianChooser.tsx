import { useEffect, useState } from "react";
import { Form } from "react-bootstrap";
import { Librarian } from "../../model";

type Props = { callback: (librarian: Librarian) => void };

function LibrarianChooser({ callback }: Props) {
  const [librarians, setLibrarians] = useState<Map<string, Librarian>>(
    new Map()
  );

  useEffect(() => {
    getLibrarians().then((librarians) => setLibrarians(librarians));
  }, []);

  return (
    <>
      <Form.Select
        onChange={(e) => callback(librarians.get(e.target.value) as Librarian)}
      >
        <option selected={true} disabled={true}>
          Choose librarian
        </option>
        {Array.from(librarians.values()).map((librarian) => (
          <option
            value={`${librarian.lastname} ${librarian.name} ${librarian.surname}`}
            key={librarian.lastname}
          >
            {`${librarian.lastname} ${librarian.name} ${librarian.surname}`}
          </option>
        ))}
      </Form.Select>
    </>
  );
}

async function getLibrarians(): Promise<Map<string, Librarian>> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/librarian`);
  const data = await res.json();

  const map = new Map<string, Librarian>();
  (data as Librarian[]).forEach((librarian) =>
    map.set(
      `${librarian.lastname} ${librarian.name} ${librarian.surname}`,
      librarian
    )
  );
  return map;
}

export default LibrarianChooser;
