/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import { SubjectColor } from "../../api/main";
import { fetchJSessionId } from "../../api/main";
import { registerAccount } from "../../api/main";
import { getTimetable } from "../../api/main";
import Popup from "./Popup";
import type { JSX } from "preact";
import { testStudent } from "../../logs/testStudent";
import "../../styles/Stundenplan.scss";
import { useState, useEffect } from "preact/hooks";

export default function Stundenplan(): JSX.Element {
    useEffect(() => {
        fetchJSessionId("account", "password").then((sessionId) => {
            if(sessionId.result) {
                document.cookie = `JSESSIONID=${sessionId.result}; max-age=600; secure; samesite=strict`
            }
            else {
                alert(sessionId.status)
            }
        })
    }, [])
    const getJSessionIdCookie = () => {
        const storedJSessionId = document.cookie.match('(^|;)\\s*' + "JSESSIONID" + '\\s*=\\s*([^;]+)')?.pop() || ''
        if(storedJSessionId) {
            console.log(storedJSessionId)
            return storedJSessionId
        }
        else {
            fetchJSessionId("", "").then((sessionId) => {
                if(sessionId.result) {
                    document.cookie = `JSESSIONID=${sessionId.result}; max-age=600; secure; samesite=strict`
                    return sessionId.result
                }
                else {
                    alert(sessionId.status)
                    return false
                }
            })
        }
    }
    const tableElements: Array<Array<JSX.Element>> = [[],[],[],[],[]];
    const [popupStatus, setPopupStatus] = useState<boolean>(false);
    const [popupContent, setPopupContent] = useState<JSX.Element>()
    const openPopup = () => {
        registerAccount("name", "1Passwort!")
        getTimetable();
        setPopupStatus(true)
    }
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
                        let roomStyle = {
                            textDecoration: "none",
                        }
                        let teacherStyle = {
                            textDecoration: "none",
                        }
                        let substitutionRoomStyle = {
                            display: "none"
                        }
                        let substitutionTeacherStyle = {
                            display: "none"
                        }
                        flexStyle = {
                            gridRowStart: lessons[k].starts.toString(),
                            gridRowEnd: "span " + lessons[k].length
                        }
                        if(!lessons[k].substitution) {
                            lessonElements.push(
                               <div style={objectStyle} onClick={() => {
                                openPopup()
                                setPopupContent(
                                <div style={objectStyle}>
                                    <p>{lessons[k].room}</p>
                                    <h2>{lessons[k].subjectShort}</h2>
                                    <p>{lessons[k].teacher}</p>
                                </div>)
                            }}>
                                <p>{lessons[k].room}</p>
                                <h2>{lessons[k].subjectShort}</h2>
                                <p>{lessons[k].teacher}</p>
                            </div> 
                            )
                        }
                        else {
                            if(lessons[k].substitution?.room) {
                                roomStyle = { textDecoration: "line-through" }
                                substitutionRoomStyle = { display: "block" }
                            }
                            if(lessons[k].substitution?.teacher) {
                                teacherStyle = { textDecoration: "line-through" }
                                substitutionTeacherStyle = { display: "block" }
                            }
                            if(lessons[k].substitution?.cancelled) {
                                roomStyle = { textDecoration: "line-through" }
                                teacherStyle = { textDecoration: "line-through" }
                            }
                            lessonElements.push(
                                <div style={objectStyle} onClick={() => {
                                    openPopup()
                                    setPopupContent(
                                    <div style={objectStyle}>
                                        <p style={roomStyle}>{lessons[k].room}</p>
                                        <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                                        <h2>{lessons[k].subjectShort}</h2>
                                        <p style={teacherStyle}>{lessons[k].teacher}</p>
                                        <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                                    </div>)
                             }}>
                                    <p style={roomStyle}>{lessons[k].room}</p>
                                    <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                                    <h2>{lessons[k].subjectShort}</h2>
                                    <p style={teacherStyle}>{lessons[k].teacher}</p>
                                    <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                                </div> 
                             )
                        }
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

    const tableDays: Array<JSX.Element> = [];
    for(let i: number = 0; i < 5; i++) {
        tableDays.push(
            <div className="table-day">
                {tableElements[i]}
            </div>
        )
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
                    <Popup trigger={popupStatus} setPopupStatus={setPopupStatus} content={popupContent}></Popup>
                    {tableDays}
                </div>
            </div>
            
        </div>
    );
};