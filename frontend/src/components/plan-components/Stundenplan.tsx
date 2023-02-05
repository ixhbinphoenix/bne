/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import type { JSX } from "preact";
import { testStudent } from "../../logs/testStudent";
import { SubjectColor } from "../../api/main";
import "../../styles/Stundenplan.scss";

export default function Stundenplan(): JSX.Element {
    const tableElements: Array<Array<JSX.Element>> = [[],[],[],[],[]];
    
    const addToDivs = (lessons: Array<TheScheduleObject>) => {
        for(let i: number = 0; i < 5; i++) {
            for(let j: number = 0; j < 10; j++) {
                let lessonElements: Array<JSX.Element> = [];

                let flexStyle = {
                    gridRowStart: "1",
                    gridRowEnd: "span 1"
                }

                for(let k: number = 0; k < lessons.length; k++) {
                    if(lessons[k].day == i && lessons[k].starts - 1 == j) {
                        const objectStyle = {
                            backgroundColor: SubjectColor[lessons[k].subjectShort]
                        }
                        flexStyle = {
                            gridRowStart: lessons[k].starts.toString(),
                            gridRowEnd: "span " + lessons[k].length
                        }
                        lessonElements.push(
                            <div style={objectStyle}>
                                <p>{lessons[k].room}</p>
                                <h2>{lessons[k].subjectShort}</h2>
                                <p>{lessons[k].teacher}</p>
                            </div>
                        )
                    }
                }

                if(lessonElements.length) {
                    tableElements[i].push(
                    <div className="parent-flex" style={flexStyle}>
                        {lessonElements}
                    </div>
                )
                }
            }
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
                    <span>
                        <div>07:55</div>
                        1
                        <div>08:40</div>
                    </span>
                    <span>
                        <div>
                            08:40
                        </div>
                        2
                        <div>
                            09:25
                        </div>
                    </span>
                    <span>
                        <div>
                            09:45
                        </div>
                        3
                        <div>
                            10:30
                        </div>
                    </span>
                    <span>
                        <div>
                            10:30
                        </div>
                        4
                        <div>
                            11:15
                        </div>
                    </span>
                    <span>
                        <div>
                            11:35
                        </div>
                        5
                        <div>
                            12:20
                        </div>
                    </span>
                    <span>
                        <div>
                            12:20
                        </div>
                        6
                        <div>
                            13:05
                        </div>
                    </span>
                    <span>
                        <div>
                            13:15
                        </div>
                        7
                        <div>
                            14:00
                        </div>
                    </span>
                    <span>
                        <div>
                            14:05
                        </div>
                        8
                        <div>
                            14:50
                        </div>
                    </span>
                    <span>
                        <div>
                            14:50
                        </div>    
                        9
                        <div>
                            15:35
                        </div>
                    </span>
                    <span>
                        <div>
                            15:40
                        </div>
                        10
                        <div>
                            16:25
                        </div>
                    </span>
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