import { fetchJSessionId } from "../api/untisAPI";
import { getTimetableServiceWorker } from "../api/theBackend";
import type { TheScheduleObject } from "../api/main";

self.addEventListener("message", async (message) => {
  requestTimetable(
    message.data,[
      {
        teacher: "SMER",
        is_lb: false,
        start: 1,
        length: 2,
        day: 3,
        subject: "MU G1",
        subject_short: "MU",
        room: "O E-02MU",
        substitution: null
      },
      {
        teacher: "PPOW",
        is_lb: false,
        start: 3,
        length: 2,
        day: 3,
        subject: "SW L1",
        subject_short: "SW",
        room: "O 2-02",
        substitution: {
          teacher: null,
          room: "O 1-19NW",
          substitution_text: null,
          cancelled: false
        }
      },
      {
        teacher: "MVCR",
        is_lb: false,
        start: 5,
        length: 2,
        day: 3,
        subject: "SP G2",
        subject_short: "SP",
        room: "O TH2",
        substitution: {
          teacher: "---",
          room: null,
          substitution_text: "Vtr. ohne Lehrer",
          cancelled: false
        }
      },
      {
        teacher: "FSMI",
        is_lb: false,
        start: 9,
        length: 2,
        day: 3,
        subject: "ER G1",
        subject_short: "ER",
        room: "O 2-01",
        substitution: null
      }
    ])
});

async function requestTimetable(untisData: { username: string; password: string }, oldLessons?: TheScheduleObject[]) {
  let today = new Date().toISOString().slice(0, 10).replace(/-/g, "");
  today = "20240620";
  const JSessionId = (await fetchJSessionId(untisData.username, untisData.password)).JSessionId;
  let lessons = await getTimetableServiceWorker(today, today, JSessionId);
  if (oldLessons) {
    const changedLessons = compareLessons(oldLessons, lessons);
    console.log(changedLessons)
    handleChanges(changedLessons)
  }
  if (calculateTimeout() > 15 * 60 * 1000) {
    lessons = []
  }
  console.log(calculateTimeout())
  setTimeout(() => requestTimetable(untisData, lessons), calculateTimeout());
}
function sendNotification(title: string, options: NotificationOptions) {
  console.log(title, options)
  if (Notification.permission === "granted") {
    self.registration.showNotification(title, options);
  } else {
    console.log("denied");
  }
}
function compareLessons(oldLessons: TheScheduleObject[], newLessons: TheScheduleObject[]): TheScheduleObject[] {
  // Use .filter to find the lessons that have changed
  const changedLessons = newLessons.filter((newLesson, index) => {
    const oldLesson = oldLessons[index];
    const oldSubstitution = oldLesson.substitution;
    const newSubstitution = newLesson.substitution;
    if (oldSubstitution && newSubstitution) {
      // Check if the substitution's cancelled status or teacher has changed
      console.log(oldSubstitution, newSubstitution)
      return (
        oldSubstitution.cancelled !== newSubstitution.cancelled || oldSubstitution.teacher !== newSubstitution.teacher || oldSubstitution.room !== newSubstitution.room
      );
    }
    else if (!oldSubstitution && newSubstitution) {
      return true
    }
    return false;
  });
  return changedLessons;
}
function calculateTimeout(): number {
  const now = new Date();
  const currentHour = now.getHours();
  const startHour = 6;
  const endHour = 14; // 2 PM is 14:00 in 24-hour format

  if (currentHour >= startHour && currentHour <= endHour) {
    // If the current time is between 6:00 AM and 2:00 PM, return 15 minutes in milliseconds
    return 15 * 60 * 1000;
  } else {
    // Calculate the time remaining until the next 6:00 AM
    let nextSixAM = new Date(now);
    nextSixAM.setHours(startHour, 0, 0, 0);

    if (now > nextSixAM) {
      // If the current time is after 6:00 AM today, set the next 6:00 AM to tomorrow
      nextSixAM.setDate(nextSixAM.getDate() + 1);
    }

    const timeUntilNextSixAM = nextSixAM.getTime() - now.getTime();
    return timeUntilNextSixAM;
  }
}
function handleChanges(changedLessons: TheScheduleObject[]) {
  changedLessons.forEach((lesson) => {
    if (lesson.substitution?.teacher == "---" || lesson.substitution?.cancelled) {
      if (lesson.length == 2) {
        sendNotification(`${lesson.start}. - ${lesson.start + 1}. Stunde entfällt`, {
          body: `${lesson.subject_short} bei ${lesson.teacher}`
        });
      } else {
        sendNotification(`${lesson.start}. Stunde entfällt`, {
          body: `${lesson.subject_short} bei ${lesson.teacher}`
        });
      }
    }
    else {
      if (lesson.length == 2) {
        sendNotification(`Änderungen in ${lesson.start}. - ${lesson.start + 1}. Stunde`, {
          body: `${lesson.subject_short} bei ${lesson.teacher}`
        });
      } else {
        sendNotification(`Änderungen in ${lesson.start}. Stunde`, {
          body: `${lesson.subject_short} bei ${lesson.teacher}`
        });
      }
    }
  });
}
