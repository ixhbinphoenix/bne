import { getLocalUntisCredentials } from "./untisAPI";
import type { TheScheduleObject } from "./main";

export function verifyPassword(password: string): boolean {
    const regex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$/;

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
        credentials: "include",
        body: JSON.stringify({
            username: username,
            password: password
        })
    })
    if(!result.body) {
        return {
            status: 400,
            message: "No result body found"
        }
    }
    let body: ReadableStream<Uint8Array> = await result.body
    let stream = await readStream(body);
    let cleanBody = JSON.parse(stream)
    if(cleanBody.success) {
        return  {
            status: 200,
            cypher: cleanBody.body.untis_cypher
        }
    }
    else {
        return {
            status: 403,
            message: cleanBody.body.message
        }
    }
}
export async function registerAccount(username: string, hashedPassword: string, personId: number, untisCredentialsEncrypted: string) {
    let result = await fetch('https://localhost:8080/register', {
        method: 'POST',
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({
            username: username,
            password: hashedPassword,
            person_id: personId,
            untis_cypher: untisCredentialsEncrypted
        })
    })
    if(!result.body) {
            console.error("No result body found!")
            return {
                status: 400,
                message: "No result body found"
            }
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
    let resultRaw = await fetch('https://localhost:8080/demo/get_timetable', {
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
export async function verifySession() {
    if(getLocalUntisCredentials()) {
        const status = await checkSessionId()
        if(status == 200) {
            return true
        }
        else {
            return false
        } 
        }
    else {
        return false
    }
}