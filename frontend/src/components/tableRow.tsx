import { useEffect, useState } from "react";
import { Button, Form } from "react-bootstrap";
import "../App.css";
import { Entity, PrimaryKey, Table, defaultModel, getKeys } from "../model";

type Props<T extends Entity> = {
  entityInitial: T;
  primaryKey: PrimaryKey<T>;
  table: Table;
  updateTable: () => void;
};

function TableRow<T extends Entity>({
  entityInitial,
  primaryKey,
  table,
  updateTable,
}: Props<T>) {
  const [entity, setEntity] = useState(entityInitial);
  const [isChanged, setChanged] = useState(false);
  const [isDeleted, setDeleted] = useState(false);
  const [oldPrimaryKey, setOldPrimaryKey] = useState(entityInitial[primaryKey]);

  useEffect(() => {
    setEntity(entityInitial);
  }, [entityInitial]);

  return (
    <tr className={isDeleted ? "table-danger" : ""}>
      {getKeys(entity).map((field) => (
        <td key={field as string} className="p-3">
          <Form.Control
            readOnly={
              field === primaryKey &&
              table !== "curriculum" &&
              table !== "country"
                ? true
                : false
            }
            type="text"
            value={
              entity[field] !== null &&
              entity[field] !== undefined &&
              (field !== primaryKey || entity[field] !== 0)
                ? (entity[field] as string)
                : ""
            }
            onChange={(e) => {
              setChanged(true);

              let val;

              switch (typeof (defaultModel[table] as T)[field]) {
                case "bigint":
                  val = BigInt(e.target.value);
                  break;
                case "boolean":
                  if (e.target.value === "true" || e.target.value === "false")
                    val = e.target.value === "true";
                  else val = e.target.value;
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

              if (e.target.value === "") val = null;

              setEntity({
                ...entity,
                [field]: val,
              });
            }}
          ></Form.Control>
        </td>
      ))}

      <td className="p-3">
        {isChanged ? (
          <Button
            variant="success"
            size="lg"
            onClick={() => {
              if (oldPrimaryKey === 0 || oldPrimaryKey === "")
                createContent(table, entity).then(setEntity);
              else if (isDeleted)
                deleteContent(table, oldPrimaryKey).then(updateTable);
              else update(table, entity, oldPrimaryKey).then(setEntity);

              setChanged(false);
              setDeleted(false);
              setOldPrimaryKey(entity[primaryKey]);
            }}
          ></Button>
        ) : (
          ""
        )}
      </td>

      <td className="p-3">
        {entity[primaryKey] !== 0 ? (
          <Button
            variant="danger"
            size="lg"
            onClick={() => {
              setDeleted(true);
              setChanged(true);
            }}
          ></Button>
        ) : (
          ""
        )}
      </td>
    </tr>
  );
}

async function update<T extends Entity>(
  table: Table,
  entity: T,
  oldKey: T[PrimaryKey<T>]
) {
  const res = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/${table}/${oldKey}`,
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
  const res = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/${table.replaceAll("_", "-")}`,
    {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(entity),
    }
  );

  const data = await res.json();

  return data as T;
}

async function deleteContent<T extends Entity>(
  table: Table,
  oldKey: T[PrimaryKey<T>]
) {
  const res = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/${table.replaceAll(
      "_",
      "-"
    )}/${oldKey}`,
    {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
    }
  );

  const data = await res.json();

  return data as T;
}

export default TableRow;
