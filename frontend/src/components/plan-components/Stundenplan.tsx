/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import { JSESSIONIDCookieString, SubjectColor, SubjectNames } from "../../api/main";
import { fetchJSessionId, getLocalUntisCredentials } from "../../api/untisAPI";
import { getTimetable } from "../../api/theBackend";
import Popup from "./Popup";
import type { JSX } from "preact";
import "../../styles/Stundenplan.scss";
import { useState, useEffect, useRef } from "preact/hooks";
import {
  getMondayAndFridayDates,
  shiftForward,
  shiftBackward,
  getWeekDays,
  getCurrentDay,
  getCurrentLesson
} from "../../api/dateHandling";
import { onSwipe } from "../../api/Touch";
import type { ChangeEvent } from "preact/compat";
import Loading from "../Loading";

export default function Stundenplan(): JSX.Element {
  const [currentWeek, setCurrentWeek] = useState(getMondayAndFridayDates());
  const [classes, setClasses] = useState<JSX.Element[]>([]);
  const [activeClass, setActiveClass] = useState<string | undefined>();
  const classRef = useRef(activeClass);
  classRef.current = activeClass;

  const highlightDates = (currentMonday: string, currentFriday: string) => {
    const days = document.getElementsByClassName("day");
    Array.from(days).forEach((day) => {
      day.classList.remove("highlighted");
    });
    const currentDay = document.getElementById("day" + getCurrentDay(currentMonday, currentFriday));
    currentDay?.classList.add("highlighted");
    const lessons = document.getElementsByClassName("lesson");
    Array.from(lessons).forEach((lesson) => {
      lesson.classList.remove("highlighted");
    });
    const currentLesson = document.getElementById("lesson" + getCurrentLesson(currentMonday, currentFriday));
    currentLesson?.classList.add("highlighted");
  };
  useEffect(() => {
    highlightDates(getMondayAndFridayDates().currentMonday, getMondayAndFridayDates().currentFriday);

    setCurrentDates(getWeekDays(currentWeek.currentMonday));
    addClasses();
    getTimetable(currentWeek.currentMonday, currentWeek.currentFriday).then(
      (lessons) => {
        addToDivs(lessons);
        const tableDaysTemp = [];
        for (let i: number = 0; i < 5; i++) {
          tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
        }
        setTableDays(tableDaysTemp);
      },
      (error) => {
        if (error.message == "Fetching from Untis failed") {
          fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
            if (result.JSessionId) {
              closePopup();
              document.cookie = JSESSIONIDCookieString(result.JSessionId);
              getTimetable(currentWeek.currentMonday, currentWeek.currentFriday).then(
                (lessons) => {
                  addToDivs(lessons);
                  const tableDaysTemp = [];
                  for (let i: number = 0; i < 5; i++) {
                    tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
                  }
                  setTableDays(tableDaysTemp);
                },
                (error) => {
                  console.error(error);
                  setPopupContent(
                    <div>
                      <h1 style="text-align: center;">{error.message}</h1>
                    </div>
                  );
                  openPopup();
                }
              );
            }
          });
        } else {
          console.error(error);
          setPopupContent(
            <div>
              <h1 style="text-align: center;">{error.message}</h1>
            </div>
          );
          openPopup();
        }
      }
    );
  }, []);

  useEffect(() => {
    onSwipe(".table-layout", { direction: "left" }, nextWeek);
    onSwipe(".table-layout", { direction: "right" }, previousWeek);
  }, [currentWeek]);
  useEffect(() => {
    getTimetable(currentWeek.currentMonday, currentWeek.currentFriday, classRef.current).then(
      (lessons) => {
        addToDivs(lessons);
        const tableDaysTemp = [];
        for (let i: number = 0; i < 5; i++) {
          tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
        }
        setTableDays(tableDaysTemp);
      },
      (error) => {
        console.error(error);
        setPopupContent(
          <div>
            <h1 style="text-align: center;">{error.message}</h1>
          </div>
        );
        openPopup();
      }
    );
  }, [activeClass]);
  const addClasses = () => {
    const classNames = ["Mein Stundenplan", "EF", "Q1", "Q2", "LB_OS"];
    for (let i = 5; i < 11; i++) {
      classNames.push(i + "a");
      classNames.push(i + "b");
      classNames.push(i + "c");
      classNames.push(i + "d");
    }
    const items = classNames.map((className) => {
      return (
        <option key={className} value={className}>
          {className}
        </option>
      );
    });
    setClasses(items);
  };
  const changeClass = (event: ChangeEvent) => {
    const className = (event!.target as HTMLOptionElement)!.value!;
    className != "Mein Stundenplan" ? setActiveClass(className) : setActiveClass(undefined);
  };
  const nextWeek = () => {
    closePopup();
    let week = shiftForward(currentWeek.currentMonday, currentWeek.currentFriday);
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getTimetable(week.currentMonday, week.currentFriday, classRef.current).then(
      (lessons) => {
        addToDivs(lessons);
        const tableDaysTemp = [];
        for (let i: number = 0; i < 5; i++) {
          tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
        }
        setTableDays(tableDaysTemp);
      },
      (error) => {
        if (error.message == "Fetching from Untis failed") {
          fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
            if (result.JSessionId) {
              document.cookie = JSESSIONIDCookieString(result.JSessionId);
              nextWeek();
            }
          });
        } else {
          console.error(error);
          setPopupContent(
            <div>
              <h1 style="text-align: center;">{error.message}</h1>
            </div>
          );
          openPopup();
        }
      }
    );
    setCurrentWeek(week);
  };
  const previousWeek = () => {
    closePopup();
    let week = shiftBackward(currentWeek.currentMonday, currentWeek.currentFriday);
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getTimetable(week.currentMonday, week.currentFriday, classRef.current).then(
      (lessons) => {
        addToDivs(lessons);
        const tableDaysTemp = [];
        for (let i: number = 0; i < 5; i++) {
          tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
        }
        setTableDays(tableDaysTemp);
      },
      (error) => {
        if (error.message == "Fetching from Untis failed") {
          fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
            if (result.JSessionId) {
              document.cookie = JSESSIONIDCookieString(result.JSessionId);
              previousWeek();
            }
          });
        } else {
          console.error(error);
          setPopupContent(
            <div>
              <h1 style="text-align: center;">{error.message}</h1>
            </div>
          );
          openPopup();
        }
      }
    );
    setCurrentWeek(week);
  };
  const goToCurrentWeek = () => {
    closePopup();
    let week = getMondayAndFridayDates();
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getTimetable(week.currentMonday, week.currentFriday).then(
      (lessons) => {
        addToDivs(lessons);
        const tableDaysTemp = [];
        for (let i: number = 0; i < 5; i++) {
          tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
        }
        setTableDays(tableDaysTemp);
      },
      (error) => {
        console.error(error);
        setPopupContent(
          <div>
            <h1 style="text-align: center;">{error.message}</h1>
          </div>
        );
        openPopup();
      }
    );
  };
  const getJSessionIdCookie = () => {
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    if (storedJSessionId) {
      return storedJSessionId;
    } else {
      fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
        if (result.JSessionId) {
          document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=strict`;
          return result.JSessionId;
        } else {
          alert(result.status);
          return false;
        }
      });
    }
  };
  let tableElements: Array<Array<JSX.Element>> = [[], [], [], [], []];
  const [popupStatus, setPopupStatus] = useState<boolean>(false);
  const [popupContent, setPopupContent] = useState<JSX.Element>();
  const [currentDates, setCurrentDates] = useState<Array<string>>(getWeekDays(currentWeek.currentMonday));
  const openPopup = () => {
    setPopupStatus(true);
  };
  const closePopup = () => {
    setPopupStatus(false);
  };
  const addToDivs = (lessons: (TheScheduleObject & { skip?: boolean })[]) => {
    tableElements = [[], [], [], [], []];
    for (let i: number = 0; i < 5; i++) {
      for (let j: number = 0; j < 10; j++) {
        let lessonElements: Array<JSX.Element> = [];
        let gridStyle;
        let hasDoubleLesson = false;
        for (let k: number = 0; k < lessons.length; k++) {
          if (
            hasDoubleLesson &&
            !lessons[k].skip &&
            lessons[k].day == i &&
            lessons[k].start - 2 == j &&
            (lessons[k].start == 2 || lessons[k].start == 4 || lessons[k].start == 6)
          ) {
            let subjectType = lessons[k].subject;
            if (lessons[k].subject_short != "") {
              subjectType = lessons[k].subject_short;
            }
            let objectStyle = {
              backgroundColor: SubjectColor[lessons[k].subject_short],
              opacity: 1,
              gridRow: "2 / span 1",
              borderBottom: "none"
            };
            let roomStyle = {
              textDecoration: "none"
            };
            let teacherStyle = {
              textDecoration: "none"
            };
            let substitutionRoomStyle = {
              display: "none"
            };
            let substitutionTeacherStyle = {
              display: "none"
            };
            let substitutionTextStyle = {
              display: "none"
            };
            if (!gridStyle) {
              gridStyle = {
                gridRowStart: lessons[k].start.toString(),
                gridRowEnd: "span " + lessons[k].length
              };
              objectStyle = {
                backgroundColor: SubjectColor[lessons[k].subject_short],
                opacity: 1,
                gridRow: "inhert",
                borderBottom: "inhert"
              };
            }
            if (!lessons[k].substitution) {
              lessonElements.push(
                <div
                  class="lesson"
                  style={objectStyle}
                  onClick={() => {
                    openPopup();
                    setPopupContent(
                      <div style={objectStyle}>
                        <p>{lessons[k].room}</p>
                        <h2>{subjectType}</h2>
                        <p>{lessons[k].teacher}</p>
                      </div>
                    );
                  }}>
                  <p>{lessons[k].room}</p>
                  <h2>{subjectType}</h2>
                  <p>{lessons[k].teacher}</p>
                </div>
              );
            } else {
              if (lessons[k].substitution?.room && lessons[k].substitution?.room != "---") {
                roomStyle = { textDecoration: "line-through" };
                substitutionRoomStyle = { display: "block" };
              }
              if (lessons[k].substitution?.room == "---") {
                roomStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.teacher && lessons[k].substitution?.teacher != "---") {
                teacherStyle = { textDecoration: "line-through" };
                substitutionTeacherStyle = { display: "block" };
              }
              if (lessons[k].substitution?.teacher == "---") {
                objectStyle.opacity = 0.5;
                teacherStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.cancelled) {
                objectStyle.opacity = 0.5;
                roomStyle = { textDecoration: "line-through" };
                teacherStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.substitution_text) {
                substitutionTextStyle = { display: "block" };
              }
              lessonElements.push(
                <div
                  class="lesson"
                  style={objectStyle}
                  onClick={() => {
                    openPopup();
                    setPopupContent(
                      <div style={objectStyle}>
                        <p style={roomStyle}>{lessons[k].room}</p>
                        <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                        <h2>{subjectType}</h2>
                        <strong style={substitutionTextStyle}>{lessons[k].substitution?.substitution_text}</strong>
                        <p style={teacherStyle}>{lessons[k].teacher}</p>
                        <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                      </div>
                    );
                  }}>
                  <p style={roomStyle}>{lessons[k].room}</p>
                  <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                  <h2>{subjectType}</h2>
                  <p style={teacherStyle}>{lessons[k].teacher}</p>
                  <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                </div>
              );
            }
            lessons[k].skip = true;
          } else if (lessons[k].day == i && lessons[k].start - 1 == j && !lessons[k].skip) {
            let subjectType = lessons[k].subject;
            if (lessons[k].length == 2) hasDoubleLesson = true;
            if (lessons[k].subject_short != "") {
              subjectType = lessons[k].subject_short;
            }
            const objectStyle = {
              backgroundColor: SubjectColor[lessons[k].subject_short],
              opacity: 1,
              gridRow: "1 / span " + lessons[k].length
            };
            let roomStyle = {
              textDecoration: "none"
            };
            let teacherStyle = {
              textDecoration: "none"
            };
            let substitutionRoomStyle = {
              display: "none"
            };
            let substitutionTeacherStyle = {
              display: "none"
            };
            let substitutionTextStyle = {
              display: "none"
            };
            if (gridStyle?.gridRowEnd != "span 3" && lessons[k].length == 3) {
              gridStyle = {
                gridRowStart: lessons[k].start.toString(),
                gridRowEnd: "span " + lessons[k].length,
                gridTemplateRows: `repeat(${lessons[k].length}, 1fr)`
              };
            }
            if (gridStyle?.gridRowEnd != "span 2") {
              gridStyle = {
                gridRowStart: lessons[k].start.toString(),
                gridRowEnd: "span " + lessons[k].length,
                gridTemplateRows: `repeat(${lessons[k].length}, 1fr)`
              };
            }
            if (!lessons[k].substitution) {
              lessonElements.push(
                <div
                  class="lesson"
                  style={objectStyle}
                  onClick={() => {
                    openPopup();
                    setPopupContent(
                      <div style={objectStyle}>
                        <p>{lessons[k].room}</p>
                        <h2>{subjectType}</h2>
                        <p>{lessons[k].teacher}</p>
                      </div>
                    );
                  }}>
                  <p>{lessons[k].room}</p>
                  <h2>{subjectType}</h2>
                  <p>{lessons[k].teacher}</p>
                </div>
              );
            } else {
              if (lessons[k].substitution?.room && lessons[k].substitution?.room != "---") {
                roomStyle = { textDecoration: "line-through" };
                substitutionRoomStyle = { display: "block" };
              }
              if (lessons[k].substitution?.room == "---") {
                roomStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.teacher && lessons[k].substitution?.teacher != "---") {
                teacherStyle = { textDecoration: "line-through" };
                substitutionTeacherStyle = { display: "block" };
              }
              if (lessons[k].substitution?.teacher == "---") {
                objectStyle.opacity = 0.5;
                teacherStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.cancelled) {
                objectStyle.opacity = 0.5;
                roomStyle = { textDecoration: "line-through" };
                teacherStyle = { textDecoration: "line-through" };
              }
              if (lessons[k].substitution?.substitution_text) {
                substitutionTextStyle = { display: "block" };
              }
              lessonElements.push(
                <div
                  class="lesson"
                  style={objectStyle}
                  onClick={() => {
                    openPopup();
                    setPopupContent(
                      <div style={objectStyle}>
                        <p style={roomStyle}>{lessons[k].room}</p>
                        <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                        <h2>{subjectType}</h2>
                        <strong style={substitutionTextStyle}>{lessons[k].substitution?.substitution_text}</strong>
                        <p style={teacherStyle}>{lessons[k].teacher}</p>
                        <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                      </div>
                    );
                  }}>
                  <p style={roomStyle}>{lessons[k].room}</p>
                  <p style={substitutionRoomStyle}>{lessons[k].substitution?.room}</p>
                  <h2>{subjectType}</h2>
                  <p style={teacherStyle}>{lessons[k].teacher}</p>
                  <p style={substitutionTeacherStyle}>{lessons[k].substitution?.teacher}</p>
                </div>
              );
            }
          }
        }
        if (lessonElements.length) {
          tableElements[i].push(
            <div className="parent-grid" style={gridStyle}>
              {lessonElements}
            </div>
          );
        }
      }
    }
  };
  const [tableDays, setTableDays] = useState<Array<JSX.Element>>([<Loading />]);

  return (
    <div className="table-layout">
      <div className="table-top" style="flex-direction: column; --spacing-top: clamp(45px, 7%, 10vh);">
        <div className="select-class">
          <form>
            <select id="classes" onChange={changeClass}>
              <label htmlFor="classes">Klasse auswählen</label>
              {classes}
            </select>
          </form>
        </div>
        <div className="dates" style="display: flex">
          <span id="day1" class="day">
            {currentDates[0]}
            <br />
            Mo.
          </span>
          <span id="day2" class="day">
            {currentDates[1]}
            <br />
            Di.
          </span>
          <span id="day3" class="day">
            {currentDates[2]}
            <br />
            Mi.
          </span>
          <span id="day4" class="day">
            {currentDates[3]}
            <br />
            Do.
          </span>
          <span id="day5" class="day">
            {currentDates[4]}
            <br />
            Fr.
          </span>
        </div>
      </div>
      <div className="table-body" style="--spacing-top: clamp(45px, 7%, 10vh);">
        <div className="table-sidebar-left">
          <span class="lesson" id="lesson1">
            <div>07:55</div>1<div>08:40</div>
          </span>
          <span class="lesson" id="lesson2">
            <div>08:40</div>2<div>09:25</div>
          </span>
          <span class="lesson" id="lesson3">
            <div>09:45</div>3<div>10:30</div>
          </span>
          <span class="lesson" id="lesson4">
            <div>10:30</div>4<div>11:15</div>
          </span>
          <span class="lesson" id="lesson5">
            <div>11:35</div>5<div>12:20</div>
          </span>
          <span class="lesson" id="lesson6">
            <div>12:20</div>6<div>13:05</div>
          </span>
          <span class="lesson" id="lesson7">
            <div>13:15</div>7<div>14:00</div>
          </span>
          <span class="lesson" id="lesson8">
            <div>14:05</div>8<div>14:50</div>
          </span>
          <span class="lesson" id="lesson9">
            <div>14:50</div>9<div>15:35</div>
          </span>
          <span class="lesson" id="lesson10">
            <div>15:40</div>
            10
            <div>16:25</div>
          </span>
        </div>
        <div className="table">
          <div className="bar-left bar" onClick={previousWeek}>
            ❰
          </div>
          <div className="bar-right bar" onClick={nextWeek}>
            ❱
          </div>
          <Popup trigger={popupStatus} setPopupStatus={setPopupStatus} content={popupContent} />
          {tableDays}
        </div>
      </div>
    </div>
  );
}
