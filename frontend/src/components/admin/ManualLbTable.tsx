import "../../styles/Admin.scss";
import NewItemButton from "./NewItemButton";
import { ManualLb } from "../../api/main";
import { getManualLbs, saveManualLbs } from "../../api/theBackend";
import { useState, useEffect } from "preact/hooks";
import { JSX } from "preact";
import ManualLbEntry from "./ManualLbEntry";
export default function TeacherTable(): JSX.Element {
  const [lbs, setLbs] = useState<ManualLb[]>([]);
  useEffect(() => {
    getManualLbs().then((res) => {
      setLbs(res.lbs);
    });
  }, []);

  const submitEmployee = async () => {
    const teacherProps: ManualLb = {
      room: "RAUM",
      start: 1,
      teacher: "LEHRKRAFT",
      day: 0
    };
    setLbs((lbs) => lbs.concat(teacherProps));
  };
  const buildTeachers = () => {
    const allLbs: ManualLb[] = [];
    const allElems = document.getElementById("teachers-table")!;
    Array.from(allElems.children).forEach((row, index, array) => {
      if (index == array.length - 1) return;
      let cells = Array.from(row.children);
      //@ts-expect-error
      if (cells[4].innerText != "Gelöscht") {
        //@ts-expect-error
        let teacher = cells[0].innerText;
        //@ts-expect-error
        let day = parseInt(cells[1].innerText);
        //@ts-expect-error
        let start = parseInt(cells[2].innerText);
        //@ts-expect-error
        let room = cells[3].innerText;
        let lb = {
          teacher: teacher,
          day: day - 1,
          start: start,
          room: room
        };
        allLbs.push(lb);
      }
    });
    saveManualLbs(allLbs).catch(() => {
      alert("Das Admin Passwort ist falsch oder Sie sind nicht angemeldet");
    });
  };
  return (
    <>
      <table class="table m-0 bg-dark" data-bs-theme="dark">
        <thead class="bg-dark">
          <tr class="border-bottom">
            <th class="col-1" scope="col">
              Lehrkraft Kürzel
            </th>
            <th scope="col">Tag</th>
            <th scope="col">Stunde</th>
            <th>Raum</th>
            <button class="btn btn-primary active my-2" onClick={buildTeachers}>
              <i class="bi bi-floppy"></i>&nbsp;Alle Speichern
            </button>
          </tr>
        </thead>
        <tbody id="teachers-table">
          {lbs.map((lb) => {
            return <ManualLbEntry {...lb} />;
          })}

          <tr>
            <td class="text-center border-bottom-0">
              <NewItemButton title="Neues Lernbüro" onClick={submitEmployee} />
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
}
