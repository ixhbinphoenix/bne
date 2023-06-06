/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changeEmail } from "../../../api/theBackend";
import { useState } from "preact/hooks";

export default function ChangeEmail(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>)
  const sendEmailChange = (event: any) => {
    event.preventDefault();
    changeEmail(event.target[0].value, event.target[1].value).then(() => { setErrorMessage(<p>Deine E-Mail-Adresse wurde geändert</p>) }, (error) => {
      setErrorMessage(<p>Etwas ist schief gegangen: {error}</p>)
    });
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Ändere deine E-Mail-Adresse</h2>
        <form onSubmit={sendEmailChange} autocomplete="on">
          <input
            name="current_pwd"
            type="password"
            placeholder="Aktuelles Passwort"
            autocomplete="current-password"
            required
          />
          <input name="new_email" type="email" placeholder="Neue E-Mail-Adresse" autocomplete="email" required />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
