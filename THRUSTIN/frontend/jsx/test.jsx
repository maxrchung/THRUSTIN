import React, { Component } from 'react';
import Container from 'react-bootstrap/Container'
import Row from 'react-bootstrap/Row'
import Col from 'react-bootstrap/Col'
import Form from 'react-bootstrap/Form'

function Message(props) {
    return (
        <p>
            <strong>{props.from}</strong> {(new Date).toLocaleTimeString()}<br/>
            {props.content}
        </p>
    );
}

class Client extends React.Component {
    constructor(props) {
        super(props);
        this.handleMessage = this.handleMessage.bind(this);
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.updateMessageCounter = this.updateMessageCounter.bind(this);

        this.connection = new WebSocket("ws://localhost:3012")
        this.connection.onmessage = this.handleMessage;

        this.state = {
            messageCounter: 0,
            messages: [
                <Message key={this.updateMessageCounter} from="THRUSTY" content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Type 'help' for help (obviously)." />,
            ]
        };
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
        }
    }

    updateMessageCounter() {
        var counter =  this.state.messageCounter;
        this.setState({
            messageCounter: messageCounter + 1
        });
        return counter;
    }

    scrollToDummy() {
        this.dummy.scrollIntoView({ behavior: "smooth"});
    }

    render() {
        return (
            <Container bsPrefix="container">
                <div id="messages">
                    {this.state.messages}
                    <div ref={ (element) => { this.dummy = element; }}>
                    </div>
                </div>
                <Form.Control id="message" type="text" placeholder="Type command..." onKeyDown={this.handleKeyDown}></Form.Control>
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById('test')
);
