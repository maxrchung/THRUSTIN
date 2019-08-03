import SanitizedHTML from "react-sanitized-html";
import React from 'react';

function Message(props) {
    return (
        <div>
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
            <hr/>
        </div>
    );
}

export default Message;