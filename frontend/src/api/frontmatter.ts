class Request {
  //class to handle primitive requests

  public static async Post(path: string, data?: object): Promise<any> {
    try {
      let result = await fetch("https://api.theschedule.de/" + path, {
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
  public static async Get(path: string, headers?: HeadersInit): Promise<any> {
    try {
      let result = await fetch("https://api.theschedule.de/" + path, {
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
export async function checkSessionIdAstro(id: string): Promise<any> {
  try {
    let result = await Request.Get("check_session", { Cookie: `id=${id}` });
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}

type LinkType = "EmailChange" | "EmailReset" | "PasswordReset" | "VerifyAccount";

export async function checkUUID(uuid: string, type: LinkType): Promise<any> {
  try {
    let result = await Request.Get(`link/check_uuid/${uuid}?type=${type}`);
    return result;
  } catch (error) {
    return Promise.reject(error);
  }
}
