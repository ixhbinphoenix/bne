export async function fetchJSessionId(username: string, password: string): Promise<{ status: string, result: string | null }> {
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
        result: resultClean.result.sessionId};
   } catch {
    return {
        status: '401 Unauthorized\nFalsche Logindaten',
        result: null}
   }
};
export interface TheScheduleObject {
    teacher: string;
    lernbuero: boolean;
    starts: number;
    length: number;
    day: number;
    subject: string;
    subjectShort: string;
    room: string;
    substitution: {
        teacher: string | null,
        room: string | null,
        subsitutionMessage: string | null,
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
    N0: "#ff1717",
    S0: "#ff1717",
    SW: "#212193",
    SP: "#4091e4",
    PA: "#ff5500",
    EK: "#00490d"
}