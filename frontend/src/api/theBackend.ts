import { getLocalUntisCredentials, fetchJSessionId } from "./untisAPI";
import type { TheScheduleObject } from "./main";

export function verifyPassword(password: string): boolean {
  const regex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^a-zA-Z\d]).{8,}$/;

  return regex.test(password);
}
export function verifyEmail(email: string): boolean {
  const regex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

  return regex.test(email);
}
export async function loginAccount(email: string, password: string) {
  try {
    let result = await fetch("https://localhost:8080/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        email: email,
        password: password
      })
    });
    if (!result.body) {
      return {
        status: 400,
        message: "No result body found"
      };
    }
    let body: ReadableStream<Uint8Array> = await result.body;
    let stream = await readStream(body);
    let cleanBody = JSON.parse(stream);
    if (cleanBody.success) {
      return {
        status: 200,
        cypher: cleanBody.body.untis_cypher
      };
    } else {
      return {
        status: 403,
        message: cleanBody.body.message
      };
    }
  } catch (error) {
    return {
      status: 500,
      message: "Server connection failed"
    };
  }
}
export async function registerAccount(
  email: string,
  hashedPassword: string,
  personId: number,
  untisCredentialsEncrypted: string
) {
  try {
    let result = await fetch("https://localhost:8080/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        email: email,
        password: hashedPassword,
        person_id: personId,
        untis_cypher: untisCredentialsEncrypted
      })
    });
    if (!result.body) {
      return {
        status: 400,
        message: "No result body found"
      };
    }
    let body: ReadableStream<Uint8Array> = await result.body;
    let stream = await readStream(body);
    let requestResult = stream.split("\n");
    return {
      status: requestResult[0],
      message: requestResult[1]
    };
  } catch {
    return {
      status: 500,
      message: "Server connection failed"
    };
  }
}
async function readStream(stream: ReadableStream<Uint8Array>) {
  const textDecode = new TextDecoder();
  const chunks = [];
  const reader = stream.getReader();
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }
    chunks.push(textDecode.decode(value));
  }
  return chunks.join("");
}
export async function getTimetable(
  monday: string,
  friday: string
): Promise<{ lessons?: TheScheduleObject[]; status: number; message?: string }> {
  try {
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    if (!storedJSessionId) {
      fetchJSessionId(localStorage.getItem("untis_username"), localStorage.getItem("untis_password")).then((result) => {
        if (result.JSessionId) {
          document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
        }
      });
    }
    const searchQuery = `?from=${monday}&until=${friday}`;
    let resultRaw = await fetch("https://localhost:8080/get_timetable" + searchQuery, {
      method: "GET",
      credentials: "include"
    });
    let resultClean = await resultRaw.json();
    try {
      if (resultClean.body.lessons) {
        return {
          lessons: resultClean.body.lessons,
          status: 200,
          message: undefined
        };
      }
      return {
        lessons: undefined,
        status: resultClean.body.code,
        message: resultClean.body.message
      };
    } catch {
      return {
        status: 400,
        message: "Bad Request"
      };
    }
  } catch {
    return {
      status: 500,
      message: "Server connection failed"
    };
  }
}
async function checkSessionId(): Promise<number> {
  try {
    let result = await fetch("https://localhost:8080/check_session", {
      method: "GET",
      credentials: "include"
    });
    return result.status;
  } catch {
    return 500;
  }
}
export async function verifySession() {
  if (getLocalUntisCredentials()) {
    const status = await checkSessionId();
    return status == 200;
  } else {
    return false;
  }
}
