export interface TheScheduleObject {
    teacher: string;
    lernbuero: boolean;
    starts: number;
    length: number;
    day: number;
    subject: string;
    subjectShort: string;
    room: string;
};
export const SubjectColor: { [key: string]: string} =  {
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
    N0: "#ff1717",
    S0: "#ff1717",
    SW: "#212193",
    SP: "#4091e4",
    PA: "#ff5500",
    EK: "#00490d"
}