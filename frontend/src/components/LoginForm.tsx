/* @jsxImportSource preact */

import "../styles/LoginForm.scss"
import type { JSX } from "preact"
import { useEffect, useState } from "preact/hooks"
import { verifyPassword, registerAccount, loginAccount, fetchJSessionId, saveUntisCredentials } from "../api/main";

export default function LoginForm(): JSX.Element  {
    const [activeButton, setActiveButton] = useState<number>(1);
    const [password, setPassword] = useState(""), [username, setUsername] = useState(""), [untisPassword, setUntisPassword] = useState(""), [untisUsername, setUntisUsername] = useState("")
    const [untisBoxStyle, setUntiBoxStyle] = useState({})
    const [notice, showPasswordNotice] = useState(<></>)
    
    useEffect(() => {
        if(activeButton == 2) {
            setUntiBoxStyle({display: "inline-block"})
        }
        else {
            setUntiBoxStyle({display: "none"})
        }
    }, [activeButton])

    const handleButtonClick = (buttonId: number) => {
        setActiveButton(buttonId)
    }
    const handleSubmit = (event: any) => {
        event.preventDefault()
        if(event.target) {
            if(activeButton == 1) {
                console.log(event.target[0].value, event.target[1].value)
                if(event.target[0].value && event.target[1].value) {
                    setUsername(event.target[0].value)
                    setPassword(event.target[1].value)
                    showPasswordNotice(<></>)
                    fetchJSessionId(localStorage.getItem("untis_username"), localStorage.getItem("untis_password")).then((result) => {
                        if(result.JSessionId && result.personId) {
                            document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=strict`
                            registerAccount("Account", "1Passwort!", result.personId)
                        }
                        else {
                            alert(result.status)
                        }
                    })
                    loginAccount(username, password)
                        
                }
                else {
                    showPasswordNotice(<p>Bitte fülle alle Felder aus</p>)
                }
            }
            if(activeButton == 2) {
                if(event.target[0].value && event.target[1].value && event.target[2].value && event.target[3].value) {
                    console.log("success")
                    setUsername(event.target[0].value)
                    setPassword(event.target[1].value)
                    setUntisUsername(event.target[2].value)
                    setUntisPassword(event.target[3].value)
                    if(!verifyPassword(event.target[1].value)) {
                        showPasswordNotice(<p>Dein Passwort muss mindestens 8 Zeichen lang sein <br/> und Groß-, Kleinbuchstaben, Zahlen und Sonderzeichen bestehen</p>)
                    }
                    else {
                        showPasswordNotice(<></>)
                        saveUntisCredentials(event.target[2].value, event.target[3].value)
                        fetchJSessionId(event.target[2].value, event.target[3].value).then((result) => {
                            if(result.JSessionId && result.personId) {
                                document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=strict`
                                registerAccount("Account", "1Passwort!", result.personId)
                            }
                            else {
                                showPasswordNotice(<p>Deine Untiszugangsdaten sind nicht korrekt</p>)
                            }
                        })
                    }
                        
                }
                else {
                    showPasswordNotice(<p>Bitte fülle alle Felder aus</p>)
                }
            }
        }
    }
    const sendRequest = () => {
    }
    return(
        <div className="form-container">
            <div className="description">
                <h1 className="title">Willkommen</h1>
                <p className="subtitle">Melde dich an, oder erstelle einen neuen Account</p>
            </div>
            <div className="login-box-container">
                <div className="login-switch">
                    <button id="login" onClick={() => {handleButtonClick(1)}}>Anmelden</button>
                    <button id="register" onClick={() => {handleButtonClick(2)}}>Registrieren</button>
                    <div className="underline" style={{
                        transform: activeButton === 1 ? 'translateX(3%)' : 'translateX(86%)',
                        width: activeButton === 1 ? '18%' : '22%'
                        }
                        }>
                    </div>
                </div>
                <form onSubmit={handleSubmit}>
                    <input type="username" placeholder="Nutzername" className="input-box" />
                    <input type="password" placeholder="Passwort" className="input-box" />
                    <p className="untis-login untis-box" style={untisBoxStyle}>Gib hier deine Untis-Daten ein:</p>
                    <input type="username" placeholder="Units-Nutzername" className="input-box untis-box" style={untisBoxStyle}/>
                    <input type="password" placeholder="Untis-Passwort" className="input-box untis-box" style={untisBoxStyle}/>
                    <div className="button-container">
                        <input type="submit" id="submit-button" />
                    </div>
                </form>
                <div className="password-notice">{notice}</div>
            </div>
            
        </div>

    )
}