/* @jsxImportSource preact */

import type { TheScheduleObject } from "../../api/main";
import { SubjectColor, SubjectNames, JSESSIONIDCookieString } from "../../api/main";
import { fetchJSessionId, getLocalUntisCredentials } from "../../api/untisAPI";
import { getLernbueros } from "../../api/theBackend";
import Popup from "./Popup";
import type { JSX } from "preact";
import "../../styles/Stundenplan.scss";
import { useState, useEffect } from "preact/hooks";
import {
  getMondayAndFridayDates,
  shiftForward,
  shiftBackward,
  getWeekDays,
  getCurrentDay,
  getCurrentLesson
} from "../../api/dateHandling";
import { onSwipe } from "../../api/Touch";
import Loading from "../Loading";

var lb_saves: TheScheduleObject[];

export default function Lernbueros(): JSX.Element {
  const [currentWeek, setCurrentWeek] = useState(getMondayAndFridayDates());

  if (sessionStorage.getItem("monday") && currentWeek.currentMonday != sessionStorage.getItem("monday")) {
    setCurrentWeek(getMondayAndFridayDates(sessionStorage.getItem("monday")!));
  } else {
    sessionStorage.setItem("monday", currentWeek.currentMonday);
  }

  const highlightDates = (currentMonday: string, currentFriday: string) => {
    const days = document.getElementsByClassName("day");
    Array.from(days).forEach((day) => {
      day.classList.remove("highlighted");
    });
    const currentDay = document.getElementById("day" + getCurrentDay(currentMonday, currentFriday));
    currentDay?.classList.add("highlighted");

    const lessons = document.getElementsByClassName("lesson-number");
    Array.from(lessons).forEach((lesson) => {
      lesson.classList.remove("highlighted");
    });
    const currentLesson = document.getElementById("lesson" + getCurrentLesson(currentMonday, currentFriday));
    currentLesson?.classList.add("highlighted");
  };
  useEffect(() => {
    highlightDates(currentWeek.currentMonday, currentWeek.currentFriday);

    setCurrentDates(getWeekDays(currentWeek.currentMonday));
    getLernbueros(currentWeek.currentMonday, currentWeek.currentFriday).then(
      (lessons) => {
        lb_saves = lessons;
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
              getLernbueros(currentWeek.currentMonday, currentWeek.currentFriday).then(
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

  const rerender = () => {
    if (lb_saves) {
      addToDivs(lb_saves);
      const tableDaysTemp = [];
      for (let i: number = 0; i < 5; i++) {
        tableDaysTemp.push(<div className="table-day">{tableElements[i]}</div>);
      }
      setTableDays(tableDaysTemp);
    } else {
      openPopup();
    }
  };

  useEffect(() => {
    onSwipe(".table-layout", { direction: "left" }, nextWeek);
    onSwipe(".table-layout", { direction: "right" }, previousWeek);
  }, [currentWeek]);

  const nextWeek = () => {
    closePopup();
    let week = shiftForward(currentWeek.currentMonday, currentWeek.currentFriday);
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getLernbueros(week.currentMonday, week.currentFriday).then(
      (lessons) => {
        lb_saves = lessons;
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
    sessionStorage.setItem("monday", week.currentMonday);
    setCurrentWeek(week);
  };
  const previousWeek = () => {
    closePopup();
    let week = shiftBackward(currentWeek.currentMonday, currentWeek.currentFriday);
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getLernbueros(week.currentMonday, week.currentFriday).then(
      (lessons) => {
        lb_saves = lessons;
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
    sessionStorage.setItem("monday", week.currentMonday);
    setCurrentWeek(week);
  };
  const goToCurrentWeek = () => {
    closePopup();
    let week = getMondayAndFridayDates();
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getLernbueros(week.currentMonday, week.currentFriday).then(
      (lessons) => {
        lb_saves = lessons;
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
  const [filterStatus, setFilterStatus] = useState(false);
  const [FilterContent, setFilterContent] = useState<JSX.Element | null>(null);
  const checkAll = () => {
    const inputs = document.getElementsByTagName("input");
    Array.from(inputs).forEach((input) => {
      input.checked = true;
    });
  };
  const uncheckAll = () => {
    const inputs = document.getElementsByTagName("input");
    Array.from(inputs).forEach((input) => {
      input.checked = false;
    });
  };
  let filterItems;
  const openFilter = () => {
    setFilterStatus(true);
    if (!localStorage.getItem("filterItems")) {
      filterItems = {
        M: true,
        D: true,
        E: true,
        CH: true,
        GE: true,
        ER: true,
        KR: true,
        PL: true,
        IF: true,
        MU: true,
        PH: true,
        BI: true,
        L8: true,
        N0: true,
        S0: true,
        SW: true,
        PA: true,
        EK: true,
        LI: true
      };
      localStorage.setItem("filterItems", JSON.stringify(filterItems));
      Filter(true, filterItems);
    } else {
      filterItems = JSON.parse(localStorage.getItem("filterItems")!);
      Filter(true, filterItems);
    }
  };
  const closeFilter = () => {
    setFilterStatus(false);
    Filter(false);
    rerender();
  };
  const changeFilter = (filterItems: any) => {
    localStorage.setItem("filterItems", JSON.stringify(filterItems));
  };

  const Filter = (filterStatus: boolean, filterItems?: any) => {
    if (filterStatus) {
      const FilterItems = [];
      for (const item in SubjectNames) {
        if (item != "SP") {
          FilterItems.push(
            <label htmlFor={item}>
              {SubjectNames[item]}
              <input
                type="checkbox"
                id={item}
                defaultChecked={filterItems[item]}
                onClick={() => {
                  filterItems[item] = !filterItems[item];
                  changeFilter(filterItems);
                }}
              />
              <span className="checkbox"></span>
            </label>
          );
        }
      }
      setFilterContent(
        <div class="filter-background">
          <div class="filter-content">
            <form>{FilterItems}</form>
          </div>
        </div>
      );
    } else {
      setFilterContent(null);
    }
  };
  const closePopup = () => {
    setPopupStatus(false);
  };
  const addToDivs = (lessons: TheScheduleObject[]) => {
    tableElements = [[], [], [], [], []];
    for (let i: number = 0; i < 5; i++) {
      for (let j: number = 0; j < 10; j++) {
        let lessonElements: Array<JSX.Element> = [];

        let flexStyle = {
          gridRowStart: "1",
          gridRowEnd: "span 1",
          flexDirection: "row"
        };

        let filter = localStorage.getItem("filterItems");

        for (let k: number = 0; k < lessons.length; k++) {
          if (
            lessons[k].day == i &&
            lessons[k].start - 1 == j &&
            (!filter || JSON.parse(filter)[lessons[k].subject_short])
          ) {
            let subjectType = lessons[k].subject;
            if (lessons[k].subject_short != "") {
              subjectType = lessons[k].subject_short;
            }
            const objectStyle = {
              backgroundColor: SubjectColor[lessons[k].subject_short],
              cursor: "pointer",
              opacity: 1
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
            flexStyle = {
              gridRowStart: lessons[k].start.toString(),
              gridRowEnd: "span " + lessons[k].length,
              flexDirection: "row"
            };
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
                  <p style={roomStyle}>{lessons[k].room}</p>
                  <h2>{subjectType}</h2>
                  <p style={teacherStyle}>{lessons[k].teacher}</p>
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
                teacherStyle = { textDecoration: "line-through" };
                objectStyle.opacity = 0.5;
              }
              if (lessons[k].substitution?.cancelled) {
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
                  <h2>{subjectType}</h2>
                  <p style={teacherStyle}>{lessons[k].teacher}</p>
                </div>
              );
            }
          }
        }
        if (lessonElements.length) {
          if (lessonElements.length < 5) {
            tableElements[i].push(
              <div className="parent-flex" style={flexStyle}>
                {lessonElements}
              </div>
            );
          } else {
            flexStyle.flexDirection = "column";
            lessonElements.forEach((lesson) => {
              lesson.props.children.shift();
              lesson.props.children.pop();
            });
            const rows = Math.ceil(lessonElements.length / 4);
            const subFlexes = [];
            let j = 0;
            for (let i = 0; i < rows; i++) {
              subFlexes.push(<div class="sub-flex">{lessonElements.slice(j, j + 4)}</div>);
              j += 4;
            }
            tableElements[i].push(
              <div className="parent-flex" style={flexStyle}>
                {subFlexes}
              </div>
            );
          }
        }
      }
    }
  };
  const [tableDays, setTableDays] = useState<Array<JSX.Element>>([<Loading />]);
  return (
    <div className="table-layout">
      <img
        style="cursor: pointer;"
        id="filter-icon"
        src="/filter.svg"
        alt="filter icon"
        onClick={() => {
          if (filterStatus) {
            closeFilter();
          } else {
            openFilter();
          }
        }}
      />
      <div className="table-top">
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
      <div className="table-body">
        <div className="table-sidebar-left">
          <span id="lesson1" className="lesson-number">
            <div>07:55</div>1<div>08:40</div>
          </span>
          <span id="lesson2" className="lesson-number">
            <div>08:40</div>2<div>09:25</div>
          </span>
          <span id="lesson3" className="lesson-number">
            <div>09:45</div>3<div>10:30</div>
          </span>
          <span id="lesson4" className="lesson-number">
            <div>10:30</div>4<div>11:15</div>
          </span>
          <span id="lesson5" className="lesson-number">
            <div>11:35</div>5<div>12:20</div>
          </span>
          <span id="lesson6" className="lesson-number">
            <div>12:20</div>6<div>13:05</div>
          </span>
          <span id="lesson7" className="lesson-number">
            <div>13:15</div>7<div>14:00</div>
          </span>
          <span id="lesson8" className="lesson-number">
            <div>14:05</div>8<div>14:50</div>
          </span>
          <span id="lesson9" className="lesson-number">
            <div>14:50</div>9<div>15:35</div>
          </span>
          <span id="lesson10" className="lesson-number">
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
          {FilterContent}
          <Popup trigger={popupStatus} setPopupStatus={setPopupStatus} content={popupContent}></Popup>
          {tableDays}
        </div>
      </div>
    </div>
  );
}
