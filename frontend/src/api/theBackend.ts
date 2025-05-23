import { getLocalUntisCredentials, fetchJSessionId, deleteLocalUntisCredentials } from "./untisAPI";
import { FreeRoom, JSESSIONIDCookieString, type TheScheduleObject, type Teacher, ManualLb, ManualLbOverwrite, Jahrgang} from "./main";

class Request {
  //class to handle primitive requests

  public static async Post<T>(path: string, data?: object): Promise<T> {
    const domain = import.meta.env.PUBLIC_BACKEND_API_DOMAIN;
    try {
      let result = await fetch(domain + path, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        credentials: "include",
        body: JSON.stringify(data)
      });
      if (!result.ok) {
        if (result.status === 403) {
          window.location.href == "./login"
        }
        return Promise.reject({ status: result.status, message: await result.text() });
      }
      if (!result.body) {
        return Promise.reject({ status: 500, message: "Server Connection Failed" });
      }
      if (result.status == 429) {
        return Promise.reject(new Error("Too many requests. Try again later"));
      }
      return result.json();
    } catch (error) {
      return Promise.reject(error);
    }
  }
  public static async Get<T>(path: string, headers?: HeadersInit): Promise<T> {
    const domain = import.meta.env.PUBLIC_BACKEND_API_DOMAIN;
    try {
      let result = await fetch(domain + path, {
        headers,
        method: "GET",
        credentials: "include"
      });
      if (result.status == 429) {
        return Promise.reject(new Error("Too many requests. Try again later"));
      }
      if (result.headers.get("Content-Type") === "application/json") {
        return await result.json();
      }
      return await result.text();
    } catch (error) {
      return Promise.reject(error);
    }
  }
  static async readStream(stream: ReadableStream<Uint8Array>) {
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
}

export async function loginAccount(email: string, password: string) {
  try {
    const result = await Request.Post<{ untis_cypher: string }>("login", { email: email, password: password });
    return result.untis_cypher;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function registerAccount(
  email: string,
  password: string,

  personId: number,
  untisCredentialsEncrypted: string
) {
  try {
    const result = await Request.Post("register", {
      email: email,
      password: password,
      person_id: personId,
      untis_cypher: untisCredentialsEncrypted
    });
    return Promise.resolve();
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getTimetable(monday: string, friday: string, className?: string): Promise<TheScheduleObject[]> {
  try {
    let body: { lessons: TheScheduleObject[] };
    let searchQuery = `?from=${monday}&until=${friday}`;
    if (className) {
      searchQuery += `&class_name=${className}`;
    }
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    const untisCredentials = getLocalUntisCredentials();
    if (!storedJSessionId && getLocalUntisCredentials()) {
      const result = await fetchJSessionId(untisCredentials.username, untisCredentials.password);
      document.cookie = JSESSIONIDCookieString(result.JSessionId);
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_timetable" + searchQuery);
    } else {
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_timetable" + searchQuery);
    }
    return body.lessons;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getTimetableServiceWorker(
  monday: string,
  friday: string,
  JSessionId: string,
  className?: string
): Promise<TheScheduleObject[]> {
  try {
    let body: { lessons: TheScheduleObject[] };
    let searchQuery = `?from=${monday}&until=${friday}`;
    if (className) {
      searchQuery += `&class_name=${className}`;
    }
    body = await Request.Post<{ lessons: TheScheduleObject[] }>("get_timetable_serviceworker" + searchQuery, {
      jsessionid: JSessionId
    });
    return body.lessons;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getLernbueros(monday: string, friday: string): Promise<TheScheduleObject[]> {
  try {
    let body: { lessons: TheScheduleObject[] };
    const searchQuery = `?from=${monday}&until=${friday}`;
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    const untisCredentials = getLocalUntisCredentials();
    if (!storedJSessionId && getLocalUntisCredentials()) {
      const result = await fetchJSessionId(untisCredentials.username, untisCredentials.password);
      document.cookie = JSESSIONIDCookieString(result.JSessionId);
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_lernbueros" + searchQuery);
    } else {
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_lernbueros" + searchQuery);
    }
    return body.lessons;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getFreeRooms(monday: string, friday: string): Promise<FreeRoom[]> {
  try {
    let body: { rooms: FreeRoom[] };
    const searchQuery = `?from=${monday}&until=${friday}`;
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    const untisCredentials = getLocalUntisCredentials();
    if (!storedJSessionId && getLocalUntisCredentials()) {
      const result = await fetchJSessionId(untisCredentials.username, untisCredentials.password);
      document.cookie = JSESSIONIDCookieString(result.JSessionId);
      body = await Request.Get<{ rooms: FreeRoom[] }>("get_free_rooms" + searchQuery);
    } else {
      body = await Request.Get<{ rooms: FreeRoom[] }>("get_free_rooms" + searchQuery);
    }
    return body.rooms;
  } catch (error) {
    return Promise.reject(error);
  }
}
async function checkSessionId(): Promise<any> {
  try {
    let result = await Request.Get("check_session");
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function verifySession() {
  try {
    getLocalUntisCredentials();
    await checkSessionId();
    return Promise.resolve();
  } catch (error) {
    deleteLocalUntisCredentials();
    return Promise.reject(error);
  }
}
export async function accountIsVerified(): Promise<boolean> {
  try {
    let result = await Request.Get("verified");
    return result ? Promise.resolve(true) : Promise.reject(result);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function resetPassword(uuid: string, password: string, untisCypher: string, personId: number) {
  try {
    let result = await Request.Post(`link/password/${uuid}`, {
      new_password: password,
      new_untis_cypher: untisCypher,
      new_person_id: personId
    });
    return Promise.resolve(result);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function changePassword(currentPassword: string, newPassword: string, untisCypher: string) {
  try {
    let result = await Request.Post(`change_password`, {
      old_password: currentPassword,
      new_password: newPassword,
      new_untis_cypher: untisCypher
    });
    return Promise.resolve(result);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function forgotPassword(email: string): Promise<any> {
  try {
    let result = await Request.Post("forgot_password", {
      mail: email
    });
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function demandEmail() {
  try {
    let result = await Request.Get("change_email");
    return Promise.resolve(result);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function resetEmail(uuid: string, email: string) {
  try {
    let result = await Request.Post(`link/email_reset/${uuid}`, {
      mail: email
    });
    return Promise.resolve();
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function changeEmail(uuid: string, password: string, email: string) {
  try {
    let result = await Request.Post(`link/email_change/${uuid}`, {
      password: password,
      mail: email
    });
    return Promise.resolve();
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function changeUntisData(
  password: string,
  personId: number,
  untisCredentialsEncrypted: string
): Promise<string> {
  try {
    let result = await Request.Post<string>("change_untis_data", {
      password: password,
      person_id: personId,
      untis_cypher: untisCredentialsEncrypted
    });
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function resendVerifyEmail() {
  try {
    let result = await Request.Get("resend_mail");
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function verifyAccount(uuid: string): Promise<string> {
  try {
    let result = await Request.Get<string>(`link/verify/${uuid}`);
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function deleteAccount(password: string) {
  try {
    let result = await Request.Post("delete", {
      password: password
    });
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function GDPRData() {
  try {
    let result = await Request.Get("gdpr_data_compliance");
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function logout() {
  try {
    await Request.Post("logout");
    deleteLocalUntisCredentials();
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function logoutAll() {
  try {
    await Request.Post("logout_all");
    deleteLocalUntisCredentials();
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getTeachers() {
  try {
    return await Request.Get < { teachers: Teacher[]} >("get_teachers");
  }
  catch (error) {
    return Promise.reject(error)
  }
}
export async function getJahrgaenge() {
  try {
    return await Request.Get < { jahrgaenge: Jahrgang[]} >("get_jahrgaenge");
  }
  catch (error) {
    return Promise.reject(error)
  }
}
export async function saveTeachers(teachers: Teacher[]) {
  try {
    return await Request.Post("save_teachers", teachers)
  }
  catch (error) {
    return Promise.reject(error)
  }
}
export async function getManualLbs() {
  try {
    return await Request.Get<{ lbs: ManualLb[] }>("get_manual_lbs");
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function saveManualLbs(lbs: ManualLb[]) {
  try {
    return await Request.Post("save_manual_lbs", lbs);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function sendJahrgaenge(jahrgaenge: Jahrgang[]) {
  try {
    return await Request.Post("save_jahrgaenge", jahrgaenge);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getManualLbOverwrites() {
  try {
    return await Request.Get<{ lbs: ManualLbOverwrite[] }>("get_manual_lb_overwrites");
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function saveManualLbOverwrites(lbs: ManualLbOverwrite[]) {
  try {
    return await Request.Post("save_manual_lb_overwrites", lbs);
  } catch (error) {
    return Promise.reject(error);
  }
}