import { useEffect, useState } from "react";
import "../App.css";
import TableChooser from "./tableChooser";
import { Button, Table } from "react-bootstrap";
import TableRow from "./tableRow";
import {
  Entity,
  tableToPrimaryKey,
  Table as TableName,
  defaultModel,
} from "../model";

function EntityTable() {
  const [content, setContent] = useState<Entity[]>([]);
  const [currentTable, setCurrentTable] = useState<TableName>("student");
  const [update, updateTable] = useState({});
  const primaryKey = tableToPrimaryKey(currentTable);

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
              ? Object.keys(defaultModel[currentTable]).map((field) => (
                  <th className="p-3" key={field}>
                    {field}
                  </th>
                ))
              : ""}
            <th className="p-3">{content[0] ? "Save" : ""}</th>
            <th className="p-3">{content[0] ? "Delete" : ""}</th>
          </tr>
        </thead>
        <tbody>
          {content.map((entity, i) => (
            <TableRow
              updateTable={() => updateTable({ ...update })}
              table={currentTable}
              primaryKey={primaryKey}
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              key={`${(entity as any)[primaryKey]} + ${i}`}
              entityInitial={entity}
            />
          ))}
        </tbody>
      </Table>
      <div className="d-grid gap-2">
        <Button
          onClick={() => {
            setContent([...content, defaultModel[currentTable]]);
          }}
          size="lg"
        >
          +
        </Button>
      </div>
    </>
  );
}

async function getContent(table: TableName): Promise<Entity[]> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/${table}`);
  const data = await res.json();

  return data;
}

export default EntityTable;
