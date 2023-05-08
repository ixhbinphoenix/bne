/* @jsxImportSource preact */

import "@fontsource/inter/600.css";
import type { JSX } from "preact";
import "../../styles/Settings.scss";
import { useEffect, useState } from "preact/hooks";
import ChangePassword from "./settings-components/ChangePassword";
import ChangeEmail from "./settings-components/ChangeEmail";
import ChangeUntisData from "./settings-components/ChangeUntisData";
import DeleteAccount from "./settings-components/DeleteAccount";
import Logout from "./settings-components/Logout";

export default function Settings(): JSX.Element {
  const [pageContent, setPageContent] = useState(<ChangePassword />);
  const [username, setUsername] = useState("");
  useEffect(() => {
    const usernameRaw = localStorage.getItem("untis_username");
    const nameParts = usernameRaw?.split("_");
    if (nameParts) {
      setUsername(nameParts[1] + " " + nameParts[0]);
    }
  }, []);

  const highlightButton = (button: string) => {
    const buttons = document.getElementsByClassName("settings-button");
    Array.from(buttons).forEach((button) => {
      button.classList.remove("active");
    });
    const activeButton = document.getElementById(button)?.classList.add("active");
  };

  return (
    <div class="settings-page">
      <div id="top-bar">
        <div id="username">{username}</div>
        <div id="settings-bar">
          <button
            class="settings-button active"
            id="button1"
            onClick={() => {
              highlightButton("button1");
              setPageContent(<ChangePassword />);
            }}>
            Passwort ändern
          </button>
          <button
            class="settings-button"
            id="button2"
            onClick={() => {
              highlightButton("button2");
              setPageContent(<ChangeEmail />);
            }}>
            E-Mail-Adresse ändern
          </button>
          <button
            class="settings-button"
            id="button3"
            onClick={() => {
              highlightButton("button3");
              setPageContent(<ChangeUntisData />);
            }}>
            Untis-Daten ändern
          </button>
          <button
            class="settings-button"
            id="button4"
            onClick={() => {
              highlightButton("button4");
              setPageContent(<DeleteAccount />);
            }}>
            Account löschen
          </button>
          <button
            class="settings-button"
            id="button5"
            onClick={() => {
              highlightButton("button5");
              setPageContent(<Logout />);
            }}>
            Abmelden
          </button>
        </div>
      </div>
      <div id="page-content">{pageContent}</div>
      <div id="bottom-bar">
        <a href="/datenschutz">Datenschutzerklärung</a>
        <a href="/password-reset">Passwort vergessen</a>
        <a href="/nutzungsbedingungen">Nutzungsbedingungen</a>
      </div>
    </div>
  );
}
