import { useState } from "react";
import "../App.css";
import { Librarian } from "../model";
import LibrarianChooser from "./libraryService/librarianChooser";

function LibraryService() {
  const [librarian, setLibrarian] = useState<Librarian>();

  return (
    <>
      <LibrarianChooser callback={setLibrarian} />
    </>
  );
}

export default LibraryService;