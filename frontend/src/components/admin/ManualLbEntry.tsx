import { allLbs, ManualLb } from "../../api/main";
import { useEffect, useState } from "preact/hooks";
import "use-bootstrap-select/dist/use-bootstrap-select.css";
import UseBootstrapSelect from "use-bootstrap-select";
import { JSX } from "preact";
type ManualLbEntryProps = ManualLb;

export default function TeacherEntry(
  props: ManualLbEntryProps
): JSX.Element {
  const [confirmDelete, setConfirm] = useState(false);
  const [editStyle, setEditStyle] = useState({ background: ""});
  const [deletedText, setDeletedText] = useState("");

  return (
    <tr>
      <th scope="row" class={editStyle.background} contentEditable>
        {props.teacher}
      </th>
      <td class={editStyle.background} contentEditable>
        {props.day + 1}
      </td>
      <td class={editStyle.background} contentEditable>
        {props.start}
      </td>
      <td class={editStyle.background} contentEditable>
        {props.room}
      </td>
      <td class={editStyle.background}>
        <button
          class="btn btn-danger"
          onClick={() => {
            if (!confirmDelete) {
              setEditStyle({
                background: "bg-danger"
              });
              setDeletedText("Gelöscht");
              setConfirm(true);
            } else {
              setEditStyle({
                background: ""
              });
              setDeletedText("");
              setConfirm(false);
            }
          }}>
          <i class="bi bi-trash"></i>
          {deletedText}
        </button>
      </td>
    </tr>
  );
}