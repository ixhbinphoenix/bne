/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { logoutEverywhere, logoutHere } from "../../../api/theBackend";

export default function Logout(): JSX.Element {
  return (
    <div class="new-password-content">
      <div class="form-container">
        <h2>Melde dich hier, oder auf allen Geräten ab</h2>
        <button
          onClick={() => {
            logoutHere();
          }}>
          Hier abmelden
        </button>
        <button
          onClick={() => {
            logoutEverywhere();
          }}>
          Auf allen Geräten abmelden
        </button>
      </div>
    </div>
  );
}
