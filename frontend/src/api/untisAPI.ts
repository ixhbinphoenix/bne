export async function fetchJSessionId(
  username: string | null,
  password: string | null
): Promise<{ status: number; JSessionId: string | null; personId: number | null }> {
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
  try {
    return {
      status: 200,
      JSessionId: resultClean.result.sessionId,
      personId: resultClean.result.personId
    };
  } catch {
    return {
      status: 401,
      JSessionId: null,
      personId: null
    };
  }
}
export function saveUntisCredentials(username: string, password: string) {
  localStorage.setItem("untis_username", username);
  localStorage.setItem("untis_password", password);
}
export function getLocalUntisCredentials(): { username: string | null; password: string | null } {
  const username = localStorage.getItem("untis_username");
  const password = localStorage.getItem("untis_password");
  if (username && password) {
    return { username: username, password: password };
  } else {
    return { username: null, password: null };
  }
}
