import type { TheScheduleObject } from "../api/main"

export const testStudent: Array<TheScheduleObject>  = [
    {
        teacher: "FSMI",
        is_lb: true,
        start: 3,
        length: 1,
        day: 0,
        subject: "Religion",
        room: "O 1-13",
        subject_short: "ER",
        substitution: null
    },
    {
        teacher: "USTR",
        is_lb: true,
        start: 3,
        length: 1,
        day: 0,
        subject: "Geschichte",
        room: "O 2-07",
        subject_short: "GE",
        substitution: null
    },
    {
        teacher: "TKIN",
        is_lb: false,
        start: 3,
        length: 1,
        day: 0,
        subject: "Mathematik",
        room: "O 2-11",
        subject_short: "M",
        substitution: null
    },
    {
        teacher: "USTR",
        is_lb: false,
        start: 3,
        length: 1,
        day: 0,
        subject: "Geschichte",
        room: "O 1-01",
        subject_short: "GE",
        substitution: null
    },
    {
        teacher: "FSPR",
        is_lb: true,
        start: 3,
        length: 1,
        day: 0,
        subject: "Musik",
        room: "O E-02",
        subject_short: "MU",
        substitution: null
    },
    {
        teacher: "PPOW",
        is_lb: false,
        start: 3,
        length: 2,
        day: 1,
        subject: "Informatik",
        room: "O 2-16",
        subject_short: "IF",
        substitution: {
            teacher: null,
            room: null,
            subsitution_text: "Vtr. ohne Lehrer",
            cancelled: true
        }
    },
    {
        teacher: "FSMI",
        is_lb: false,
        start: 5,
        length: 2,
        day: 1,
        subject: "Religion",
        room: "O 2-01",
        subject_short: "ER",
        substitution: null
    },
    {
        teacher: "RKAR",
        is_lb: false,
        start: 1,
        length: 2,
        day: 2,
        subject: "Englisch",
        room: "O 2-11",
        subject_short: "E",
        substitution: null
    },
    {
        teacher: "TKIN",
        is_lb: false,
        start: 3,
        length: 2,
        day: 2,
        subject: "Physik",
        room: "H PH",
        subject_short: "PH",
        substitution: null
    },
    {
        teacher: "MVEL",
        is_lb: true,
        start: 5,
        length: 1,
        day: 2,
        subject: "Chemie",
        room: "O 1-17",
        subject_short: "CH",
        substitution: null
    },
    {
        teacher: "PPOW",
        is_lb: true,
        start: 6,
        length: 1,
        day: 2,
        subject: "Informatik",
        room: "O 2-16",
        subject_short: "IF",
        substitution: null
    },
    {
        teacher: "JMUL",
        is_lb: true,
        start: 8,
        length: 1,
        day: 2,
        subject: "Latein",
        room: "O 1-02",
        subject_short: "L8",
        substitution: null
    },
    {
        teacher: "RSCH",
        is_lb: false,
        start: 9,
        length: 2,
        day: 2,
        subject: "Deutsch",
        room: "O 2-07",
        subject_short: "D",
        substitution: null
    },
    {
        teacher: "RSCH",
        is_lb: true,
        start: 1,
        length: 1,
        day: 3,
        subject: "Deutsch",
        room: "O 2-11",
        subject_short: "D",
        substitution: null
    },
    {
        teacher: "TKIN",
        is_lb: true,
        start: 2,
        length: 1,
        day: 3,
        subject: "Mathematik",
        room: "O 2-11",
        subject_short: "M",
        substitution: null
    },
    {
        teacher: "JMUL",
        is_lb: false,
        start: 3,
        length: 2,
        day: 3,
        subject: "Latein",
        room: "---",
        subject_short: "L8",
        substitution: null
    },
    {
        teacher: "HKÜS",
        is_lb: false,
        start: 5,
        length: 2,
        day: 3,
        subject: "Sozialwissenschaft",
        room: "O 1-01",
        subject_short: "SW",
        substitution: null
    },
    {
        teacher: "TWAR",
        is_lb: false,
        start: 8,
        length: 2,
        day: 3,
        subject: "Sport",
        room: "O TH2",
        subject_short: "SP",
        substitution: null
    },
    {
        teacher: "HKUS",
        is_lb: true,
        start: 10,
        length: 1,
        day: 3,
        subject: "Sozialwissenschaft",
        room: "O E-01",
        subject_short: "SW",
        substitution: null
    },
    {
        teacher: "TKIN",
        is_lb: true,
        start: 4,
        length: 1,
        day: 4,
        subject: "Physik",
        room: "H PH",
        subject_short: "PH",
        substitution: null
    },
    {
        teacher: "FSPR",
        is_lb: false,
        start: 5,
        length: 2,
        day: 4,
        subject: "Musik",
        room: "O E-02",
        subject_short: "MU",
        substitution: null
    },
    {
        teacher: "RKAR",
        is_lb: true,
        start: 8,
        length: 1,
        day: 4,
        subject: "Englisch",
        room: "O 2-11",
        subject_short: "E",
        substitution: null
    },
    {
        teacher: "MVEL",
        is_lb: false,
        start: 9,
        length: 2,
        day: 4,
        subject: "Chemie",
        room: "O 1-17",
        subject_short: "CH",
        substitution: null
    },
]