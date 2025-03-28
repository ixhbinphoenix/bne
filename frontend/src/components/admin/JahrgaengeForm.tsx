import { useEffect, useState } from "preact/hooks";
import { Jahrgang } from "../../api/main";
import { JSX } from "preact";
import { get } from "http";
import { getJahrgaenge, sendJahrgaenge } from "../../api/theBackend";
import React from "preact/compat";


export default function JahrgaengeForm(): JSX.Element {
    const [jahrgaenge, setJahrgaenge] = useState<Jahrgang[]>([]);
    useEffect(() => {
        getJahrgaenge().then((res) => {
            setJahrgaenge(res.jahrgaenge.sort((a, b) => a.name.localeCompare(b.name)));
        });
    }, []);
    const saveJahrgaenge = () => {
        const updatedJahrgaenge: Jahrgang[] = [];
        const allElems = document.getElementById("jahrgaenge-table")!;
        Array.from(allElems.children).forEach((row) => {
            const cells = Array.from(row.children);
            const jahrgangName = (cells[0] as HTMLElement).innerText; // Name des Jahrgangs
            const checkbox = cells[1].querySelector("input[type='checkbox']") as HTMLInputElement;
    
            if (checkbox) {
                updatedJahrgaenge.push({
                    name: jahrgangName,
                    active: checkbox.checked, // Speichert den Status der Checkbox
                });
            }
        });
    
        sendJahrgaenge(updatedJahrgaenge).catch(() => {
            alert("Das Admin Passwort ist falsch oder Sie sind nicht angemeldet");
        });
    };
    return (
        <>
            <table class="table m-0 bg-dark" data-bs-theme="dark">
                <thead class="bg-dark">
                    <tr class="border-bottom">
                        <th class="col-5" scope="col">
                            Jahrgang
                        </th>
                        <th class="col-5" scope="col">
                            Aktiv
                        </th>
                        <button class="btn btn-primary active my-2" onClick={saveJahrgaenge}>
                            <i class="bi bi-floppy"></i>&nbsp;Alle Speichern
                        </button>
                    </tr>
                </thead>
                <tbody id="jahrgaenge-table">
                    {jahrgaenge.map((jahrgang) => (
                        <tr key={jahrgang.name}>
                            <td>{jahrgang.name}</td>
                            <td><input
                                type="checkbox"
                                checked={jahrgang.active}
                            /></td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </>
    )
}