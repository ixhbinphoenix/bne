/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { resetPassword } from "../../../api/theBackend";
import { useState } from "preact/hooks";
interface IProps {
  uuid: string
}


export default function ResetPassword(props: IProps): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>)
  const sendPasswordChange = (event: any) => {
    event.preventDefault();
    resetPassword(props.uuid, event.target[0].value).then(() => {
      setErrorMessage(<p>Dein Passwort wurde geändert</p>)
    }, (error) => {
      setErrorMessage(<p>Etwas ist schief gegangen: {error}</p>)
    });
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
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
