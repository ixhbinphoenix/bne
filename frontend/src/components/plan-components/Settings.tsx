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
  const MenuButton = (title: string, route: JSX.Element): JSX.Element => {
    return (
      <button
        class="menu-button"
        onClick={() => {
          setBackButtonStyle({ opacity: "100%" });
          setPageContent(<div>{route}</div>);
        }}>
        {title}
      </button>
    );
  };

  const Menu = (
    <div class="menu">
      {MenuButton("Passwort ändern", <ChangePassword />)}
      {MenuButton("E-Mail ändern", <ChangeEmail />)}
      {MenuButton("Untis-Daten ändern", <ChangeUntisData />)}
      {MenuButton("Account löschen", <DeleteAccount />)}
      {MenuButton("Abmelden", <Logout />)}
    </div>
  );

  const [backButtonStyle, setBackButtonStyle] = useState({ opacity: "0%" });
  const [pageContent, setPageContent] = useState<JSX.Element>(Menu);
  const [username, setUsername] = useState("");
  useEffect(() => {
    const usernameRaw = localStorage.getItem("untis_username");
    const nameParts = usernameRaw?.split("_");
    if (nameParts) {
      setUsername(nameParts[1] + " " + nameParts[0]);
    }
  }, []);

  return (
    <div class="settings-page">
      <div id="top-bar">
        <div id="username">{username}</div>
      </div>
      <button
        class="back-button"
        style={backButtonStyle}
        onClick={() => {
          setPageContent(Menu);
          setBackButtonStyle({ opacity: "0%" });
        }}>
        ❰ Zurück
      </button>
      <div id="page-content">{pageContent}</div>
      <div id="bottom-bar">
        <a href="/datenschutz">Datenschutzerklärung</a>
        <a href="/password-reset">Passwort vergessen</a>
        <a href="/nutzungsbedingungen">Nutzungsbedingungen</a>
      </div>
    </div>
  );
}
