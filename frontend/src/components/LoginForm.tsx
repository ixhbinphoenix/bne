/* @jsxImportSource preact */

import "../styles/LoginForm.scss"
import type { JSX } from "preact"
import { useEffect, useState } from "preact/hooks"
import { verifyPassword, verifyEmail, registerAccount, loginAccount, fetchJSessionId, saveUntisCredentials } from "../api/main";

export default function LoginForm(): JSX.Element  {
    const [activeButton, setActiveButton] = useState<number>(1);
    const [buttonStyle1, setButtonStyle1] = useState({borderBottom: "2px solid #5974e2"})
    const [buttonStyle2, setButtonStyle2] = useState({})
    const [password, setPassword] = useState(""), [username, setUsername] = useState(""), [untisPassword, setUntisPassword] = useState(""), [untisUsername, setUntisUsername] = useState("")
    const [untisBoxStyle, setUntiBoxStyle] = useState({})
    const [notice, showPasswordNotice] = useState(<p style={{opacity: "0"}}>A</p>)

    
    useEffect(() => {
        if(activeButton == 2) {
            setButtonStyle2({borderBottom: "2px solid #5974e2"})
            setButtonStyle1({borderBottom: "none"})
            showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
            setUntiBoxStyle({opacity: "100"})
        }
        else {
            setButtonStyle1({borderBottom: "2px solid #5974e2"})
            setButtonStyle2({borderBottom: "none"})
            showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
            setUntiBoxStyle({opacity: "0"})
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
                    showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
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
                    showPasswordNotice(<p style={{opacity: "100"}}>Bitte fülle alle Felder aus</p>)
                }
            }
            if(activeButton == 2) {
                if(event.target[0].value && event.target[1].value && event.target[2].value && event.target[3].value) {
                    console.log("success")
                    setUsername(event.target[0].value)
                    setPassword(event.target[1].value)
                    setUntisUsername(event.target[2].value)
                    setUntisPassword(event.target[3].value)
                    if(!verifyEmail(event.target[0].value)) {
                        showPasswordNotice(<p style={{opacity: "100"}}>Bitte gib eine gültige Mailadresse ein</p>)
                        return
                    }
                    else if(!verifyPassword(event.target[1].value)) {
                        showPasswordNotice(<p style={{opacity: "100"}}>Dein Passwort muss mindestens 8 Zeichen lang sein <br/> und aus Groß-, Kleinbuchstaben, Zahlen und Sonderzeichen bestehen</p>)
                        return
                    }
                    else {
                        showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
                        saveUntisCredentials(event.target[2].value, event.target[3].value)
                        fetchJSessionId(event.target[2].value, event.target[3].value).then((result) => {
                            if(result.JSessionId && result.personId) {
                                document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=strict`
                                registerAccount("Account", "1Passwort!", result.personId)
                                window.location.href = "/stundenplan"
                            }
                            else {
                                showPasswordNotice(<p style={{opacity: "100"}}>Deine Untiszugangsdaten sind nicht korrekt</p>)
                            }
                        })
                    }
                        
                }
                else {
                    showPasswordNotice(<p style={{opacity: "100"}}>Bitte fülle alle Felder aus</p>)
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
                    <button id="login" style={buttonStyle1} onClick={() => {handleButtonClick(1)} }>Anmelden</button>
                    <button id="register" style={buttonStyle2}onClick={() => {handleButtonClick(2)}}>Registrieren</button>
                </div>
                <form onSubmit={handleSubmit}>
                    <input type="username" placeholder="Mailadresse" className="input-box" />
                    <input type="password" placeholder="Passwort" className="input-box" />
                    <div className="password-notice">{notice}</div>
                    <input id="untis-username" type="username" placeholder="Units-Nutzername" className="input-box untis-box" style={untisBoxStyle}/>
                    <input type="password" placeholder="Untis-Passwort" className="input-box untis-box" style={untisBoxStyle}/>
                    <div className="button-container">
                        <input type="submit" id="submit-button" value="Absenden"/>
                    </div>
                </form>
            </div>
            
        </div>

    )
}