import "../../styles/Admin.scss";
import NewItemButton from "./NewItemButton";
import { ManualLb, ManualLbOverwrite } from "../../api/main";
import { getManualLbOverwrites, getManualLbs, saveManualLbOverwrites, saveManualLbs } from "../../api/theBackend";
import { useState, useEffect } from "preact/hooks";
import { JSX } from "preact";
import ManualLbOverwriteEntry from "./ManualLbOverwriteEntry";
export default function TeacherTable(): JSX.Element {
  const [lbs, setLbs] = useState<ManualLbOverwrite[]>([]);
  useEffect(() => {
    getManualLbOverwrites().then((res) => {
      setLbs(res.lbs);
    });
  }, []);

  const submitEmployee = async () => {
    const teacherProps: ManualLbOverwrite = {
      start: 1,
      teacher: "LEHRKRAFT",
      day: 0
    };
    setLbs((lbs) => lbs.concat(teacherProps));
  };
  const buildTeachers = () => {
    const allLbs: ManualLbOverwrite[] = [];
    const allElems = document.getElementById("teachers-table")!;
    Array.from(allElems.children).forEach((row, index, array) => {
      if (index == array.length - 1) return;
      let cells = Array.from(row.children);
      //@ts-expect-error
      if (cells[3].innerText != "Gelöscht") {
        //@ts-expect-error
        let teacher = cells[0].innerText;
        //@ts-expect-error
        let day = parseInt(cells[1].innerText);
        //@ts-expect-error
        let start = parseInt(cells[2].innerText);
        let lb = {
          teacher: teacher,
          day: day - 1,
          start: start,
        };
        allLbs.push(lb);
      }
    });
    saveManualLbOverwrites(allLbs).catch(() => {
      alert("Das Admin Passwort ist falsch oder Sie sind nicht angemeldet")
    })
  };
  return (
    <>
      <table class="table m-0 bg-dark" data-bs-theme="dark">
        <thead class="bg-dark">
          <tr class="border-bottom">
            <th class="col-2" scope="col">
              Lehrkraft Kürzel
            </th>
            <th scope="col">Tag</th>
            <th scope="col">Stunde</th>
            <button class="btn btn-primary active my-2" onClick={buildTeachers}>
              <i class="bi bi-floppy"></i>&nbsp;Alle Speichern
            </button>
          </tr>
        </thead>
        <tbody id="teachers-table">
          {lbs.map((lb) => {
            return <ManualLbOverwriteEntry {...lb} />;
          })}

          <tr>
            <td class="text-center border-bottom-0">
              <NewItemButton title="Neue Überschreibung" onClick={submitEmployee} />
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
}
