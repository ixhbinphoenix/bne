import { allLbs } from "../../api/main";
import { useEffect, useState } from "preact/hooks";
import "use-bootstrap-select/dist/use-bootstrap-select.css";
import UseBootstrapSelect from "use-bootstrap-select";
import { JSX } from "preact";
type TeacherEntryProps = {
  longname: string;
  shortname: string;
  lessons: string[];
};

export default function TeacherEntry(
  props: TeacherEntryProps
): JSX.Element {
  const [confirmDelete, setConfirm] = useState(false);
  const [editStyle, setEditStyle] = useState({ background: ""});
  const [deletedText, setDeletedText] = useState("");
  let select: UseBootstrapSelect | null;
  useEffect(() => {
    select = new UseBootstrapSelect(document.getElementById(`lessons-select-${props.shortname}`)! as HTMLSelectElement)
  });
  


  return (
    <tr>
      <th scope="row" class={editStyle.background} id={`user-${props.shortname}`} contentEditable>
        {props.shortname}
      </th>
      <td
        id={`name-${props.shortname}`}
        class={editStyle.background}
        contentEditable>
        {props.longname}{" "}
      </td>
      <td id={`select-${props.shortname}`} class={editStyle.background}>
        <select id={`lessons-select-${props.shortname}`} class="form-select" multiple data-searchable="true">
          {allLbs.map((lb) => {
            return <option value={lb.toLowerCase()} selected={props.lessons.includes(lb)}>{lb.toUpperCase()}</option>
          })}
        </select>
      </td>
      <td class={editStyle.background}>
        <button class="btn btn-danger" onClick={() => {
          if (!confirmDelete) {
            setEditStyle({
              background: "bg-danger"
            })
            setDeletedText("Gelöscht")
            setConfirm(true)
          }
          else {
            setEditStyle({
              background: ""
            })
            setDeletedText("")
            setConfirm(false)
          }
        }}>
          <i class="bi bi-trash"></i>{deletedText}
        </button>
      </td>
    </tr>
  );
}
