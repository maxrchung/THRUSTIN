import "./main.css";
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
             allowedTags={["br","u","table","tr","th","td","a"]} 
             allowedAttributes={
                { 
                    "table": ["class"],
                    "a": ["href"]
                }}
             html={props.content} />
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
                <Message key={this.updateMessageCounter} from="THRUSTY" content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Yeah aite THRUSTIN is a really neat THRUST in the blank game and you can make your own THRUSTS and play them in this console terminal shell interface web format in lobbies. Yeah it's lit thanks to our swaggy devs <a href=&quot;https://osu.ppy.sh/users/1472763&quot;>max</a>, <a href=&quot;https://osu.ppy.sh/users/2747929&quot;>royce</a>, <a href=&quot;https://osu.ppy.sh/users/3071659&quot;>alex</a>. Ok enter '.help' below if you need some more help (obviously)." />,
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
