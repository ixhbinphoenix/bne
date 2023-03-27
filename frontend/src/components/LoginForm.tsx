/* @jsxImportSource preact */

import "../styles/LoginForm.scss"
import type { JSX } from "preact"
import { useState } from "preact/hooks"

export default function LoginForm(): JSX.Element  {
    const [activeButton, setActiveButton] = useState<number>(1);

    const handleButtonClick = (buttonId: number) => {
        setActiveButton(buttonId)
    }

    return(
        <div className="form-container">
            <div className="description">
                <h1 className="title">Willkommen</h1>
                <p className="subtitle">Melde dich an, oder erstelle einen neuen Account</p>
            </div>
            <div className="login-switch">
                <button id="login" onClick={() => {handleButtonClick(1)}}>Anmelden</button>
                <button id="register" onClick={() => {handleButtonClick(2)}}>Registrieren</button>
                <div className="underline" style={{
                    transform: activeButton === 1 ? 'translateX(0%)' : 'translateX(88%)',
                    width: activeButton === 1 ? '15%' : '17.675%'
                    }
                    }>
                </div>
            </div>
            <div className="login-box-container">
                <input type="username" placeholder="Nutzername" className="input-box"/>
                <input type="password" placeholder="Passwort" className="input-box" />
                <p className="untis-login">Gib hier deine Untis-Daten ein:</p>
                <input type="username" placeholder="Units-Nutzername" className="input-box"/>
                <input type="password" placeholder="Untis-Passwort" className="input-box" />
            </div>
        </div>

    )
}