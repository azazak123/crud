import { useState } from "react";
import "./App.css";
import EntityTable from "./components/entityTable";
import LibraryService from "./components/libraryService";
import { Tab, Tabs } from "react-bootstrap";

function App() {
  return (
    <>
      <Tabs
        defaultActiveKey="table"
        id="fill-tab-example"
        className="mb-3"
        fill
        mountOnEnter={true}
        unmountOnExit={true}
      >
        <Tab eventKey="crud" title="Crud">
          <EntityTable />
        </Tab>
        <Tab eventKey="library" title="Library">
          <LibraryService />
        </Tab>
      </Tabs>
    </>
  );
}

export default App;
