import {
  getLocalUntisCredentials,
  fetchJSessionId,
  deleteLocalUntisCredentials
} from "./untisAPI";
import type { TheScheduleObject } from "./main";

class Request {
  //class to handle primitive requests

  public static async Post<T>(path: string, data?: object): Promise<T> {
    try {
      let result = await fetch("https://localhost:8080/" + path, {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        credentials: "include",
        body: JSON.stringify(data)
      });
      if (!result.body) {
        return Promise.reject({ status: 500, message: "Server Connection Failed" });
      }
      let stream = await Request.readStream(result.body);
      let body = JSON.parse(stream);
      if (!body.success) {
        return Promise.reject(body.body);
      }
      return body.body;
    } catch (error) {
      return Promise.reject(error);
    }
  }
  public static async Get<T>(path: string, headers?: HeadersInit): Promise<T> {
    try {
      let result = await fetch("https://localhost:8080/" + path, {
        headers,
        method: "GET",
        credentials: "include"
      });
      const body = await result.json();
      if (!body.success) {
        return Promise.reject(body.body);
      }
      return body.body;
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
export async function getTimetable(monday: string, friday: string): Promise<TheScheduleObject[]> {
  try {
    let body: {lessons: TheScheduleObject[]};
    const searchQuery = `?from=${monday}&until=${friday}`;
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    if (!storedJSessionId) {
      const result = await fetchJSessionId(
        localStorage.getItem("untis_username"),
        localStorage.getItem("untis_password")
      );
      document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
      body = await Request.Get<{lessons: TheScheduleObject[]}>("get_timetable" + searchQuery);
    } else {
      body = await Request.Get<{lessons: TheScheduleObject[]}>("get_timetable" + searchQuery);
    }
    return body.lessons;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getLernbueros(monday: string, friday: string): Promise<TheScheduleObject[]> {
  try {
    let body: {lessons: TheScheduleObject[]};
    const searchQuery = `?from=${monday}&until=${friday}`;
    const storedJSessionId = document.cookie.match("(^|;)\\s*" + "JSESSIONID" + "\\s*=\\s*([^;]+)")?.pop() || "";
    if (!storedJSessionId) {
      const result = await fetchJSessionId(
        localStorage.getItem("untis_username"),
        localStorage.getItem("untis_password")
      );
      document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`;
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_lernbueros" + searchQuery);
    } else {
      body = await Request.Get<{ lessons: TheScheduleObject[] }>("get_lernbueros" + searchQuery);
    }
    return body.lessons;
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
export async function resetPassword(uuid: string, password: string) {
  try {
    let result = await Request.Post(`link/password/reset/${uuid}`, {
      password: password
    });
    return Promise.resolve(result);
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function changePassword(currentPassword: string, newPassword: string) {
  try {
    let result = await Request.Post(`link/password/change`, {
      current_password: currentPassword,
      new_password: newPassword
    });
    return Promise.resolve(result);
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
export async function changeUntisData(password: string, personId: number, untisCredentialsEncrypted: string) {
  try {
    let result = await fetch("https://localhost:8080/change_untis_data", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        password: password,
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
    let body: ReadableStream<Uint8Array> = result.body;
    let stream = await readStream(body);
    let requestResult = stream.split("\n");
    return {
      status: requestResult[0],
      message: requestResult[1]
    };
  } catch {
    return {
      status: 500,
      message: "Server Connection Failed"
    };
  }
}
export async function deleteAccount(password: string) {
  try {
    let result = await fetch("https://localhost:8080/delete_account", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        password: password
      })
    });
    if (!result.body) {
      return {
        status: 400,
        message: "No result body found"
      };
    }
  } catch {
    return {
      status: 500,
      message: "Server Connection Failed"
    };
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
