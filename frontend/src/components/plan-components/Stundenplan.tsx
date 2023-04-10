/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import { SubjectColor } from "../../api/main";
import { fetchJSessionId } from "../../api/untisAPI";
import { getTimetable, verifySession } from "../../api/theBackend";
import Popup from "./Popup";
import type { JSX } from "preact"; 
import "../../styles/Stundenplan.scss";
import { useState, useEffect } from "preact/hooks";
import { getMondayAndFridayDates, shiftForward, shiftBackward, getWeekDays } from "../../api/dateHandling";

export default function Stundenplan(): JSX.Element {

    const [currentWeek, setCurrentWeek] = useState(getMondayAndFridayDates())

    useEffect(() => {
        verifySession().then((status) => {
            if(!status) {
                window.location.href = "/login" //bye bye go back to lobby
            }
        })
        fetchJSessionId(localStorage.getItem("untis_username"), localStorage.getItem("untis_password")).then((result) => {
            if(result.JSessionId && result.personId) {
                document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`
            }
            else { 
                alert(result.status)
            }
        })
        setCurrentDates(getWeekDays(currentWeek.currentMonday))
        getTimetable(currentWeek.currentMonday, currentWeek.currentFriday).then(result => {
            if(result.lessons) {
                addToDivs(result.lessons)
                const tableDaysTemp = []; 
                for(let i: number = 0; i < 5; i++) {
                    tableDaysTemp.push(
                        <div className="table-day">
                            {tableElements[i]}
                        </div>
                    )
                }
                setTableDays(tableDaysTemp)
            }
        })

    }, [])
   
    const nextWeek = () => {
        let week = shiftForward(currentWeek.currentMonday, currentWeek.currentFriday);
        setCurrentDates(getWeekDays(week.currentMonday))

        getTimetable(week.currentMonday, week.currentFriday).then(result => {
            if(result.lessons) {
                addToDivs(result.lessons)
                const tableDaysTemp = []; 
                for(let i: number = 0; i < 5; i++) {
                    tableDaysTemp.push(
                        <div className="table-day">
                            {tableElements[i]}
                        </div>
                    )
                }
                setTableDays(tableDaysTemp)
            }
        })
        setCurrentWeek(shiftForward(currentWeek.currentMonday, currentWeek.currentFriday))
    }
    const previousWeek = () => {
        let week = shiftBackward(currentWeek.currentMonday, currentWeek.currentFriday);
        setCurrentDates(getWeekDays(week.currentMonday))
        getTimetable(week.currentMonday, week.currentFriday).then(result => {
            if(result.lessons) {
                addToDivs(result.lessons)
                const tableDaysTemp = []; 
                for(let i: number = 0; i < 5; i++) {
                    tableDaysTemp.push(
                        <div className="table-day">
                            {tableElements[i]}
                        </div>
                    )
                }
                setTableDays(tableDaysTemp)
            }
        })
        setCurrentWeek(shiftBackward(currentWeek.currentMonday, currentWeek.currentFriday))
    }
    const goToCurrentWeek = () => {
        setCurrentWeek(getMondayAndFridayDates())

        getTimetable(currentWeek.currentMonday, currentWeek.currentFriday).then(result => {
            if(result.lessons) {
                console.log("setting data")
                console.log(result.lessons)
                addToDivs(result.lessons)
                const tableDaysTemp = []; 
                for(let i: number = 0; i < 5; i++) {
                    tableDaysTemp.push(
                        <div className="table-day">
                            {tableElements[i]}
                        </div>
                    )
                }
                setTableDays(tableDaysTemp)
                console.log(tableDaysTemp)
            }
        })
    }
    const getJSessionIdCookie = () => {
        const storedJSessionId = document.cookie.match('(^|;)\\s*' + "JSESSIONID" + '\\s*=\\s*([^;]+)')?.pop() || ''
        if(storedJSessionId) {
            console.log(storedJSessionId)
            return storedJSessionId
        }
        else {
            fetchJSessionId("", "").then((result) => {
                if(result.JSessionId) {
                    document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=strict`
                    return result.JSessionId
                }
                else {
                    alert(result.status)
                    return false
                }
            })
        }
    }
    let tableElements: Array<Array<JSX.Element>> = [[],[],[],[],[]];
    const [popupStatus, setPopupStatus] = useState<boolean>(false);
    const [popupContent, setPopupContent] = useState<JSX.Element>()
    const [currentDates, setCurrentDates] = useState<Array<string>>(getWeekDays(currentWeek.currentMonday));
    const openPopup = () => {
        setPopupStatus(true)
    }
    const addToDivs = (lessons: TheScheduleObject[]) => {
        tableElements = [[],[],[],[],[]]      
        for(let i: number = 0; i < 5; i++) {
            for(let j: number = 0; j < 10; j++) {
                let lessonElements: Array<JSX.Element> = [];

                let flexStyle = {
                    gridRowStart: "1",
                    gridRowEnd: "span 1"
                }

                for(let k: number = 0; k < lessons.length; k++) {
                    if(lessons[k].day == i && lessons[k].start - 1 == j) {
                        let subjectType = lessons[k].subject;
                        if(lessons[k].subject_short != "") {
                            subjectType = lessons[k].subject_short
                        }
                        const objectStyle = {
                            backgroundColor: SubjectColor[lessons[k].subject_short]
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
                            gridRowStart: lessons[k].start.toString(),
                            gridRowEnd: "span " + lessons[k].length
                        }
                        if(!lessons[k].substitution) {
                            lessonElements.push(
                               <div style={objectStyle} onClick={() => {
                                openPopup()
                                setPopupContent(
                                <div style={objectStyle}>
                                    <p>{lessons[k].room}</p>
                                    <h2>{subjectType}</h2>
                                    <p>{lessons[k].teacher}</p>
                                </div>)
                            }}>
                                <p>{lessons[k].room}</p>
                                <h2>{subjectType}</h2>
                                <p>{lessons[k].teacher}</p>
                            </div> 
                            )
                        }
                        else {
                            if(lessons[k].substitution?.room && lessons[k].substitution?.room != "---") {
                                roomStyle = { textDecoration: "line-through" }
                                substitutionRoomStyle = { display: "block" }
                            }
                            if(lessons[k].substitution?.teacher) {
                                teacherStyle = { textDecoration: "line-through" }
                                if(lessons[k].substitution?.teacher) {
                                    substitutionTeacherStyle = { display: "block" }
                                }
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
                                        <h2>{subjectType}</h2>
                                        <p style={teacherStyle}>{lessons[k].teacher}</p>
                                        <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                                    </div>)
                                    }}>
                                    <p style={roomStyle}>{lessons[k].room}</p>
                                    <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                                    <h2>{subjectType}</h2>
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
    const [tableDays, setTableDays] = useState<Array<JSX.Element>>([]);
    return(
        <div className="table-layout">
            <div className="table-top">
                <span className="day">{currentDates[0]}<br/>Mo.</span>
                <span className="day">{currentDates[1]}<br/>Di.</span>
                <span className="day">{currentDates[2]}<br/>Mi.</span>
                <span className="day">{currentDates[3]}<br/>Do.</span>
                <span className="day">{currentDates[4]}<br/>Fr.</span>
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
                    <div className="bar-left bar" onClick={previousWeek}>❰</div>
                    <div className="bar-right bar" onClick={nextWeek}>❱</div>
                    <Popup trigger={popupStatus} setPopupStatus={setPopupStatus} content={popupContent}></Popup>
                    {tableDays}
                </div>
            </div>
        </div>
    );
};