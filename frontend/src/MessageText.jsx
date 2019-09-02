import DOMPurify from "dompurify";
import React from "react";

class MessageText extends React.Component {
    static defaultProps = {
        bg: "000",
        fg: "b7410e",
        from: "THRUSTY",
        level: 0
    }

    render() {
        const props = this.props;
        return (
            <>
                <div className="mb-1">
                    <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}
                    {props.from !== "THRUSTY" &&
                        <div 
                            className="float-right px-1" 
                            style={{
                                color: `#${props.bg}`, 
                                backgroundColor: `#${props.fg}`
                            }}
                        >
                            Level {props.level}
                        </div>
                    }
                </div>
                <div dangerouslySetInnerHTML={{__html: DOMPurify.sanitize(props.content)}} />
            </>
        );
    }
}

export default MessageText;