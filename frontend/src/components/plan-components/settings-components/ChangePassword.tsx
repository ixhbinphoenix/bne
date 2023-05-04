/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss"
import type { JSX } from "preact";

export default function ChangePassword(): JSX.Element {
    return (
      <div class="new-password-content">
        <div class="form-container">
            <h2>Such dir ein neues Passwort aus</h2>
            <form action="http://localhost:8089" autocomplete="on" method="post" target="dummyframe">
            <input name="current_pwd" type="password" placeholder="Aktuelles Passwort" autocomplete="current-password" />
              <input
                name="new_pwd"
                type="password"
                placeholder="Neues Passwort"
                autocomplete="new-password"
                pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
                title="Dein Passwort muss mindestens als 8 Zeichen lang sein und eine Zahl, ein Sonderzeichen, einen Klein- und einen Großbuchstaben enthalten"
              />
              <input type="submit" id="submit-button" />
          </form>
          <iframe name="dummyframe" id="dummyframe" style="display: none"></iframe>
        </div>
      </div>
    );
}
