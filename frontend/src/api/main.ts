export interface TheScheduleObject {
  teacher: string;
  is_lb: boolean;
  start: number;
  length: number;
  day: number;
  subject: string;
  subject_short: string;
  room: string;
  substitution: {
    teacher: string | null;
    room: string | null;
    substitution_text: string | null;
    cancelled: boolean;
  } | null;
}
export const SubjectColor: { [key: string]: string } = {
  M: "#dba402",
  D: "#ff1717",
  E: "#4040e4",
  CH: "#3b07c5",
  GE: "#003540",
  ER: "#a32de4",
  KR: "#a32de4",
  PL: "#a32de4",
  IF: "#e46a2d",
  MU: "#4c4c4c",
  KU: "#4c4c4c",
  PH: "#43c95b",
  BI: "#00d226",
  L8: "#f87406",
  N0: "#ff4d17",
  S0: "#ff4d17",
  SW: "#212193",
  SP: "#4091e4",
  PA: "#ff5500",
  EK: "#00490d"
};
export const SubjectNames: { [key: string]: string } = {
  M: "Mathematik",
  D: "Deutsch",
  E: "Englisch",
  CH: "Chemie",
  GE: "Geschichte",
  ER: "Evangelische Religionslehre",
  KR: "Katholische Religionslehre",
  PL: "Philosophie",
  IF: "Informatik",
  MU: "Musik",
  PH: "Physik",
  BI: "Biologie",
  L8: "Latein",
  N0: "Niederländisch",
  S0: "Spanisch",
  SW: "Sozialwissenschaft",
  SP: "Sport",
  PA: "Pädagogik",
  EK: "Erdkunde",
  LI: "Literatur"
};
