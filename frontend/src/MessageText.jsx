import DOMPurify from "dompurify";
import React from "react";

function MessageText(props) {
    return (
        <>
            <strong>{props.from}</strong> {(new Date).toLocaleTimeString()} <br/>
            <div dangerouslySetInnerHTML={{__html: DOMPurify.sanitize(props.content)}} />
        </>
    );
}

export default MessageText;