/* @jsxImportSource preact */

import { useEffect, useState } from "preact/hooks";
import type { JSX } from "preact";
import "../styles/Sidebar.css";
import { lazy, Suspense } from "preact/compat";
import Loading from "./Loading";

const Stundenplan = lazy(() => import("./plan-components/Stundenplan"));
const Lernbuero = lazy(() => import("./plan-components/Lernbueros"));
const FreeRooms = lazy(() => import("./plan-components/FreeRooms"));
const Settings = lazy(() => import("./plan-components/Settings"));

export default function Sidebar(): JSX.Element {
  const [activePage, setActivePage] = useState<JSX.Element>();

  useEffect(() => {
    setActivePage(<Stundenplan />);
  }, []);

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
            id="freiräume"
            onClick={() => {
              setActivePage(<FreeRooms />);
              highlightButton("freiräume");
            }}>
            Freiräume
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
        </div>
        <Suspense fallback={<Loading />}>{activePage}</Suspense>
      </div>
    </div>
  );
}
