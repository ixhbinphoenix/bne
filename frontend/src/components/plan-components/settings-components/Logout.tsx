/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { logout, logoutAll } from "../../../api/theBackend";

export default function Logout(): JSX.Element {
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Melde dich hier, oder auf allen Geräten ab</h2>
        <button
          onClick={() => {
            logout().then(() => {
              window.location.href = "/login";
            });
          }}>
          Hier abmelden
        </button>
        <button
          onClick={() => {
            logoutAll().then(() => {
              window.location.href = "/login";
            });
          }}>
          Auf allen Geräten abmelden
        </button>
      </div>
    </div>
  );
}
