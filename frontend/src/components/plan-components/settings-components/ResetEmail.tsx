/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { resetEmail } from "../../../api/theBackend";
import { useState } from "preact/hooks";
interface IProps {
  uuid: string;
}

export default function ResetEmail(props: IProps): JSX.Element {
    const [errorMessage, setErrorMessage] = useState(<p></p>)
  const sendPasswordChange = (event: any) => {
    event.preventDefault();
      resetEmail(props.uuid, event.target[0].value).then(() => {
        setErrorMessage(<p>Deine E-Mail-Adresse wurde geändert</p>)
      }, (error) => {
        setErrorMessage(<p>Etwas ist schief gelaufen: {error}</p>)
    });
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Such dir eine neue E-Mail-Adresse aus</h2>
        <form onSubmit={sendPasswordChange} autocomplete="on">
          <input
            name="new_email"
            type="email"
            placeholder="Neue E-Mail-Adresse"
            autocomplete="email"
            title="Gib eine gültige E-Mail-Adresse ein"
            required
          />
          <input type="submit" id="submit-button" />
        </form>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
