import { useState } from "react";
import "../App.css";
import { Librarian } from "../model";
import LibrarianChooser from "./libraryService/librarianChooser";
import { Tab, Tabs } from "react-bootstrap";
import StudentTable from "./libraryService/studentTable";

function LibraryService() {
  const [librarian, setLibrarian] = useState<Librarian>();

  return (
    <>
      <LibrarianChooser callback={setLibrarian} />
      <Tabs
        defaultActiveKey="table"
        id="fill-tab-example"
        className="mb-3"
        fill
      >
        <Tab eventKey="students" title="Students">
          <StudentTable />
        </Tab>
        <Tab eventKey="teachers" title="Teachers"></Tab>
        <Tab eventKey="books" title="Books"></Tab>
        <Tab eventKey="cards" title="Cards"></Tab>
      </Tabs>
    </>
  );
}

export default LibraryService;
