/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changePassword } from "../../../api/theBackend";

export default function ChangeEmail(): JSX.Element {
  const sendPasswordChange = (event: any) => {
    event.preventDefault();
    changePassword(event.target[0].value, event.target[1].value);
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
      </div>
    </div>
  );
}
