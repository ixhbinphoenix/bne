/*@jsxImportSource preact */

import type { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";
import "../styles/CookieBanner.scss";

export default function CookieBanner(): JSX.Element | null {
  const [bannerContent, setBannerContent] = useState<JSX.Element | null>(null);
  const [showBanner, toggleBanner] = useState<boolean>(true);

  useEffect(() => {
    if (!document.cookie.match(/^(.*;)?\s*cookie-consent\s*=\s*[^;]+(.*)?$/)) {
      setBannerContent(
        <div className="cookie-banner-container">
          <p className="consent-message">
            Wir benutzen Cookies, die ausschließlich der Funktionalität dieser Seite dienen.
          </p>
          <button className="consent-button" onClick={setConsentCookie}>
            Nicht mehr anzeigen
          </button>
        </div>
      );
    } else {
      setBannerContent(null);
    }
  }, [showBanner]);
  const setConsentCookie = () => {
    document.cookie = `cookie-consent=True; max-age=15552000;`;
    console.log("hiding");
    toggleBanner(false);
  };
  return bannerContent;
}
