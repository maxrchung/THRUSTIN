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
        this.commandBar.focus();
        document.addEventListener("keydown", this.handleKeyDown);
    }

    handleClose = () => {
        this.setMessage("Yo the connection broke so that probably means you were inactive too long or the server blew up. Try refreshing maybe.");
    };

    handleKeyDown = (e) => {
        if (document.activeElement !== this.commandBar) {
            this.commandBar.focus();
        }
        if (e.key == "Enter" && this.commandBar.value !== "") {
            const command = this.commandBar.value;
            this.connection.send(command);
            this.setState({
                messages: this.state.messages.concat(<Message key={this.updateMessageCounter()} from="You" content={command} />)
            });
            this.commandBar.value = "";
            this.scrollToDummy();
        }
    }

    handleMessage = (e) => {
        this.setMessage(e.data);
    }

    setMessage = (message) => {
        this.setState({
            messages: this.state.messages.concat(<Message key={this.updateMessageCounter()} from="THRUSTY" content={message} />)
        });

        this.scrollToDummy();
    }

    scrollToDummy = () => {
        this.dummy.scrollIntoView();
    }

    updateMessageCounter = () => {
        const counter = this.state.messageCounter;
        this.setState({
            messageCounter: this.state.messageCounter + 1
        });
        return counter;
    }

    componentWillUnmount() {
        document.removeEventListener("keydown", this.handleKeyDown);
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
                    <Form.Control ref={(input) => {this.commandBar = input}} type="text" placeholder="Enter command..." />
                </div>
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
