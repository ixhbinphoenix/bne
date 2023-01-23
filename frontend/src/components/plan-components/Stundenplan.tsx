/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import type { JSX } from "preact";
import { testStudent } from "../../logs/testStudent";
import { SubjectColor } from "../../api/main";
import "../../styles/Stundenplan.scss";

export default function Stundenplan(): JSX.Element {
    const tableElements: Array<JSX.Element> = [];
    
    const addToDivs = (lesson: TheScheduleObject) => {
        const objectStyle = {
            backgroundColor: SubjectColor[lesson.subjectShort],
            gridColumnStart: lesson.day,
            gridRow: lesson.starts + " / span " + lesson.length
        }
        const parentFlexboxStyle = {
            backgroundColor: "transparent",
            height: "100%",
            display: "flex",
            flexFlow: "row",
            gridRow: lesson.starts + " / span " + lesson.length
        }

        let match = false;

        for(let i: number = 0; i < testStudent.length; i++) {
            if(lesson.day == testStudent[i].day && lesson.starts == testStudent[i].starts && lesson.subject != testStudent[i].subject) {
                if(!testStudent[i].matched) {
                    testStudent[testStudent.indexOf(lesson)].matched = true;
                    tableElements.push(
                        <div style={parentFlexboxStyle}>
                            <div style={objectStyle}>
                                <p>{lesson.room}</p>
                                <h2>{lesson.subject}</h2>
                                <p>{lesson.teacher}</p>
                            </div>
                            <div style={{backgroundColor: SubjectColor[testStudent[i].subjectShort]}}>
                                <p>{testStudent[i].room}</p>
                                <h2>{testStudent[i].subject}</h2>
                                <p>{testStudent[i].teacher}</p>
                            </div>
                        </div>
                    );  
                }
                match = true;
            }
        }
        if(!match) {
            tableElements.push(
                <div style={objectStyle}>
                    <p>{lesson.room}</p>
                    <h2>{lesson.subject}</h2>
                    <p>{lesson.teacher}</p>
                </div>
            );
        }
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
                    {tableElements}
                </div>
            </div>
            
        </div>
    );
};