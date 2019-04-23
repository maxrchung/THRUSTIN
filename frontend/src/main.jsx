import "./main.scss";
import React from 'react';
import ReactDOM from 'react-dom';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import SanitizedHTML from "react-sanitized-html";

function Message(props) {
    return (
        <div className="mb-3 mr-3">
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

class Client extends React.Component {
    constructor(props) {
        super(props);

        this.handleClose = this.handleClose.bind(this);
        this.handleError = this.handleError.bind(this);
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleMessage = this.handleMessage.bind(this);
        this.scrollToDummy = this.scrollToDummy.bind(this);
        this.setMessage = this.setMessage.bind(this);
        this.updateMessageCounter = this.updateMessageCounter.bind(this);

        this.state = {
            messageCounter: 1,
            messages: [
                <Message key={0} from="THRUSTY" content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Yeah aite THRUSTIN is a really neat THRUST in the blank game and you can make your own THRUSTS and play them in this console terminal shell interface web format in lobbies. Yeah it's lit thanks to our swaggy devs <a href=&quot;https://osu.ppy.sh/users/1472763&quot;>max</a>, <a href=&quot;https://osu.ppy.sh/users/2747929&quot;>royce</a>, <a href=&quot;https://osu.ppy.sh/users/3071659&quot;>alex</a>. Ok enter '.help' below if you need some more help (obviously). If you wanta keep up with our development check us out on <a href=&quot;https://github.com/maxrchung/THRUSTIN/&quot;>GitHub.com</a> and <a href=&quot;https://twitter.com/THRUSTIN_rs?iloveducks&quot;>Twitter.com</a>."/>,
            ]
        };
    }

    componentDidMount() {
        if (process.env.NODE_ENV === "production") {
            this.connection = new WebSocket("wss://THRUSTIN.rs:3011")
        }
        else {
            this.connection = new WebSocket("ws://localhost:3012")
        }
        this.connection.onmessage = this.handleMessage;
        this.connection.onclose = this.handleClose;
        this.connection.onerror = this.handleError;
    }

    handleClose() {
        this.setMessage("Yo the connection broke so that probably means you were inactive too long or the server blew up. Try refreshing maybe.");
    }

    handleError() {
        this.setMessage("Um... there was an error with the WebSocket connection to the server. I don't know what's happened, maybe you need to refresh or maybe it's just a temporary problem on our side. Sorry, not sure.");
    }

    handleKeyDown(e) {
        if (e.key == "Enter") {
            const command = e.target.value;
            this.connection.send(command);
            e.target.value = "";
            this.scrollToDummy();
        }
    }

    handleMessage(e) {
        this.setMessage(e.data);
    }

    setMessage(message) {
        this.setState({
            messages: this.state.messages.concat(<Message key={this.updateMessageCounter()} from="THRUSTY" content={message} />)
        });

        this.scrollToDummy();
    }

    scrollToDummy() {
        this.dummy.scrollIntoView();
    }

    updateMessageCounter() {
        const counter = this.state.messageCounter;
        this.setState({
            messageCounter: this.state.messageCounter + 1
        });
        return counter;
    }

    render() {
        return (
            <Container fluid={true}>
                <div id="messages">
                    <img src="favicon-96.png"/>
                    <div className="mb-3 mr-3">
                        <hr/>
                    </div>
                    {this.state.messages}
                    <div ref={el => this.dummy = el} />
                </div>
                <div className="mb-3 mr-3">
                    <Form.Control type="text" placeholder="Enter command..." onKeyDown={this.handleKeyDown} />
                </div>
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
