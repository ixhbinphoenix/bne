import "../../styles/Admin.scss";
import NewItemButton from "./NewItemButton";
import TeacherEntry from "./TeacherEntry";
import { Teacher } from "../../api/main";
import { getTeachers, saveTeachers } from "../../api/theBackend";
import { useState, useEffect } from "preact/hooks";
import { JSX } from "preact";
export default function TeacherTable(): JSX.Element {
  const [users, setUsers] = useState<Teacher[]>([]);
  useEffect(() => {
    getTeachers().then((res) => {
      setUsers(res.teachers);
    });
  }, []);

  const submitEmployee = async () => {
    const teacherProps: Teacher = {
      longname: "Neue Lehrkraft",
      shortname: "NEU",
      lessons: []
    };
    setUsers((users) => users.concat(teacherProps));
  };
  const buildTeachers = () => {
    const allTeachers: Teacher[] = [];
    const allElems = document.getElementById("teachers-table")!;
    Array.from(allElems.children).forEach((row, index, array) => {
      if (index == array.length - 1) return;
      let cells = Array.from(row.children);
      //@ts-expect-error
      if (cells[3].innerText != "Gelöscht") {
        //@ts-expect-error
        let shortname = cells[0].innerText;
        //@ts-expect-error
        let longname = cells[1].innerText;
        let lessons: string[] = [];
        Array.from(cells[2].children[1].firstChild!.firstChild!.firstChild!.childNodes).forEach((lesson) => {
          //@ts-expect-error
          lessons.push(lesson.textContent);
        });
        lessons.pop();
        console.log(lessons);
        let teacher = {
          longname: longname,
          shortname: shortname,
          lessons: lessons
        };
        allTeachers.push(teacher);
      }
    });
    saveTeachers(allTeachers).catch(() => {
      alert("Das Admin Passwort ist falsch oder Sie sind nicht angemeldet");
    });
  };
  return (
    <>
      <table class="table m-0 bg-dark" data-bs-theme="dark">
        <thead class="bg-dark">
          <tr class="border-bottom">
            <th class="col-1" scope="col">
              Kürzel
            </th>
            <th scope="col">Name</th>
            <th scope="col">Fächer</th>
            <button class="btn btn-primary active my-2" onClick={buildTeachers}>
              <i class="bi bi-floppy"></i>&nbsp;Alle Speichern
            </button>
          </tr>
        </thead>
        <tbody id="teachers-table">
          {users.map((user) => {
            return <TeacherEntry {...user} />;
          })}

          <tr>
            <td class="text-center border-bottom-0">
              <NewItemButton title="Neue Lehrkraft" onClick={submitEmployee} />
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
}
