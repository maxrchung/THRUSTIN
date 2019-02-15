import React, { Component } from 'react';
import Container from 'react-bootstrap/Container'
import Form from 'react-bootstrap/Form'

function Message(props) {
    return (
        <p>
            <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}<br />
            {props.content}
        </p>
    );
}

class Client extends React.Component {
    constructor(props) {
        super(props);

        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleMessage = this.handleMessage.bind(this);
        this.handleScroll = this.handleScroll.bind(this);
        this.updateMessageCounter = this.updateMessageCounter.bind(this);

        this.state = {
            scrolledToBottom: true,
            messageCounter: 0,
            messages: [
                <Message key={this.updateMessageCounter} from="THRUSTY" content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Type 'help' for help (obviously)." />,
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

        if (this.state.scrolledToBottom) {
            this.scrollToDummy();
        }
    }

    handleScroll(e) {
        const bottom = e.target.scrollHeight - e.target.scrollTop === e.target.clientHeight;
        this.setState({
            scrolledToBottom: bottom ? true : false
        });
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
                <div id="messages" onScroll={this.handleScroll}>
                    {this.state.messages}
                    <div ref={el => this.dummy = el}>
                    </div>
                </div>
                <Form.Control type="text" placeholder="Enter command..." onKeyDown={this.handleKeyDown}></Form.Control>
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
