/* @jsxImportSource preact */

import "@fontsource/inter/600.css";
import type { JSX } from "preact";
import "../../styles/Settings.scss";
import { useEffect, useState } from "preact/hooks";
import ChangePassword from "./settings-components/ChangePassword";
import DemandEmail from "./settings-components/DemandEmail";
import ChangeUntisData from "./settings-components/ChangeUntisData";
import DeleteAccount from "./settings-components/DeleteAccount";
import Logout from "./settings-components/Logout";
import UserData from "./settings-components/UserData";
import { onSwipe } from "../../api/Touch";
import { accountIsVerified, resendVerifyEmail } from "../../api/theBackend";
import { getCommitHash } from "../../api/main";

export default function Settings(): JSX.Element {
  const [commitHash, setCommitHash] = useState("");
  const [resendMessage, setResendMessage] = useState("Link erneut versenden");
  const [NotVerifiedMessage, setNotVerifiedMessage] = useState({ display: "none" });
  const [TopbarColor, setTopbarColor] = useState({ "background-color": "var(--highlight-blue)" });
  const showNotVerifiedMessage = () => {
    accountIsVerified().catch((error) => {
      setNotVerifiedMessage({ display: "block" });
      setTopbarColor({ "background-color": "var(--highlight-red)" });
    });
  };
  const sendLink = () => {
    resendVerifyEmail().then(
      () => {
        setResendMessage("Link versendet");
      },
      (error) => {
        setResendMessage(`Etwas ist schief gegangen: ${error.message}`);
      }
    );
  };
  const notVerifiedMessageDiv = (
    <div style={NotVerifiedMessage}>
      <p style="margin-bottom: 1vmin; text-align: center">
        Du bist noch nicht verifiziert!
        <br />
        <button style="color: #0010ff; text-decoration: underline; cursor: pointer" onClick={sendLink}>
          {resendMessage}
        </button>
      </p>
    </div>
  );
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
      {MenuButton("E-Mail ändern", <DemandEmail />)}
      {MenuButton("Untis-Daten ändern", <ChangeUntisData />)}
      {MenuButton("Account löschen", <DeleteAccount />)}
      {MenuButton("Abmelden", <Logout />)}
      {MenuButton("Daten anfordern", <UserData />)}
    </div>
  );

  const [backButtonStyle, setBackButtonStyle] = useState({ opacity: "0%" });
  const [pageContent, setPageContent] = useState<JSX.Element>(Menu);
  const [username, setUsername] = useState("");
  useEffect(() => {
    getCommitHash().then(
      (result) => {
        setCommitHash(result)
      }
    )
    showNotVerifiedMessage();
    const usernameRaw = localStorage.getItem("untis_username");
    const nameParts = usernameRaw?.split("_");
    if (nameParts) {
      setUsername(nameParts[1] + " " + nameParts[0]);
    }
  }, []);

  useEffect(() => {
    onSwipe(".settings-page", { direction: "right", renew: true }, () => {
      setPageContent(Menu);
      setBackButtonStyle({ opacity: "0%" });
    });
  }, [pageContent]);

  return (
    <div class="settings-page" >
      <div id="top-bar" style={TopbarColor}>
        <div id="username">{username}</div>
        {notVerifiedMessageDiv}
        <p style="position: absolute; font-size: 1vmin; right: 0%; bottom: 0%;">Version: {commitHash}</p>
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
        <a href="/passwort-vergessen">Passwort vergessen</a>
        <a href="/nutzungsbedingungen">Nutzungsbedingungen</a>
      </div>
    </div>
  );
}
