import { useEffect, useState } from "react";
import "../App.css";
import TableChooser from "./tableChooser";
import { Col, Container, Form, Row } from "react-bootstrap";

function Table() {
  const [content, setContent] = useState<any>([]);
  const [currentTable, setCurrentTable] = useState("");

  useEffect(() => {
    getContent(currentTable).then((content) => setContent(content));
  }, [currentTable]);

  return (
    <>
      <TableChooser callback={setCurrentTable} />
      <p />
      <Container fluid>
        <Row>
          {content[0]
            ? Object.keys(content[0]).map((field) => (
                <Col key={field}>
                  <Row className="p-3">{field}</Row>
                  {content.map((v, i) => (
                    <Row key={i} className="p-3">
                      <Form.Control type="text" value={v[field]}></Form.Control>
                    </Row>
                  ))}
                </Col>
              ))
            : ""}
        </Row>
      </Container>
    </>
  );
}

async function getContent(table: string): Promise<any> {
  const res = await fetch(`${import.meta.env.VITE_SERVER_URL}/${table}`);
  const data = await res.json();

  console.log(data);

  return data;
}

export default Table;
