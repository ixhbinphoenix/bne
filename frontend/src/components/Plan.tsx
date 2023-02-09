/* @jsxImportSource preact */

interface IProps {
    activePage: string
}

import Stundenplan from "./plan-components/Stundenplan";
import Kontakt from "./plan-components/Kontakt";
import type { JSX } from "preact";

export default function Plan(props: IProps): JSX.Element {
    
    const tableElements: {[key: string]: JSX.Element} = {
        stundenplan: <Stundenplan></Stundenplan>,
        lernbueros: <div></div>,
        fehler: <div></div>,
        kontakt: <Kontakt></Kontakt>
    }

    return(
        tableElements[props.activePage]
    );
};
