import {
  getLocalUntisCredentials,
  fetchJSessionId,
  saveUntisCredentials,
  deleteLocalUntisCredentials
} from "./untisAPI";
import type { TheScheduleObject } from "./main";

class Request {
  //class to handle primitive requests

  public static async Post(path: string, data: object): Promise<any> {
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
  public static async Get(path: string): Promise<any> {
    try {
      let result = await fetch("https://localhost:8080/" + path, {
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
    const result = await Request.Post("login", { email: email, password: password });
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
export async function getTimetable(monday: string, friday: string): Promise<TheScheduleObject[] | any> {
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
    let body = await Request.Get("get_timetable" + searchQuery);
    return body.lessons;
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function getLernbueros(monday: string, friday: string): Promise<any> {
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
    let body = await Request.Get("get_lernbueros" + searchQuery);
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
    if (getLocalUntisCredentials()) {
      await checkSessionId();
      return Promise.resolve();
    }
  } catch (error) {
    return Promise.reject(error);
  }
}
export async function changePassword(currentPassword: string, newPassword: string) {
  try {
    let result = await fetch("https://localhost:8080/change_password", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        current_password: currentPassword,
        new_password: newPassword
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
export async function changeEmail(password: string, email: string) {
  try {
    let result = await fetch("https://localhost:8080/change_email", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      credentials: "include",
      body: JSON.stringify({
        password: password,
        email: email
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
export function logoutHere() {
  deleteLocalUntisCredentials();
  fetch("https://localhost:8080/logout_here", {
    method: "GET",
    credentials: "include"
  });
}
export function logoutEverywhere() {
  deleteLocalUntisCredentials();
  fetch("https://localhost:8080/logout_everywhere", {
    method: "GET",
    credentials: "include"
  });
}
