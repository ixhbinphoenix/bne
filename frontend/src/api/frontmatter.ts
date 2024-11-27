class Request {
  //class to handle primitive requests

  public static async Post(path: string, data?: object): Promise<any> {
    const domain = import.meta.env.PUBLIC_FRONTMATTER_API_DOMAIN;
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
        return Promise.reject(result.status);
      }
      if (!result.body) {
        return Promise.reject({ status: 500, message: "Server Connection Failed" });
      }
      if (result.status == 429) {
        return Promise.reject(new Error("Too many requests. Try again later"));
      }
      let stream = await Request.readStream(result.body);
      let body = JSON.parse(stream);
      return body;
    } catch (error) {
      return Promise.reject(error);
    }
  }
  public static async Get(path: string, headers?: HeadersInit): Promise<any> {
    const domain = import.meta.env.PUBLIC_FRONTMATTER_API_DOMAIN;
    try {
      let result = await fetch(domain + path, {
        headers,
        method: "GET",
        credentials: "include"
      });
      if (!result.ok) {
        return Promise.reject(result.status);
      }
      if (result.status == 429) {
        return Promise.reject(new Error("Too many requests. Try again later"));
      }
      return result.body;
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
    console.log(result);
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
