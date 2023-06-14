/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { resetPassword } from "../../../api/theBackend";
import { useState } from "preact/hooks";
import { generateKey, passwordEncrypt } from "../../../api/encryption";
import { fetchJSessionId } from "../../../api/untisAPI";
interface IProps {
  uuid: string;
}

export default function ResetPassword(props: IProps): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const sendPasswordChange = (event: any) => {
    event.preventDefault();
    if (event.target[0].value !== event.target[1].value) {
      return setErrorMessage(<p>Deine Passwörter stimmen nicht überein</p>)
    }
    const key = generateKey(event.target[0].value);
    const untisCredentials = { username: event.target[2].value, password: event.target[3].value };
    const untisCredentialsEncrypted = passwordEncrypt(key, JSON.stringify(untisCredentials)).toString();
    fetchJSessionId(untisCredentials.username, untisCredentials.password).then(
      (result) => {
        resetPassword(props.uuid, event.target[0].value, untisCredentialsEncrypted, result.personId).then(
          () => {
            setErrorMessage(<p>Dein Passwort wurde geändert</p>);
          },
          (error) => {
            setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
          }
        );
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
            name="new_pwd"
            type="password"
            placeholder="Neues Passwort"
            autocomplete="new-password"
            pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
            title="Dein Passwort muss mindestens als 8 Zeichen lang sein und eine Zahl, ein Sonderzeichen, einen Klein- und einen Großbuchstaben enthalten"
            required
          />
          <input
            name="new_pwd"
            type="password"
            placeholder="Neues Passwort wiederholen"
            autocomplete="new-password"
            pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
            title="Dein Passwort muss mindestens als 8 Zeichen lang sein und eine Zahl, ein Sonderzeichen, einen Klein- und einen Großbuchstaben enthalten"
            required
          />
          <p style="text-align: center;">
            Du musst deine Untis-Nutzerdaten nochmal eingeben, damit wir sie neu verschlüsseln können
          </p>
          <input
            id="untis-username"
            type="username"
            placeholder="Untis-Nutzername"
            className="input-box untis-box"
            autocomplete="off"
            required
          />
          <input
            type="password"
            placeholder="Untis-Passwort"
            className="input-box untis-box"
            autocomplete="off"
            required
          />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
