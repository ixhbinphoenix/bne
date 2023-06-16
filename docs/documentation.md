# Documentation BNE: Stundenplan App

Fortschritt
### 17.11.22 : Beginn der Entwicklung
1. Erste Planung welche Features Die Website haben soll.
2. Github Repository erstellt.
3. Projekt Struktur auf Github erstellt.
4. Licens und Contribution Guidelines zum Projekt Hinzugefügt.
5. Grundsätzliches aussehen der Website geplant.
6. Farb Schema Der Website geplant.
7. Website Name Entschieden = TheSchedule
8. Erste Design Tests.
9. [Link](https://github.com/ixhbinphoenix/bne) zu dem Projekt

### 12.1.23 : Zwischenstand
1. Entwurf der Willkommensseite.
2. Layout der Stundeplanseite (inklusive Testinhalt).

### 16.01.23 : Beginn Serverentwicklung
1. Neue Ordnerstruktur: Aufteilung frontend / backend
2. Beginn API-Wrapperentwicklung
3. Beginn HTTP-Serverentwicklung mit actix und surrealDB

### 09.02.23 : Zwischenstand
1. Erste Version des Plans, die echte Daten anzeigen kann.

### 24.03.23 : Vorläufige Fertigstellung Serverentwicklung
1. Server kann Anfragen bearbeiten
2. API-Wrapper kann Daten von Untis abfragen und formatieren
3. Server kann Datenbank verwalten
4. Datenbank kann eigene Accounts erstellen und verwalten

### 27.03.23 : Beginn Entwicklung Login und Session Validierung
1. Neue Login Seite
2. Session überprüfen

### 28.03.23 : Cookie-Banner
1. Cookie-Banner mit vorläufigem Inhalt ***[NFY]***

### 29.03.23 : Clientside Verschlüsselung
1. Untis-Daten werden mit Nutzerpasswort verschlüsselt und an Server gesendet
2. Eigenes Accountpasswort wird an Server geschickt und unwiderruflich verschlüsselt
3. Untis-Daten werden bei Login abgerufen und entschlüsselt + gespeichert

### 10.04.23 : Navigation
1. Nutzer kann durch verschiedene Wochen navigieren

### 12.04.23 : Vorläufige Fertistellung Login und Session Validierung
1. Nutzeraccounts können über die Loginpage erstellt werden
2. Bestehende Accounts können sich erneut einloggen
3. Untis-Daten werden überprüft und sicher gespeichert

### 16.04.23 : Web Design änderungen
1. [Inter](https://rsms.me/inter/) als neue Standardschriftart.
2. Kleine Fehler behebungen.

### 17.04.23 : Layout für statische Seiten + fehler behebung bei gestrichenen Stunden
1. Statischer Kontent kann in Markdown geschrieben werden und ann In das Layout eingebunden werden.
2. Gestrichene Stunden werden jetzt auch an den Client geschickt.

### 21.04.23 : Aktueller Tag im Stundenplan angezeigt
1. Der aktuelle Tag wird nun im Stundenplan Angezeigt sowie die aktuelle Stunde.

### 30.04.23 : Formattierung + Design änderungen
1. Formattierung zu frontend und backend hinzugefügt.
2. Bessere Implementation für Buttons (Webdesign).

### 01.05.23 : Hintergrund änderungen
1. Hintergrund neu gestaltet.

### 02.05.23 : Datenschutzerklärung + Tage Highlighten
1. Datenschutzerklärung hinzugefügt.
2. Aktuellen Tag nur in der Aktuellen Woche highlighten.

### 03.05.23 : Lernbüros zur Api und Ui hinzugefügt
1. Lernbüros sind nun in der Api unterstützt und in der Ui zu sehen.

### 04.05.23 : Einstellungen sind auf der Homepage zu sehen + fix + Passwort ändern in frontend hizugefügt
1. Einstellungen Button auf Homepage sichtbar.
2. fix: Prüfungen sind keine Lernbüros mehr.
3. Passwort ändern möglich (frontend only).

### 05.05.23 : Login und Registrieren nichtmehr auf gleicher Seite

### 06.05.23 : Nutzungsbedingungen hinzugefügt

### 08.05.23 : Account Settings und Funktionen hinzuefügt
1. Email ändern, Untis Daten ändern, Account Löschen, Ausloggen und überall Ausloggen hinzugefügt als einstellungen.

### 09.05.23 : Datenbank umstrukturiert
1. Neue methode um mit der Datenbank zu interagieren + Nutzernamen änderung.
2. Nutzernamen sind jetzt Emails.

### 10.05.23 : Api feature + Session Rework
1. get_lernbüros methode schickt nun auch Ferien tage.
2. CookieSessionStore geändert zu SurrealSessionStore.

### 12.05.23 : Erste 404 Seite hinzugefügt

### 15.05.23 : [actix-session-surrealdb](https://github.com/ixhbinphoenix/actix-session-surrealdb) update um fehler zu beheben

### 19.05.23 : fix Api + fix login
1. fehler mit get_lernbüros methode behoben.
2. fehler bei Login mit invalidem Id Cookie behoben.

### 27.05.23
1. Datenbank Fehler behoben um logout zu ermöglichen.
2. Nutzungsbedingungen geupdated.
3. Sicherheitskonzept hizugefügt.

### 30.05.23 : Swipe Geste hinzugefügt für Touchscreen

### 06.06.23 : Email Template für ändern der Email hinzugefügt

### 08.06.23 : Verbindung zum Email Server + Temporäre Links hinzugefügt

### 11.06.23 : Email fixes für temporäre links + hinzufügen von Email zurücksetzen

### 12.06.23 : Account Settings + Lb änderungen
1. Email ändern und zurücksetzen.
2. fixen von temporären links.
3. passwort änderung und zurücksetzen.
4.  Lernbüros Zeigen jedes Fach des Lehrers an.

### 13.06.23 : Sicherheits änderungen + mehr
1. Passwort vergessen funktion.
2. Account Verifikation.
3. fix lb.
4. Untis daten ändern setting.
5. account löschen setting.
6. sichereheits fix in temporären links.

### 14.06.23 : Nutzungsbedingung Akzeptieren + EU Datenschutz-Grundverordnung wird jetzt befolgt + mehr
1. Verifations mail erneut senden.
2. nutzerdaten anfragen möglich.
3. rate limiter.
4. nutzer müssen nutzungsbedingungen akzeptieren.

### 15.06.23 : Lb Filter für bessere übersicht
1. kann von bestimmten fach nun jedes lernbüro angucken.

### 16.06.23 : Rendern führt nun sachen nicht doppelt aus