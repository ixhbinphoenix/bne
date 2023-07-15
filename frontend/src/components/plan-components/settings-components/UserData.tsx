/* @jsxImportSource preact */

import type { JSX } from "preact";
import "../../../styles/SettingsElement.scss";
import { useState } from "preact/hooks";
import { GDPRData } from "../../../api/theBackend";

export default function UserData(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const handleSubmit = () => {
    GDPRData().then(
      () => {
        setErrorMessage(<p>Wir haben dir eine E-Mail geschickt</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
      }
    );
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Du kannst die über dich gespeicherten Daten anfordern</h2>
        <button onClick={handleSubmit}>Daten anfordern</button>
      </div>
      <div class="error-message">{errorMessage}</div>
    </div>
  );
}
