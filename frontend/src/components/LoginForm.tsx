/* @jsxImportSource preact */

import "../styles/LoginForm.scss";
import type { JSX } from "preact";
import { useState } from "preact/hooks";
import { loginAccount } from "../api/theBackend";
import { fetchJSessionId, saveUntisCredentials } from "../api/untisAPI";
import { generateKey, passwordDecrypt } from "../api/encryption";
import { JSESSIONIDCookieString } from "../api/main";

export default function LoginForm(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState<JSX.Element>(<p></p>);

  const handleSubmit = (event: any) => {
    event.preventDefault();
    sendLogin(event.target[0].value, event.target[1].value);
  };
  const sendLogin = (username: string, password: string) => {
    const key = generateKey(password);
    loginAccount(username, password).then(
      (cypher) => {
        const untisCredentialsDecrypted = JSON.parse(passwordDecrypt(key, cypher));
        saveUntisCredentials(untisCredentialsDecrypted.username, untisCredentialsDecrypted.password);
        fetchJSessionId(untisCredentialsDecrypted.username, untisCredentialsDecrypted.password).then((result) => {
          document.cookie = JSESSIONIDCookieString(result.JSessionId);
          window.location.href = "/home";
        });
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
        <p className="subtitle">Melde dich mit einem bestehenden Account an</p>
      </div>
      <div className="login-box-container">
        <div className="login-switch">
          <button id="login" style="border-bottom: 2px solid #5974e2;">
            Anmelden
          </button>
          <button
            id="register"
            onClick={() => {
              window.location.href = "/registrieren";
            }}>
            Registrieren
          </button>
        </div>
        <form onSubmit={handleSubmit} autocomplete="on">
          <input type="email" placeholder="E-Mail-Adresse" className="input-box" autocomplete="email" required />
          <input
            type="password"
            placeholder="Passwort"
            className="input-box"
            autocomplete="current-password"
            required
          />
          <div className="button-container">
            <input type="submit" id="submit-button" value="Absenden" />
          </div>
        </form>
        <div class="error-message">{errorMessage}</div>
        <a href="/passwort-vergessen">Passwort vergessen</a>
      </div>
    </div>
  );
}
