import React, { Component } from 'react';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import SanitizedHTML from "react-sanitized-html";

function Message(props) {
    return (
        <p>
            <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}<br />
            <SanitizedHTML allowedTags={["br"]} html={props.content} />
        </p>
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
        this.connection = new WebSocket("ws://localhost:3012")
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
            this.connection.send(e.target.value);
            e.target.value = "";
            this.scrollToDummy();
        }
    }

    updateMessageCounter() {
        var counter = this.state.messageCounter;
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
            <Container>
                <div id="messages">
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
