/* @jsxImportSource preact */

import type { JSX } from "preact";
import "../styles/Plan.css"
import { useState } from "preact/hooks";

export default function Plan(): JSX.Element {
    const [content, setContent] = useState([""])

    const createContent = []

    for(let i = 0; i < 50; i++) {
        let newContent = "Test" + i;
        createContent.push(newContent);
    }

    setContent(createContent);
    
    const tableDivs = [];

    for(let index = 0; index < 50; index++) {
        let tableIndex: string = "table-index-" + index;
        tableDivs.push(<div id={tableIndex}><p>{content[index]}</p></div>)
        //console.log(content[index]);
    }

    return(
        <div className="table-layout">
            <div className="table">
                {tableDivs}
            </div>
        </div>
    );
};
