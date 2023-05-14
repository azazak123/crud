import { useEffect, useState } from "react";
import "../App.css";

type Props = { callback: (table: string) => void };

function TableChooser({ callback }: Props) {
  const [tables, setTables] = useState([]);
  // const [currentTable, setCurrentTable] = useState();

  useEffect(() => {
    getTables().then((tables) => setTables(tables));
  }, []);

  return (
    <>
      <select onChange={(e) => callback(e.target.value)}>
        {tables.map((table) => (
          <option value={table} key={table}>
            {table}
          </option>
        ))}
      </select>
    </>
  );
}

async function getTables() {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/table`);
  const data = await res.json();
  return data;
}

export default TableChooser;
