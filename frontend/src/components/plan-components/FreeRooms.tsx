/* @jsxImportSource preact */

import { JSESSIONIDCookieString, type FreeRoom } from "../../api/main";
import { getFreeRooms } from "../../api/theBackend";
import Loading from "../Loading";
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
import { fetchJSessionId, getLocalUntisCredentials } from "../../api/untisAPI";

type Floor = "E" | "1" | "2";

export default function FreeRooms(): JSX.Element {
  const [currentWeek, setCurrentWeek] = useState(getMondayAndFridayDates());

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
    getFreeRooms(currentWeek.currentMonday, currentWeek.currentFriday).then(
      (rooms) => {
        addToDivs(rooms);
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
              getFreeRooms(currentWeek.currentMonday, currentWeek.currentFriday).then(
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

  const nextWeek = () => {
    closePopup();
    let week = shiftForward(currentWeek.currentMonday, currentWeek.currentFriday);
    highlightDates(week.currentMonday, week.currentFriday);
    setCurrentDates(getWeekDays(week.currentMonday));

    getFreeRooms(week.currentMonday, week.currentFriday).then(
      (rooms: FreeRoom[]) => {
        addToDivs(rooms);
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

    getFreeRooms(week.currentMonday, week.currentFriday).then(
      (rooms: FreeRoom[]) => {
        addToDivs(rooms);
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

  const addToDivs = (rooms: FreeRoom[]) => {
    tableElements = [[], [], [], [], []];
    const colors = {
      E: "#4c4c4c",
      1: "#43c95b",
      2: "rgb(0 181 246)"
    };
    for (let i: number = 0; i < 5; i++) {
      for (let j: number = 0; j < 10; j++) {
        let lessonElements: Array<JSX.Element> = [];

        let flexStyle = {
          gridRowStart: "1",
          gridRowEnd: "span 1",
          flexDirection: "row"
        };

        for (let k: number = 0; k < rooms.length; k++) {
          if (rooms[k].day == i && rooms[k].start - 1 == j) {
            const floor: Floor = rooms[k].room[2] as Floor;
            const objectStyle = {
              backgroundColor: colors[floor],
              cursor: "pointer",
              opacity: 1
            };
            flexStyle = {
              gridRowStart: rooms[k].start.toString(),
              gridRowEnd: "span " + rooms[k].length,
              flexDirection: "row"
            };
            lessonElements.push(
              <div
                class="lesson"
                style={objectStyle}
                onClick={() => {
                  openPopup();
                  setPopupContent(
                    <div style={objectStyle}>
                      <p>{floor}</p>
                      <h2>{rooms[k].room}</h2>
                      <p>&nbsp;</p>
                    </div>
                  );
                }}>
                <p>{floor}</p>
                <h2>{rooms[k].room}</h2>
                <p>&nbsp;</p>
              </div>
            );
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
  const [tableDays, setTableDays] = useState<Array<JSX.Element>>([<Loading />]);
  return (
    <div className="table-layout">
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
