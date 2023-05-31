import { useEffect, useState } from "react";
import { Student } from "../../model";
import { Table } from "react-bootstrap";

type ReadonlyStudent = Student & { group: string };

function StudentTable() {
  const [students, setStudents] = useState<ReadonlyStudent[]>([]);

  useEffect(() => {
    getStudents().then((students) => setStudents(students));
  }, []);

  return (
    <>
      <Table>
        <thead>
          <tr>
            <th className="p-3">Name</th>
            <th className="p-3">Lastname</th>
            <th className="p-3">Surname</th>
            <th className="p-3">Age</th>
            <th className="p-3">Group</th>
            <th className="p-3">Start study date</th>
            <th className="p-3">Status</th>
          </tr>
        </thead>
        <tbody>
          {students.map((student, i) => (
            <tr key={i}>
              <td className="p-3">{student.name}</td>
              <td className="p-3">{student.lastname}</td>
              <td className="p-3">{student.surname}</td>
              <td className="p-3">{student.age}</td>
              <td className="p-3">{student.group}</td>
              <td className="p-3">{student.start_study_date}</td>
              <td className="p-3">{student.status}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getStudents(): Promise<ReadonlyStudent[]> {
  const studentsRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/student-group`
  );
  const students = (await studentsRes.json()) as ReadonlyStudent[];

  return students;
}

export default StudentTable;
