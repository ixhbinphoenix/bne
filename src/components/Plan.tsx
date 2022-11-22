/* @jsxImportSource preact */

import type { JSX } from "preact";
import "../styles/Plan.css"
import { useState } from "preact/hooks";

export default function Plan(): JSX.Element {
    const tableDivs = [];

    for(let index = 0; index < 50; index++) {
        let tableIndex: string = "table-index-" + index;
        tableDivs.push(<div id={tableIndex}><p>Test</p></div>)
    }

    return(
        <div className="table-layout">
            <div className="table">
                {tableDivs}
            </div>
        </div>
    );
};
