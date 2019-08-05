import React from 'react';

function Message(props) {
    let isFromUser = props.from == "THRUSTY";
    let fg = isFromUser ? "primary" : "secondary";
    let bg = isFromUser ? "secondary" : "primary";
    return (
        <div className={`text-${fg} bg-${bg} mr-3`}>
            <div className="py-3 px-3">
                {props.children}
            </div>
            <hr className={`msg-border-${fg} m-0`}/>
        </div>
    );
}

export default Message;