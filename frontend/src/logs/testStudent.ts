import type { TheScheduleObject } from "../api/main"

export const testStudent: Array<TheScheduleObject>  = [
    {
        teacher: "FSMI",
        lernbuero: true,
        starts: 3,
        length: 1,
        day: 0,
        subject: "Religion",
        room: "O 1-13",
        subjectShort: "ER",
        substitution: null
    },
    {
        teacher: "USTR",
        lernbuero: true,
        starts: 3,
        length: 1,
        day: 0,
        subject: "Geschichte",
        room: "O 2-07",
        subjectShort: "GE",
        substitution: null
    },
    {
        teacher: "TKIN",
        lernbuero: false,
        starts: 3,
        length: 1,
        day: 0,
        subject: "Mathematik",
        room: "O 2-11",
        subjectShort: "M",
        substitution: null
    },
    {
        teacher: "USTR",
        lernbuero: false,
        starts: 3,
        length: 1,
        day: 0,
        subject: "Geschichte",
        room: "O 1-01",
        subjectShort: "GE",
        substitution: null
    },
    {
        teacher: "FSPR",
        lernbuero: true,
        starts: 3,
        length: 1,
        day: 0,
        subject: "Musik",
        room: "O E-02",
        subjectShort: "MU",
        substitution: null
    },
    {
        teacher: "PPOW",
        lernbuero: false,
        starts: 3,
        length: 2,
        day: 1,
        subject: "Informatik",
        room: "O 2-16",
        subjectShort: "IF",
        substitution: {
            teacher: null,
            room: null,
            subsitutionMessage: "Vtr. ohne Lehrer",
            cancelled: true
        }
    },
    {
        teacher: "FSMI",
        lernbuero: false,
        starts: 5,
        length: 2,
        day: 1,
        subject: "Religion",
        room: "O 2-01",
        subjectShort: "ER",
        substitution: null
    },
    {
        teacher: "RKAR",
        lernbuero: false,
        starts: 1,
        length: 2,
        day: 2,
        subject: "Englisch",
        room: "O 2-11",
        subjectShort: "E",
        substitution: null
    },
    {
        teacher: "TKIN",
        lernbuero: false,
        starts: 3,
        length: 2,
        day: 2,
        subject: "Physik",
        room: "H PH",
        subjectShort: "PH",
        substitution: null
    },
    {
        teacher: "MVEL",
        lernbuero: true,
        starts: 5,
        length: 1,
        day: 2,
        subject: "Chemie",
        room: "O 1-17",
        subjectShort: "CH",
        substitution: null
    },
    {
        teacher: "PPOW",
        lernbuero: true,
        starts: 6,
        length: 1,
        day: 2,
        subject: "Informatik",
        room: "O 2-16",
        subjectShort: "IF",
        substitution: null
    },
    {
        teacher: "JMUL",
        lernbuero: true,
        starts: 8,
        length: 1,
        day: 2,
        subject: "Latein",
        room: "O 1-02",
        subjectShort: "L8",
        substitution: null
    },
    {
        teacher: "RSCH",
        lernbuero: false,
        starts: 9,
        length: 2,
        day: 2,
        subject: "Deutsch",
        room: "O 2-07",
        subjectShort: "D",
        substitution: null
    },
    {
        teacher: "RSCH",
        lernbuero: true,
        starts: 1,
        length: 1,
        day: 3,
        subject: "Deutsch",
        room: "O 2-11",
        subjectShort: "D",
        substitution: null
    },
    {
        teacher: "TKIN",
        lernbuero: true,
        starts: 2,
        length: 1,
        day: 3,
        subject: "Mathematik",
        room: "O 2-11",
        subjectShort: "M",
        substitution: null
    },
    {
        teacher: "JMUL",
        lernbuero: false,
        starts: 3,
        length: 2,
        day: 3,
        subject: "Latein",
        room: "---",
        subjectShort: "L8",
        substitution: null
    },
    {
        teacher: "HKÜS",
        lernbuero: false,
        starts: 5,
        length: 2,
        day: 3,
        subject: "Sozialwissenschaft",
        room: "O 1-01",
        subjectShort: "SW",
        substitution: null
    },
    {
        teacher: "TWAR",
        lernbuero: false,
        starts: 8,
        length: 2,
        day: 3,
        subject: "Sport",
        room: "O TH2",
        subjectShort: "SP",
        substitution: null
    },
    {
        teacher: "HKUS",
        lernbuero: true,
        starts: 10,
        length: 1,
        day: 3,
        subject: "Sozialwissenschaft",
        room: "O E-01",
        subjectShort: "SW",
        substitution: null
    },
    {
        teacher: "TKIN",
        lernbuero: true,
        starts: 4,
        length: 1,
        day: 4,
        subject: "Physik",
        room: "H PH",
        subjectShort: "PH",
        substitution: null
    },
    {
        teacher: "FSPR",
        lernbuero: false,
        starts: 5,
        length: 2,
        day: 4,
        subject: "Musik",
        room: "O E-02",
        subjectShort: "MU",
        substitution: null
    },
    {
        teacher: "RKAR",
        lernbuero: true,
        starts: 8,
        length: 1,
        day: 4,
        subject: "Englisch",
        room: "O 2-11",
        subjectShort: "E",
        substitution: null
    },
    {
        teacher: "MVEL",
        lernbuero: false,
        starts: 9,
        length: 2,
        day: 4,
        subject: "Chemie",
        room: "O 1-17",
        subjectShort: "CH",
        substitution: null
    },
]