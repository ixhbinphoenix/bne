export async function fetchJSessionId(
  username: string | null,
  password: string | null
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
    return { JSessionId: resultClean.sessionId, personId: resultClean.personId };
  } catch (error) {
    return Promise.reject(error);
  }
}
export function saveUntisCredentials(username: string, password: string) {
  localStorage.setItem("untis_username", username);
  localStorage.setItem("untis_password", password);
}
export function getLocalUntisCredentials(): { username: string; password: string } | null {
  const username = localStorage.getItem("untis_username");
  const password = localStorage.getItem("untis_password");

  return username && password ? { username: username, password: password } : null;
}
export function deleteLocalUntisCredentials() {
  localStorage.removeItem("untis_username");
  localStorage.removeItem("untis_password");
}
