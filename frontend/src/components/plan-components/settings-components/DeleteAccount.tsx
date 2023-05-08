/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { deleteAccount } from "../../../api/theBackend";

export default function DeleteAccount(): JSX.Element {
  const sendDeleteAccount = (event: any) => {
    event.preventDefault();
    deleteAccount(event.target[0].value);
  };
  return (
    <div class="new-password-content">
      <div class="form-container">
        <h2>
          Wenn du deine Account löschst, <br />
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
      </div>
    </div>
  );
}
