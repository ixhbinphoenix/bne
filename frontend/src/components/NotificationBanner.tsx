/*@jsxImportSource preact */

import "@fontsource/inter";
import type { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";
import "../styles/CookieBanner.scss";

export default function NotificationBanner(): JSX.Element | null {
  const [bannerContent, setBannerContent] = useState<JSX.Element | null>(null);
  const [showBanner, toggleBanner] = useState<boolean>(true);

  useEffect(() => {
    if (!document.cookie.match(/^(.*;)?\s*notification-consent\s*=\s*[^;]+(.*)?$/)) {
      setBannerContent(
        <div className="cookie-banner-container">
          <p className="consent-message">
            Wir können dir Benachrichtigungen über Entfälle senden.
            <br />
            Du kannst sie jederzeit in den Einstellungen ändern
          </p>
          <button className="consent-button" onClick={setConsentCookie}>
            Benachrichtigungen erlauben
          </button>
        </div>
      );
    } else {
      setBannerContent(null);
    }
  }, [showBanner]);
  const setConsentCookie = () => {
    Notification.requestPermission().then((permission) => {
      if (permission === "granted") {
        if ("serviceWorker" in navigator) {
          navigator.serviceWorker.register("/notificationWorker.js", { scope: "/home" }).then((worker) => {
            console.log(worker.scope);
          });
        }
      }
    });
    document.cookie = `notification-consent=True; max-age=15552000;`;
    toggleBanner(false);
  };
  return bannerContent;
}
