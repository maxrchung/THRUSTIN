import "./main.css";
import React from 'react';
import ReactDOM from 'react-dom';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import SanitizedHTML from "react-sanitized-html";

function Message(props) {
    return (
        <div>
            <p>
                <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}<br />
                <SanitizedHTML allowedTags={["br","u"]} html={props.content} />
            </p>
            <hr/>
        </div>
    );
}

class Client extends React.Component {
    constructor(props) {
        super(props);

        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleMessage = this.handleMessage.bind(this);
        this.updateMessageCounter = this.updateMessageCounter.bind(this);

        this.state = {
            messageCounter: 0,
            messages: [
                <Message key={this.updateMessageCounter} from="THRUSTY" content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Enter '.help' for help (obviously)." />,
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
    }

    handleMessage(e) {
        this.setState({
            messages: this.state.messages.concat(<Message key={this.updateMessageCounter} from="THRUSTY" content={e.data} />)
        });

        this.scrollToDummy();
    }

    handleKeyDown(e) {
        if (e.key == "Enter") {
            const command = e.target.value;
            this.connection.send(command);
            e.target.value = "";
            this.scrollToDummy();
        }
    }

    updateMessageCounter() {
        const counter = this.state.messageCounter;
        this.setState({
            messageCounter: messageCounter + 1
        });
        return counter;
    }

    scrollToDummy() {
        this.dummy.scrollIntoView();
    }

    render() {
        return (
            <Container fluid={true}>
                <div id="messages">
                    <hr/>
                    {this.state.messages}
                    <div ref={el => this.dummy = el} />
                </div>
                <Form.Control type="text" placeholder="Enter command..." onKeyDown={this.handleKeyDown} />
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);