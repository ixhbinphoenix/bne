/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { deleteAccount } from "../../../api/theBackend";
import { useState } from "preact/hooks";

export default function DeleteAccount(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const sendDeleteAccount = (event: any) => {
    event.preventDefault();
    deleteAccount(event.target[0].value).then(
      () => {
        setErrorMessage(<p>Deine Account wurde gelöscht</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error}</p>);
      }
    );
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>
          Wenn du deinen Account löschst, <br />
          kannst du ihn nicht wiederherstellen
        </h2>
        <form onSubmit={sendDeleteAccount} autocomplete="on">
          <input
            name="current_pwd"
            type="password"
            placeholder="Aktuelles Passwort"
            autocomplete="current-password"
            required
          />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
