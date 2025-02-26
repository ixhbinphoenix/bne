/* @jsxImportSource preact */

import "../../styles/LoginForm.scss";
import type { JSX } from "preact";
import { useState } from "preact/hooks";

export default function LoginForm(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState<JSX.Element>(<p></p>);

  const handleSubmit = (event: any) => {
    event.preventDefault();
    sendLogin(event.target[0].value);
  };
  const sendLogin = (password: string) => {
    document.cookie = `admin_password=${password}; max-age=600; secure; samesite=none; domain=${import.meta.env.PUBLIC_COOKIE_DOMAIN}; path=/;`;
    document.location.href = "./lehrkraefte"
  };
  return (
    <div className="form-container">
      <div className="description">
        <h1 className="title">Admin</h1>
        <p className="subtitle">Gib das Admin Passwort ein</p>
      </div>
      <div className="login-box-container">
        <form onSubmit={handleSubmit} autocomplete="on">
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
      </div>
    </div>
  );
}
