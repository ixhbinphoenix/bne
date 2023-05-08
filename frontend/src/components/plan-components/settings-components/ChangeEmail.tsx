/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { changeEmail } from "../../../api/theBackend";

export default function ChangePassword(): JSX.Element {
  const sendEmailChange = (event: any) => {
    event.preventDefault();
    changeEmail(event.target[0].value, event.target[1].value);
  };
  return (
    <div class="new-password-content">
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
      </div>
    </div>
  );
}
