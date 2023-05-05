/* @jsxImportSource preact */

import "../styles/LoginForm.scss";
import type { JSX } from "preact";
import { useEffect } from "preact/hooks";
import { loginAccount, verifySession } from "../api/theBackend";
import { fetchJSessionId, getLocalUntisCredentials, saveUntisCredentials } from "../api/untisAPI";
import { generateKey, passwordDecrypt } from "../api/encryption";

export default function LoginForm(): JSX.Element {

  useEffect(() => {
    verifySession().then((session) => {
      if (session) {
        window.location.href = "/stundenplan";
      }
    });
  }, []);
  const handleSubmit = (event: any) => {
    event.preventDefault();
    sendLogin(event.target[0].value, event.target[1].value);
    fetchJSessionId(localStorage.getItem("untis_username"), localStorage.getItem("untis_password")).then(
      (result) => {
        if (result.JSessionId && result.personId) {
          document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
        } else {
          alert(result.status);
        }
      }
    );      
  };
  const sendLogin = (username: string, password: string) => {
    const key = generateKey(password);
    loginAccount(username, password).then((result) => {
      if (result.status == 200) {
        const untisCredentialsDecrypted = JSON.parse(passwordDecrypt(key, result.cypher));
        saveUntisCredentials(untisCredentialsDecrypted.username, untisCredentialsDecrypted.password);
        fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
          if (result.status == 200) {
            window.location.href = "/home";
          }
        });
      }
    });
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
      </div>
    </div>
  );
}
