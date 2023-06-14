/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changeUntisData } from "../../../api/theBackend";
import { fetchJSessionId, getLocalUntisCredentials, saveUntisCredentials } from "../../../api/untisAPI";
import { generateKey, passwordEncrypt } from "../../../api/encryption";
import { useState } from "preact/hooks";

export default function ChangeUntisData(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const sendUntisDataChange = (event: any) => {
    event.preventDefault();
    fetchJSessionId(event.target[1].value, event.target[2].value).then((result) => {
      saveUntisCredentials(event.target[1].value, event.target[2].value);
      document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
      const key = generateKey(event.target[0].value);
      const untisCredentials = JSON.stringify({ username: event.target[1].value, password: event.target[2].value });
      const untisCredentialsEncrtypted = passwordEncrypt(key, untisCredentials).toString();
      changeUntisData(event.target[0].value, result.personId, untisCredentialsEncrtypted).then(
        () => {
          setErrorMessage(<p>Deine Untis-Daten wurden geändert</p>);
        },
        (error) => {
          setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
        }
      );
    });
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Ändere deine Untis-Nutzerdaten</h2>
        <form onSubmit={sendUntisDataChange} autocomplete="on">
          <input
            name="current_pwd"
            type="password"
            placeholder="Aktuelles Passwort"
            autocomplete="current-password"
            required
          />

          <input
            type="username"
            name="new_untis_username"
            placeholder="Neuer Untis-Nutzername"
            autocomplete="off"
            required
          />
          <input name="new_untis_pwd" type="password" placeholder="Neues Untis-Passwort" autocomplete="off" required />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
