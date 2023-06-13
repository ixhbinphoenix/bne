/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { verifyAccount } from "../../../api/theBackend";
import { useState } from "preact/hooks";
interface IProps {
  uuid: string;
}

export default function VerifyAccount(props: IProps): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const sendVerify = () => {
    verifyAccount(props.uuid).then(
      () => {
        setErrorMessage(<p>Dein Account wurde verifiziert</p>);
      },
      (error) => {
        setErrorMessage(<p>Etwas ist schief gegangen: {error}</p>);
      }
    );
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Verifiziere deinen Account</h2>
        <button onClick={sendVerify}>Verifizieren</button>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
