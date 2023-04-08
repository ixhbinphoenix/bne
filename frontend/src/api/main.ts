type APIReturnValue = Promise<{ status: string, result: string | null }>;

export async function fetchJSessionId(username: string | null, password: string | null): Promise<{ status: string, JSessionId: string | null, personId: number | null}> {
    let resultRaw = await fetch('https://borys.webuntis.com/WebUntis/jsonrpc.do?school=ges-münster', {
        method: 'POST',
        body: JSON.stringify({
            id: 'theSchedule',
            method: 'authenticate',
            params: {user: username, password: password, client: "theSchedule"},
            jsonrpc: '2.0'
        })
    })
   let resultClean = await resultRaw.json()
    try {
    return {
        status: '200 Ok',
        JSessionId: resultClean.result.sessionId,
        personId: resultClean.result.personId
    };
   } catch {
    return {
        status: '401 Unauthorized\nFalsche Logindaten',
        JSessionId: null,
        personId: null
    }
   }
};
export function saveUntisCredentials(username: string, password: string) {
    localStorage.setItem("untis_username", username);
    localStorage.setItem("untis_password", password);
}
export function getLocalUntisCredentials() {
    const username = localStorage.getItem("untis_username")
    const password = localStorage.getItem("untis_password")
    if(username && password) {
        return {username: username, password: password}
    }
    else {
        return null
    }
}
export function verifyPassword(password: string): boolean {
    const regex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$%^&*()_+])(?=.*[^\da-zA-Z]).{8,}$/;
    
    if (regex.test(password)) {
      return true;
    } else {
      return false
    }
}
export function verifyEmail(email: string): boolean {
    const regex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

    if(regex.test(email)) {
        return true;
    } else {
        return false;
    }

}
export async function loginAccount(username: string, password: string) {
    let result = await fetch("https://localhost:8080/login", {
        method: 'POST',
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username: username,
            password: password
        })
    })
}
export async function registerAccount(username: string, password: string, personId: number) {
    let result = await fetch('https://localhost:8080/register', {
        method: 'POST',
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({
            username: username,
            password: password,
            person_id: personId
        })
    })
    if(!result.body) {
            console.error("No result body found!")
            return "No result body found!"
    }
    let body: ReadableStream<Uint8Array> = await result.body
    let stream = await readStream(body);
    let requestResult = stream.split("\n")
    return {
        status: requestResult[0],
        message: requestResult[1]
    }
}
async function readStream(stream: ReadableStream<Uint8Array>) {
    const textDecode = new TextDecoder()
    const chunks = [];
    const reader = stream.getReader()
    while(true) {
        const { done, value } = await reader.read()
        if(done) {
            break;
        }
        chunks.push(textDecode.decode(value))
    }
    return chunks.join("")
}
export async function getTimetable(): Promise<{lessons?: TheScheduleObject[], status: string, message?: string}> {
    let resultRaw = await fetch('https://localhost:8080/get_timetable', {
        method: 'GET',
        credentials: "include"
    })
    let resultClean = await resultRaw.json()
    try {
        if(resultClean.body.lessons) {
            return {
                lessons: resultClean.body.lessons,
                status: "200 OK",
                message: undefined
            }
        }
        return {
            lessons: undefined,
            status: resultClean.body.code,
            message: resultClean.body.message
        }
    }
    catch {
        return {
            status: "400",
            message: "Bad Request"
        }
    }
}
async function checkSessionId() {
    let result = await fetch('https://localhost:8080/check_session', {
        method: "GET",
        credentials: "include"
    })
    return result.status
}
export function verifySession() {
    if(getLocalUntisCredentials()) {
        checkSessionId().then((status) => {
            if(status == 200) {
                return true
            }
            else {
                return false
            }
        })
    }
    else {
        console.log("nope")
        return false
    }
}
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
        teacher: string | null,
        room: string | null,
        subsitution_text: string | null,
        cancelled: boolean
    } | null
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
    N0: "#ff4d17",
    S0: "#ff4d17",
    SW: "#212193",
    SP: "#4091e4",
    PA: "#ff5500",
    EK: "#00490d"
}