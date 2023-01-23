/* @jsxImportSource preact */

interface IProps {
    activePage: string
}

import type { TheScheduleObject } from "../api/main";
import type { JSX } from "preact";
import { testStudent } from "../logs/testStudent";
import { SubjectColor } from "../api/main";
import "../styles/Plan.scss";

export default function Plan(props: IProps): JSX.Element {
    
    switch(props.activePage) {
        case "stundenplan":

    }

    const tableElements: {[key: string]: Array<JSX.Element>} = {
        stundenplan: [],
        lernbueros: [],
    }

    const addToDivs = (lesson: TheScheduleObject) => {
        const objectStyle = {
            backgroundColor: SubjectColor[lesson.subjectShort],
            gridColumnStart: lesson.day,
            gridRow: lesson.starts + " / span " + lesson.length
        }

        tableElements.stundenplan.push(
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
                    {tableElements[props.activePage]}
                </div>
            </div>
            
        </div>
    );
};
