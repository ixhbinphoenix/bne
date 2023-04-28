/* @jsxImportSource preact */

import "../styles/LoginForm.scss";
import type { JSX } from "preact";
import { useEffect, useState } from "preact/hooks";
import { verifyPassword, verifyEmail, registerAccount, loginAccount, verifySession } from "../api/theBackend";
import { fetchJSessionId, getLocalUntisCredentials, saveUntisCredentials,  } from "../api/untisAPI";
import { generateKey, passwordDecrypt, passwordEncrypt } from "../api/encryption";

export default function LoginForm(): JSX.Element  {
    const [isLogin, setIsLogin] = useState<boolean>(true);
    const [buttonStyle1, setButtonStyle1] = useState({borderBottom: "2px solid #5974e2"})
    const [buttonStyle2, setButtonStyle2] = useState({})
    let password: string, username: string, untisPassword: string, untisUsername: string, personId: number;
    const [untisBoxStyle, setUntiBoxStyle] = useState({})
    const [notice, showPasswordNotice] = useState(<p style={{opacity: "0"}}>A</p>)

    verifySession().then( (session) => {
        if(session) {window.location.href = "/stundenplan"}
    })

    useEffect(() => {
        if(!isLogin) {
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
    }, [isLogin])
    const handleButtonClick = (buttonId: number) => {
        setIsLogin(buttonId == 1)
    }
    const handleSubmit = (event: any) => {
        event.preventDefault()
        if(event.target) {
            if(isLogin) {
                if(event.target[0].value && event.target[1].value) {
                    username = event.target[0].value
                    password = event.target[1].value
                    showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
                    sendLogin()
                    fetchJSessionId(localStorage.getItem("untis_username"), localStorage.getItem("untis_password")).then((result) => {
                        if(result.JSessionId && result.personId) {
                            document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`
                        }
                        else {
                            alert(result.status)
                        }
                    })                       
                }
                else {
                    showPasswordNotice(<p style={{opacity: "100"}}>Bitte fülle alle Felder aus</p>)
                }
            }
            if(!isLogin) {
                if(event.target[0].value && event.target[1].value && event.target[2].value && event.target[3].value) {
                    username = event.target[0].value
                    password = event.target[1].value
                    untisUsername = event.target[2].value
                    untisPassword = event.target[3].value
                    if(!verifyEmail(username)) {
                        showPasswordNotice(<p style={{opacity: "100"}}>Bitte gib eine gültige Mailadresse ein</p>)
                        return
                    }
                    else if(!verifyPassword(password)) {
                        showPasswordNotice(<p style={{opacity: "100"}}>Dein Passwort muss mindestens 8 Zeichen lang sein <br/> und aus Groß-, Kleinbuchstaben, Zahlen und Sonderzeichen bestehen</p>)
                        return
                    }
                    else {
                        showPasswordNotice(<p style={{opacity: "0"}}>A</p>)
                        saveUntisCredentials(event.target[2].value, event.target[3].value)
                        fetchJSessionId(untisUsername, untisPassword).then((result) => {
                            if(result.JSessionId && result.personId) {
                                personId = result.personId;
                                document.cookie = `JSESSIONID=${result.JSessionId}; max-age=600; secure; samesite=none`
                                sendRegister()
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
    const sendRegister = () => {
        const key = generateKey(password)
        const untisCredentials = JSON.stringify({username: untisUsername, password: untisPassword})
        const untisCredentialsEncrtypted = passwordEncrypt(key, untisCredentials).toString()

        registerAccount(username, password, personId, untisCredentialsEncrtypted).then((result) => {
            if(result.status == "200 OK") {
                window.location.href = "/stundenplan"
            }
        })
    }
    const sendLogin = () => {
        const key = generateKey(password)
        loginAccount(username, password).then((result) => {
            if(result.status == 200) {
                const untisCredentialsDecrypted = JSON.parse(passwordDecrypt(key, result.cypher))
                saveUntisCredentials(untisCredentialsDecrypted.username, untisCredentialsDecrypted.password)
                fetchJSessionId(getLocalUntisCredentials().username, getLocalUntisCredentials().password).then((result) => {
                    if(result.status == 200) {
                        window.location.href = "/stundenplan"
                    }
                })
            }
            else {
                showPasswordNotice(<p style={{opacity: "100"}}>{result.message}</p>)
            }
        })
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
                    <input type="username" placeholder="Mailadresse" className="input-box" autocomplete="email"/>
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