/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changePassword } from "../../../api/theBackend";
import { useState } from "preact/hooks";
import { getLocalUntisCredentials } from "../../../api/untisAPI";
import { generateKey, passwordEncrypt } from "../../../api/encryption";

export default function ChangePassword(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);

  const sendPasswordChange = (event: any) => {
    event.preventDefault();
    const key = generateKey(event.target[1].value);
    const untisCredentials = JSON.stringify(getLocalUntisCredentials());
    console.log(untisCredentials);
    const untisCypher = passwordEncrypt(key, untisCredentials).toString();
    changePassword(event.target[0].value, event.target[1].value, untisCypher).then(
      () => {
        setErrorMessage(<p>Dein Passwort wurde geändert</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
      }
    );
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Such dir ein neues Passwort aus</h2>
        <form onSubmit={sendPasswordChange} autocomplete="on">
          <input
            name="current_pwd"
            type="password"
            placeholder="Aktuelles Passwort"
            autocomplete="current-password"
            required
          />
          <input
            name="new_pwd"
            type="password"
            placeholder="Neues Passwort"
            autocomplete="new-password"
            pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
            title="Dein Passwort muss mindestens als 8 Zeichen lang sein und eine Zahl, ein Sonderzeichen, einen Klein- und einen Großbuchstaben enthalten"
            required
          />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
