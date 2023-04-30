/* @jsxImportSource preact */
import type { JSX } from "preact";
import "../../styles/Kontakt.scss";

export default function Kontakt(): JSX.Element {
  return (
    <div className="contact">
      <h1>Gibt es Fragen zu dieser App?</h1>
      <div>
        <p>
          Schicken Sie uns gerne eine <a>eMail</a>.
        </p>
        <p>
          Unsere Lizenz kann <a href="https://raw.githubusercontent.com/ixhbinphoenix/bne/master/LICENSE">hier</a>{" "}
          eingesehen werden.
        </p>
      </div>
    </div>
  );
}
