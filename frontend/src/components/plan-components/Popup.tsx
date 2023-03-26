/* @jsxImportSource preact */
import type { JSX } from "preact";
import "../../styles/Popup.scss"

interface IProps {
    trigger: boolean;
    setPopupStatus: Function;
    content: JSX.Element | undefined;
}

export default function Popup(props: IProps): JSX.Element | null {

    return (props.trigger) ? (
        <div className={"popup-background"} onClick={() => props.setPopupStatus(false)}>
            <div className={"popup-content"} onClick={() => props.setPopupStatus(false)}>
                {props.content}
            </div>
        </div>
    ) : null;
}