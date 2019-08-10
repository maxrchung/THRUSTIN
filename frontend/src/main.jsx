import "./main.scss";
import React from 'react';
import ReactDOM from 'react-dom';
import Container from 'react-bootstrap/Container';
import CommandBar from "./CommandBar";
import Message from "./Message";
import MessageText from "./MessageText";
// Don't really need a super secure hash
// Using a fast hash just so we don't "accidentally" see passwords in log
import SHA1 from "crypto-js/sha1";

const MAX_INPUT = 6669;
const MAX_MSGS = 696;

class Client extends React.Component {
    state = {
        inputType: "text",
        messageCounter: 1,
        messages: [
            <Message from="THRUSTY" key={0}>
                <MessageText 
                    content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Yeah aite THRUSTIN is a really neat THRUST in the blank game and you can make your own THRUSTS and play them in this console terminal shell interface web format in lobbies. Yeah it's lit thanks to our swaggy devs <a href=&quot;https://osu.ppy.sh/users/1472763&quot;>max</a>, <a href=&quot;https://osu.ppy.sh/users/2747929&quot;>royce</a>, <a href=&quot;https://osu.ppy.sh/users/3071659&quot;>alex</a>. Ok enter '.help' below if you need some more help (obviously). If you wanta keep up with our development check us out on <a href=&quot;https://github.com/maxrchung/THRUSTIN/&quot;>GitHub.com</a> and <a href=&quot;https://twitter.com/THRUSTIN_rs?iloveducks&quot;>Twitter.com</a>."
                    from="THRUSTY" 
                />
            </Message>
        ],
    }

    // Regex if string is password
    // Capture groups separate command from password sections
    // Only check for account related passwords
    isPasswordRegex = [
        /^(.p )(\w+) (\w+)$/, // account password, not lobby password
        /^(.password )(\w+) (\w+)$/,
        /^(.l \w+ )(\w+)$/,
        /^(.login \w+ )(\w+)$/,
        /^(.r \w+ )(\w+) (\w+)$/,
        /^(.register \w+ )(\w+) (\w+)$/,
    ]

    // Regex if you should use type="password"
    isPasswordTypeRegex = [
        /^.m \w+/,
        /^.make \w+/,
        /^.p \w+/,
        /^.password \w+/,
        /^.l \w+ \w+/,
        /^.login \w+ \w+/,
        /^.r \w+ \w+/,
        /^.register \w+ \w+/,
    ]

    componentDidMount() {
        if (process.env.NODE_ENV === "production") {
            this.connection = new WebSocket("wss://THRUSTIN.rs:3011")
        }
        else {
            this.connection = new WebSocket("ws://localhost:3012")
        }
        this.connection.onmessage = this.handleMessage; 
        this.connection.onclose = this.handleClose;
        document.addEventListener("keydown", this.handleKeyDown);
    }

    componentWillUnmount() {
        document.removeEventListener("keydown", this.handleKeyDown);
    }
    
    getHintVal = () => {
		return document.getElementsByClassName('rbt-input-hint')[0].value;
	}
	
    handleClose = () => {
        this.setMessage("Yo the connection broke so that probably means you were inactive too long or the server blew up. Try refreshing maybe.");
    };

    handleKeyDown = e => {
        const isWhitedModifier = e.getModifierState("Control") || e.getModifierState("Meta") || e.key == "PageDown" || e.key == "PageUp";
        if (document.activeElement !== this.typeahead && !isWhitedModifier) {
            this.typeahead.focus();
        }

        let value = this.typeahead.getInput().value;
		if (e.key == "Enter" && value !== "") {
			const hintVal = this.getHintVal(); // Autocomplete check

			if (hintVal) {
				this.connection.send(hintVal);
			}
			else {
				this.handleMessageMax();
				if (value.length <= MAX_INPUT) {
                    // Hash passwords if detected
                    value = this.matchPassword(value);
					this.connection.send(value);
				}
				else {
					this.setMessage("BRO CHILLOUT that message is too long my man.");
				}
			}

			this.typeahead.clear();
			this.scrollToDummy();
        }
    }

    // Validation stuff to execute when input has been changed, can't be done in keydown
    handleInputChange = value => {
        const inputType = this.testPasswordType(value);
        if (this.state.inputType != inputType) {
            this.setState({
                inputType
            });
        }
    }

    handleMessage = e => {
        this.setJSON(e.data);
    }

    handleMessageMax = () => {
        if (this.state.messageCounter + 1 > MAX_MSGS) {
            var newMsg = this.state.messages.slice(0);
            newMsg.shift();
            this.setState({
                messages: newMsg
            });
            this.scrollToDummy();
        }
    }

    matchPassword = value => {
        for (let regex of this.isPasswordRegex) {
            let match = value.match(regex);
            if (match) {
                // (.l user )(pass)
                if (match.length == 3) {
                    const pass = SHA1(match[2]);
                    const join = match[1] + pass;
                    return join;
                }
                // (.r user)(pass) (confirmation) 
                else { //if (match.length == 4)
                    const pass = SHA1(match[2]);
                    const confirmation = SHA1(match[3]);
                    const join = match[1] + pass + " " + confirmation;
                    return join;
                }
            }
        }
        return value;
    }

    scrollToDummy = () => {
        this.dummy.scrollIntoView();
    }

    setJSON = data => {
        const message = JSON.parse(data);
        this.handleMessageMax();
        this.setState({
            messages: this.state.messages.concat(
                <Message from={message.from} key={this.updateMessageCounter()} >
                    <MessageText content={message.message} from={message.from} />
                </Message>
            )
        });

        this.scrollToDummy();
    }

    setMessage = message => {
        const data = JSON.stringify({
            from: "THRUSTY",
            message: message
        });
       this.setJSON(data);
    }

    testPasswordType = value => {
        for (let regex of this.isPasswordTypeRegex) {
            if (regex.test(value)) {
                return "password";
            }
        }
        return "text";
    }

    updateMessageCounter = () => {
        const counter = this.state.messageCounter;
        this.setState({
            messageCounter: this.state.messageCounter + 1
        });
        return counter;
    }

    renderTop() {
        if (this.state.messageCounter > MAX_MSGS) {
            return <div id="ellipsis" className="py-2">...</div>;
        }
        return (
            <Message from="THRUSTY">
                <img src="favicon-96.png"/>
            </Message>
        );
	}

    render() {
        return (
            <Container fluid={true}>
                <div id="messages">
                    {this.renderTop()}
                    {this.state.messages}
                    <div ref={el => this.dummy = el} />
                </div>
                
                <CommandBar
                    onInputChange={this.handleInputChange}
                    ref={commandBar => {if (commandBar) this.typeahead = commandBar.typeahead}} 
                    type={this.state.inputType}
                />
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
