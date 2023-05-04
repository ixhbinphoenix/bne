/* @jsxImportSource preact */

import Stundenplan from "./plan-components/Stundenplan";
import Kontakt from "./plan-components/Kontakt";
import Lernbuero from "./plan-components/Lernbueros";
import Settings from "./plan-components/Settings";
import { useState } from "preact/hooks";
import type { JSX } from "preact";
import "../styles/Sidebar.css";

export default function Sidebar(): JSX.Element {
  const [activePage, setActivePage] = useState(<Stundenplan />);

  const highlightButton = (button: string) => {
    const buttons = document.getElementsByClassName("sidebar-element");
    for (let i = 0; i < buttons.length; i++) {
      buttons[i].classList.remove("active");
    }
    document.getElementById(button)?.classList.add("active");
  };

  return (
    <div class="background">
      <div class="title">
        <h1>TheSchedule</h1>
      </div>
      <div class="content">
        <div class="sidebar">
          <button
            class="sidebar-element active"
            id="stundenplan"
            onClick={() => {
              setActivePage(<Stundenplan />);
              highlightButton("stundenplan");
            }}>
            Stundenplan
          </button>
          <button
            class="sidebar-element"
            id="lernbueros"
            onClick={() => {
              setActivePage(<Lernbuero />);
              highlightButton("lernbueros");
            }}>
            Lernbüros
          </button>
          <button
            class="sidebar-element"
            id="settings"
            onClick={() => {
              setActivePage(<Settings />);
              highlightButton("settings");
            }}>
            Einstellungen
          </button>
          <button
            class="sidebar-element"
            id="kontakt"
            onClick={() => {
              setActivePage(<Kontakt />);
              highlightButton("kontakt");
            }}>
            Kontakt
          </button>
        </div>
        {activePage}
      </div>
    </div>
  );
}
