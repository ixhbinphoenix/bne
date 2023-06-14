export async function fetchJSessionId(
  username: string,
  password: string
): Promise<{ JSessionId: string; personId: number } | any> {
  try {
    let resultRaw = await fetch("https://borys.webuntis.com/WebUntis/jsonrpc.do?school=ges-münster", {
      method: "POST",
      body: JSON.stringify({
        id: "theSchedule",
        method: "authenticate",
        params: { user: username, password: password, client: "theSchedule" },
        jsonrpc: "2.0"
      })
    });
    let resultClean = await resultRaw.json();
    if (!resultClean.result) {
      return Promise.reject(resultClean.error);
    }
    return { JSessionId: resultClean.result.sessionId, personId: resultClean.result.personId };
  } catch (error) {
    return Promise.reject(error);
  }
}
export function saveUntisCredentials(username: string, password: string) {
  localStorage.setItem("untis_username", username);
  localStorage.setItem("untis_password", password);
}
export function getLocalUntisCredentials(): { username: string; password: string } {
  const username = localStorage.getItem("untis_username");
  const password = localStorage.getItem("untis_password");
  if (username && password) {
    return { username: username, password: password };
  }
  throw new Error("No Untis-Credentials saved");
}
export function deleteLocalUntisCredentials() {
  localStorage.removeItem("untis_username");
  localStorage.removeItem("untis_password");
}
