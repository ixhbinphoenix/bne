import { JSX } from "preact";


interface Props {
  title: string;
  onClick: Function;
}

export default function NewItemButton(props: Props): JSX.Element {
  return (
    <div className="timeline-row">
      <div className="timeline-content">
        <button
          className="btn btn-primary"
          onClick={() => {
            props.onClick();
          }}>
          <span className="bi-plus-circle">&nbsp; {props.title}</span>
        </button>
      </div>
    </div>
  );
}
