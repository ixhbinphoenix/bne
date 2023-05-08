/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changeUntisData } from "../../../api/theBackend";
import { fetchJSessionId, saveUntisCredentials } from "../../../api/untisAPI";
import { generateKey, passwordEncrypt } from "../../../api/encryption";

export default function ChangeUntisData(): JSX.Element {
  const sendUntisDataChange = (event: any) => {
    event.preventDefault();
    fetchJSessionId(event.target[1].value, event.target[2].value).then((result) => {
      if (result.JSessionId && result.personId) {
        saveUntisCredentials(event.target[1].value, event.target[2].value);
        const key = generateKey(event.target[0].value);
        const untisCredentials = JSON.stringify({ username: event.target[1], password: event.target[2].value });
        const untisCredentialsEncrtypted = passwordEncrypt(key, untisCredentials).toString();
        changeUntisData(event.target[0].value, result.personId, untisCredentialsEncrtypted);
      }
    });
  };
  return (
    <div class="new-password-content">
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

          <input type="username" name="new_untis_username" placeholder="Neuer Untis-Nutzername" autocomplete="off" />
          <input name="new_untis_pwd" type="password" placeholder="Neues Untis-Passwort" autocomplete="off" required />
          <input type="submit" id="submit-button" />
        </form>
      </div>
    </div>
  );
}
