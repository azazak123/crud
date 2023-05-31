import { useEffect, useState } from "react";
import { Teacher } from "../../model";
import { Table } from "react-bootstrap";

type ReadonlyTeacher = Teacher & { faculty: string };

function TeacherTable() {
  const [students, setStudents] = useState<ReadonlyTeacher[]>([]);

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
            <th className="p-3">Faculty</th>
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
              <td className="p-3">{student.faculty}</td>
              <td className="p-3">{student.status}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getStudents(): Promise<ReadonlyTeacher[]> {
  const teachersRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/teacher-readonly`
  );
  const teachers = (await teachersRes.json()) as ReadonlyTeacher[];

  return teachers;
}

export default TeacherTable;
