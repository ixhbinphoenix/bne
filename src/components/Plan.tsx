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
    }

    return(
        <div className="table-layout">
            <div className="table-top">
                <span className="day">Montag</span>
                <span className="day">Dienstag</span>
                <span className="day">Mittwoch</span>
                <span className="day">Donnerstag</span>
                <span className="day">Freitag</span>
            </div>
            <div className="table-body">
                <div className="table-sidebar-left">
                    <span>1</span>
                    <span>2</span>
                    <span>3</span>
                    <span>4</span>
                    <span>5</span>
                    <span>6</span>
                    <span>7</span>
                    <span>8</span>
                    <span>9</span>
                    <span>10</span>
                </div>
                <div className="table">
                    {tableDivs}
                </div>
            </div>
            
        </div>
    );
};
