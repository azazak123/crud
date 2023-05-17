import { useEffect, useState } from "react";
import "../App.css";
import TableChooser from "./tableChooser";
import { Table } from "react-bootstrap";
import TableRow from "./tableRow";
import { Entity, tableToPrimaryKey, Table as TableName } from "../model";

function EntityTable() {
  const [content, setContent] = useState<Entity[]>([]);
  const [currentTable, setCurrentTable] = useState<TableName>("student");

  useEffect(() => {
    getContent(currentTable).then((content) => setContent(content));
  }, [currentTable]);

  return (
    <>
      <TableChooser callback={setCurrentTable} />
      <Table>
        <thead>
          <tr>
            {content[0]
              ? Object.keys(content[0]).map((field) => (
                  <th className="p-3" key={field}>
                    {field}
                  </th>
                ))
              : ""}
            <th className="p-3">{content[0] ? "Save" : ""}</th>
          </tr>
        </thead>
        <tbody>
          {content.map((entity, i) => (
            <TableRow
              table={currentTable}
              primaryKey={tableToPrimaryKey(currentTable)}
              key={i}
              entityInitial={entity}
            />
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getContent(table: TableName): Promise<Entity[]> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/${table}`);
  const data = await res.json();

  return data;
}

export default EntityTable;
