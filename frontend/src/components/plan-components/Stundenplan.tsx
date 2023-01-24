/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import type { JSX } from "preact";
import { testStudent } from "../../logs/testStudent";
import { SubjectColor } from "../../api/main";
import "../../styles/Stundenplan.scss";
import { createDefaultDevConfig } from "astro/dist/core/config";

export default function Stundenplan(): JSX.Element {
    const tableElements: Array<Array<JSX.Element>> = [[],[],[],[],[]];
    
    const getGridColumns = (currentLesson: TheScheduleObject, lessons: Array<TheScheduleObject>) => {
        for(let i: number = 0; i < lessons.length; i++) {
            if(currentLesson.day == lessons[i].day && currentLesson.starts == lessons[i].starts && currentLesson.subject != lessons[i].subject) {
                return null;
            }
        }
        return "1 / -4";
    }

    const addToDivs = (lessons: Array<TheScheduleObject>) => {
        for(let i: number = 0; i < lessons.length; i++) {
            const objectStyle = {
                backgroundColor: SubjectColor[lessons[i].subjectShort],
                gridRow: lessons[i].starts + "/ span " + lessons[i].length,
                width: "100%",
                gridColumn: getGridColumns(lessons[i], lessons)
            }
            tableElements[lessons[i].day].push(
                <div style={objectStyle}>
                    <p>{lessons[i].room}</p>
                    <h2>{lessons[i].subject}</h2>
                    <p>{lessons[i].teacher}</p>
                </div>
            )
        }
    }
    addToDivs(testStudent);

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
                    <div className="table-day">
                        {tableElements[0]}
                    </div>
                    <div className="table-day">
                        {tableElements[1]}
                    </div>
                    <div className="table-day">
                        {tableElements[2]}
                    </div>
                    <div className="table-day">
                        {tableElements[3]}
                    </div>
                    <div className="table-day">
                        {tableElements[4]}
                    </div>
                </div>
            </div>
            
        </div>
    );
};