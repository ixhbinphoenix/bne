/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { demandEmail } from "../../../api/theBackend";
import { useState } from "preact/hooks";

export default function DemandEmail(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const sendEmailChange = (event: any) => {
    event.preventDefault();
    demandEmail().then(
      () => {
        setErrorMessage(<p>Du hast eine Mail von uns bekommen</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
      }
    );
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Fordere eine E-Mail zum ändern deiner E-Mail-Adresse an</h2>
        <form onSubmit={sendEmailChange} autocomplete="on">
          <input type="submit" id="submit-button" value="E-Mail anfordern" />
        </form>
        <h4>
          Wenn du keinen Zugriff mehr auf deine E-Mail-Adresse hast, kannst du uns eine{" "}
          <a href="mailto:support@theschedule.de?subject=Zugriff auf E-Mail-Adresse verloren">Mail</a> von einer anderen
          Adresse schicken
        </h4>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
