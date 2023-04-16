/* @jsxImportSource preact */

import Plan from "./Plan";
import { useState } from "preact/hooks";
import type { JSX } from "preact";
import "../styles/Sidebar.css";

export default function Sidebar(): JSX.Element {

    const [activePage, setActivePage] = useState("stundenplan")

    const highlightButton = (button: string) => {
        const buttons = document.getElementsByClassName("sidebar-element")
        for(let i = 0; i < buttons.length; i++) {
            buttons[i].classList.remove("active")
        }
        document.getElementById(button)?.classList.add("active")
    }

    return(
        <div class="background">
            <div class="title">
                <h1>TheSchedule</h1>
            </div>
            <div class="content">
                <div class="sidebar">
                    <button class="sidebar-element active" id="stundenplan" onClick={() => {setActivePage("stundenplan"); highlightButton("stundenplan")}}>Stundenplan</button>
                    <button class="sidebar-element" id="lernbueros" onClick={() => {setActivePage("lernbueros"); highlightButton("lernbueros")}}>Lernbüros</button>
                    <button class="sidebar-element" id="fehler" onClick={() => {setActivePage("fehler"); highlightButton("fehler")}}>Fehler</button>
                    <button class="sidebar-element" id="kontakt" onClick={() => {setActivePage("kontakt"); highlightButton("kontakt")}}>Kontakt</button>
                </div>
                <Plan activePage={activePage}></Plan>
            </div>
        </div>
    )
}