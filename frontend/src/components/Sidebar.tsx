/* @jsxImportSource preact */

import Plan from "./Plan";
import { useState } from "preact/hooks";
import type { JSX } from "preact";
import "../styles/Sidebar.css";

export default function Sidebar(): JSX.Element {

    const [activePage, setActivePage] = useState("stundenplan")

    return(
        <div class="background">
            <div class="title">
                <h1>TheSchedule</h1>
            </div>
            <div class="content">
                <div class="sidebar">
                    <button class="sidebar-element" onClick={() => setActivePage("stundenplan")}>Stundenplan</button>
                    <button class="sidebar-element" onClick={() => setActivePage("lernbueros")}>Lernbüros</button>
                    <button class="sidebar-element" onClick={() => setActivePage("fehler")}>Fehler</button>
                    <button class="sidebar-element" onClick={() => setActivePage("kontakt")}>Kontakt</button>
                </div>
                <Plan activePage={activePage}></Plan>
            </div>
        </div>
    )
}