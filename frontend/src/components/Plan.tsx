/* @jsxImportSource preact */

import type { TheScheduleObject } from "../api/main";
import type { JSX } from "preact";
import "../styles/Plan.scss"

export default function Plan(): JSX.Element {
    
    const testStudent: Array<TheScheduleObject>  = [
        {
            teacher: "ABCDE",
            lernbuero: false,
            starts: 1,
            length: 1,
            day: 1,
            subject: "Sozialwissenschaft",
            room: "OG0-00"
        },
    ]
    
    const tableDivs: Array<JSX.Element> = [];
    
    const addToDivs = (lesson: TheScheduleObject) => {
        const objectStyle = {
            gridColumnStart: lesson.day,
            gridRow: lesson.starts + " / span " + lesson.length
        }

        tableDivs.push(
            <div style={objectStyle}>
                <p>{lesson.room}</p>
                <h2>{lesson.subject}</h2>
                <p>{lesson.teacher}</p>
            </div>
        )
    }

    for(let i: number = 0; i < testStudent.length; i++) {
        addToDivs(testStudent[i]);
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
