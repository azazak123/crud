import { useEffect, useState } from "react";
import { Teacher } from "../../model";
import { Table } from "react-bootstrap";

type ReadonlyTeacher = Teacher & { faculty: string };

function TeacherTable() {
  const [teachers, setTeachers] = useState<ReadonlyTeacher[]>([]);

  useEffect(() => {
    getTeachers().then((teachers) => setTeachers(teachers));
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
          {teachers.map((teacher, i) => (
            <tr key={i}>
              <td className="p-3">{teacher.name}</td>
              <td className="p-3">{teacher.lastname}</td>
              <td className="p-3">{teacher.surname}</td>
              <td className="p-3">{teacher.age}</td>
              <td className="p-3">{teacher.faculty}</td>
              <td className="p-3">{teacher.status}</td>
            </tr>
          ))}
        </tbody>
      </Table>
    </>
  );
}

async function getTeachers(): Promise<ReadonlyTeacher[]> {
  const teachersRes = await fetch(
    `${import.meta.env.VITE_SERVER_URL}/teacher-readonly`
  );
  const teachers = (await teachersRes.json()) as ReadonlyTeacher[];

  return teachers;
}

export default TeacherTable;
