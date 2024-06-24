/* @jsxImportSource preact */

import "../../../styles/SettingsElement.scss";
import type { JSX } from "preact";
import { useState } from "preact/hooks";
import { getLocalUntisCredentials } from "../../../api/untisAPI";

export default function Notifications(): JSX.Element {
  const [errorMessage, setErrorMessage] = useState(<p></p>);
  const unsubscribeNotification = async () => {
    try {
      const worker = await navigator.serviceWorker.getRegistration();
      await worker?.unregister();
      setErrorMessage(<p>Benachrichtigungen deaktiviert</p>);
    } catch {}
  };
  const enableNotification = async () => {
    await Notification.requestPermission();
    if (Notification.permission === "granted") {
      if ("serviceWorker" in navigator) {
        navigator.serviceWorker.register("/notificationWorker.js", { scope: "/home" }).then((worker) => {
          worker.active?.postMessage(getLocalUntisCredentials());
        });
        navigator.serviceWorker.ready.then((worker) => {
          worker.active?.postMessage(getLocalUntisCredentials());
        });
      }
      setErrorMessage(<p>Benachrichtigungen aktiviert</p>);
    }
  };
  return (
    <div class="page-content">
      <div class="form-container">
        <h2>Verwalte deine Benachrichtigungen</h2>
        <button onClick={enableNotification}>Benachrichtigungen aktivieren</button>
        <button onClick={unsubscribeNotification}>Benachrichtigungen deaktiveren</button>
        <div class="error-message">{errorMessage}</div>
      </div>
    </div>
  );
}
