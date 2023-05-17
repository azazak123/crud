import { useEffect, useState } from "react";
import { Form } from "react-bootstrap";
import "../App.css";
import { Table } from "../model";

type Props = { callback: (table: Table) => void };

function TableChooser({ callback }: Props) {
  const [tables, setTables] = useState<Table[]>([]);

  useEffect(() => {
    getTables().then((tables) => setTables(tables));
  }, []);

  return (
    <>
      <Form.Select onChange={(e) => callback(e.target.value as Table)}>
        {tables.map((table) => (
          <option value={table} key={table}>
            {table}
          </option>
        ))}
      </Form.Select>
    </>
  );
}

async function getTables(): Promise<Table[]> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/table`);
  const data = await res.json();
  return data as Table[];
}

export default TableChooser;
