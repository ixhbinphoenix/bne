/* @jsxImportSource preact */

import "../styles/LoginForm.scss";
import type { JSX } from "preact";
import { useState } from "preact/hooks";
import { registerAccount } from "../api/theBackend";
import { fetchJSessionId, saveUntisCredentials } from "../api/untisAPI";
import { generateKey, passwordEncrypt } from "../api/encryption";

export default function LoginForm(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState<JSX.Element>(<p></p>);

  const handleSubmit = (event: any) => {
    event.preventDefault();
    if (event.target[1].value !== event.target[2].value) {
      return setErrorMessage(<p>Deine Passwörter stimmen nicht überein</p>);
    }
    saveUntisCredentials(event.target[3].value, event.target[4].value);
    fetchJSessionId(event.target[3].value, event.target[4].value).then(
      (result) => {
        if (result.JSessionId && result.personId) {
          document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
          sendRegister(
            event.target[0].value,
            event.target[1].value,
            result.personId,
            event.target[3].value,
            event.target[4].value
          );
        }
      },
      (error) => {
        setErrorMessage(error.message);
      }
    );
  };
  const sendRegister = (
    username: string,
    password: string,
    personId: number,
    untisUsername: string,
    untisPassword: string
  ) => {
    const key = generateKey(password);
    const untisCredentials = JSON.stringify({ username: untisUsername, password: untisPassword });
    const untisCredentialsEncrtypted = passwordEncrypt(key, untisCredentials).toString();

    registerAccount(username, password, personId, untisCredentialsEncrtypted).then(
      () => {
        window.location.href = "/home";
      },
      (error) => {
        setErrorMessage(error.message);
      }
    );
  };
  return (
    <div className="form-container">
      <div className="description">
        <h1 className="title">Willkommen</h1>
        <p className="subtitle">Erstelle dir einen neuen Account</p>
      </div>
      <div className="login-box-container">
        <div className="login-switch">
          <button
            id="login"
            onClick={() => {
              window.location.href = "/login";
            }}>
            Anmelden
          </button>
          <button id="register" style="border-bottom: 2px solid #5974e2">
            Registrieren
          </button>
        </div>
        <form onSubmit={handleSubmit}>
          <input
            type="email"
            placeholder="E-Mail-Adresse"
            className="input-box"
            autocomplete="email"
            pattern="^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
            required
          />
          <input
            type="password"
            title="Dein Passwort muss mindestens 8 Zeichen lang sein, ein Zahl, einen Groß-, einen Kleinbuchstaben und ein Sonderzeichen enthalten"
            placeholder="Passwort"
            className="input-box"
            required
            autocomplete="new-password"
            pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
          />
          <input
            type="password"
            title="Dein Passwort muss mindestens 8 Zeichen lang sein, ein Zahl, einen Groß-, einen Kleinbuchstaben und ein Sonderzeichen enthalten"
            placeholder="Passwort wiederholen"
            className="input-box"
            required
            autocomplete="new-password"
            pattern="^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$"
          />
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
          <div className="button-container">
            <input type="submit" id="submit-button" value="Absenden" />
          </div>
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
