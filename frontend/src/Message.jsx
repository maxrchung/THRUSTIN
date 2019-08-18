import React from 'react';

class Message extends React.Component {
    static defaultProps = {
        bg: "000",
        fg: "b7410e",
        from: "THRUSTY"
    }

    // Fade foreground by about 50%
    borderColor = () => {
        let hex = this.props.fg;
        if (hex.length === 3) {
            // 0008
            return hex + "8"; 
        } else if (hex.length === 6) {
            // 00000088
            return hex + "88";
        } else {
            return hex;
        }
    }

    render() {
        let props = this.props;
        let bg = props.bg;
        let fg = props.fg;
        let border = this.borderColor();
        return (
            <div className="mr-3" style={{
                color: `#${fg}`, 
                backgroundColor: `#${bg}`
            }}>
                <div className="py-3 px-3">
                    {props.children}
                </div>
                <hr className="m-0" style={{
                    borderTop: `1px solid #${border}`
                }} />
            </div>
        );
    }
}

export default Message;