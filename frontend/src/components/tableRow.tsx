import { useState } from "react";
import { Button, Form } from "react-bootstrap";
import "../App.css";
import { Entity, PrimaryKey, Table, getKeys } from "../model";

type Props<T extends Entity> = {
  entityInitial: T;
  primaryKey: PrimaryKey<T>;
  table: Table;
};

function TableRow<T extends Entity>({
  entityInitial,
  primaryKey,
  table,
}: Props<T>) {
  const [entity, setEntity] = useState(entityInitial);
  const [isChanged, setChanged] = useState(false);

  return (
    <tr>
      {getKeys(entity).map((field) => (
        <td key={field as string} className="p-3">
          <Form.Control
            readOnly={field === primaryKey ? true : false}
            type="text"
            value={entity[field] ? (entity[field] as string) : ""}
            onChange={(e) => {
              setChanged(true);

              let val;

              switch (typeof entity[field]) {
                case "bigint":
                  val = BigInt(e.target.value);
                  break;
                case "boolean":
                  val = e.target.value as unknown as boolean;
                  break;
                case "number":
                  val = +e.target.value;
                  break;
                case "object":
                  val = eval(e.target.value);
                  break;
                case "string":
                  val = e.target.value;
                  break;
                default:
                  throw new Error("Unsupported type");
              }

              setEntity({
                ...entity,
                [field]: val,
              });
            }}
          ></Form.Control>
        </td>
      ))}
      {isChanged ? (
        <td className="p-3">
          <Button
            variant="success"
            size="lg"
            onClick={() => {
              if (entity[primaryKey] !== 0)
                update(table, primaryKey, entity).then(setEntity);
              else createContent(table, entity).then(setEntity);

              setChanged(false);
            }}
          ></Button>
        </td>
      ) : (
        ""
      )}
    </tr>
  );
}

async function update<T extends Entity>(
  table: Table,
  primaryKey: PrimaryKey<T>,
  entity: T
) {
  const res = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/${table}/${entity[primaryKey]}`,
    {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(entity),
    }
  );

  const data = await res.json();

  return data as T;
}

async function createContent<T extends Entity>(
  table: Table,
  entity: T
): Promise<T> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/${table}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(entity),
  });

  const data = await res.json();

  return data as T;
}

export default TableRow;
