import { useState } from "react";
import "../App.css";
import { Librarian } from "../model";
import LibrarianChooser from "./libraryService/librarianChooser";
import { Tab, Tabs } from "react-bootstrap";
import StudentTable from "./libraryService/studentTable";
import TeacherTable from "./libraryService/teacherTable";
import BookTable from "./libraryService/bookTable";
import CardTable from "./libraryService/cardTable";

function LibraryService() {
  const [librarian, setLibrarian] = useState<Librarian>();

  return (
    <>
      <LibrarianChooser callback={setLibrarian} />
      {librarian ? (
        <Tabs
          defaultActiveKey="table"
          id="fill-tab-example"
          className="mb-3"
          fill
        >
          <Tab eventKey="students" title="Students">
            <StudentTable />
          </Tab>
          <Tab eventKey="teachers" title="Teachers">
            <TeacherTable />
          </Tab>
          <Tab eventKey="books" title="Books">
            <BookTable />
          </Tab>
          <Tab eventKey="cards" title="Cards">
            <CardTable librarian={librarian as Librarian} />
          </Tab>
        </Tabs>
      ) : (
        ""
      )}
    </>
  );
}

export default LibraryService;
