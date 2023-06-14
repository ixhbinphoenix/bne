/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { forgotPassword } from "../../../api/theBackend";
import { useState } from "preact/hooks";

export default function ForgotPassword(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const handleSubmit = (event: any) => {
    event.preventDefault();
    forgotPassword(event.target[0].value).then(
      () => {
        setErrorMessage(<p>Du hast eine E-Mail von uns bekommen</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error.message}</p>);
      }
    );
  };

  return (
    <div class="page-content">
      <div class="form-container">
        <h1 style="text-align: center;">Hast du dein Passwort vergessen?</h1>
        <p style="text-align: center;">
          Gib hier deine E-Mail-Adresse an, um eine Wiederherstellungs-E-Mail zu erhalten
        </p>
        <form id="form" onSubmit={handleSubmit}>
          <input
            type="email"
            placeholder="E-Mail-Adresse"
            autocomplete="email"
            pattern="^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
            required
          />
          <input type="submit" value="E-Mail anfordern" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
