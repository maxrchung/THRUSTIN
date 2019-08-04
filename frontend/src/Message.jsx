import SanitizedHTML from "react-sanitized-html";
import React from 'react';

function Message(props) {
    let isFromUser = props.from == "THRUSTY";
    let fg = isFromUser ? "primary" : "secondary";
    let bg = isFromUser ? "secondary" : "primary";
    return (
        <div className={`text-${fg} bg-${bg} mr-3`}>
            <div className="py-3 px-3">
                <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}<br />
                <SanitizedHTML
                allowedTags={["br","u","table","tr","th","td","a","img"]} 
                allowedAttributes={
                    { 
                        "table": ["class"],
                        "a": ["href"],
                        "img": ["src"]
                    }}
                html={props.content} />
            </div>
            <hr className={`msg-border-${fg} m-0`}/>
        </div>
    );
}

export default Message;