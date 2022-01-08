import "./main.scss";
import React from "react";
import ReactDOM from "react-dom";
import { Container } from "reactstrap";
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
		playerState: "ChooseName",
        inputType: "text",
        messageCounter: 1,
        messages: [
            <Message key={0}>
                <MessageText 
                    content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Yeah aite THRUSTIN is a really neat THRUST in the blank game similar to Cards Against Humanity and you can make your own THRUSTS and play them in this console terminal shell interface web format in lobbies. Yeah it's lit thanks to our swaggy devs <a href=&quot;https://osu.ppy.sh/users/1472763&quot;>max</a>, <a href=&quot;https://osu.ppy.sh/users/3071659&quot;>alex</a>, <a href=&quot;https://osu.ppy.sh/users/2747929&quot;>royce</a>. Ok enter '.help' below if you need some more help (obviously). If you wanta keep up with our development check us out on <a href=&quot;https://github.com/maxrchung/THRUSTIN/&quot;>GitHub.com</a> and <a href=&quot;https://twitter.com/THRUSTIN_rs?iloveducks&quot;>Twitter.com</a>."
                />
            </Message>
        ],
    }

    // Regex if string is password
    // Capture groups separate command from password sections
    // Only check for account related passwords
    isPasswordRegex = [
        /^(\.p )(\w+) (\w+)$/, // account password, not lobby password
        /^(\.password )(\w+) (\w+)$/,
        /^(\.l \w+ )(\w+)$/,
        /^(\.login \w+ )(\w+)$/,
        /^(\.r \w+ )(\w+) (\w+)$/,
        /^(\.register \w+ )(\w+) (\w+)$/,
    ]

    // Regex if you should use type="password"
    isPasswordTypeRegex = [
        /^\.m \w+/,
        /^\.make \w+/,
        /^\.p \w+/,
        /^\.password \w+/,
        /^\.l \w+ \w+/,
        /^\.login \w+ \w+/,
        /^\.r \w+ \w+/,
        /^\.register \w+ \w+/,
	]
	
	stateToOptions = {
		"ChooseName": [".help", ".name", ".login", ".register"],
		"OutOfLobby": [".color", ".help", ".join", ".list", ".make", ".name", ".play", ".THRUST", ".UNTHRUST", ".who", ".account", ".username", ".password", ".ban", ".unban", ".chieftain", ".unchieftain"],
		"InLobby": [".help", ".info", ".leave", ".THRUST", ".UNTHRUST", ".who", ".chief", ".house", ".kick", ".password", ".players", ".points", ".start", ".THRUSTEE", ".THRUSTERS", ".account", ".ban", ".unban", ".chieftain", ".unchieftain"],
		"Playing": [".help", ".info", ".leave", ".THRUST", ".who", ".end", ".kick", ".account", ".ban", ".unban", ".chieftain", ".unchieftain"],
		"Choosing": [".help", ".info", ".leave", ".THRUST", ".who", ".end", ".kick", ".account", ".chieftain", ".unchieftain"],
		"Deciding": [".help", ".info", ".leave", ".THRUST", ".who", ".end", ".kick", ".account", ".ban", ".unban", ".chieftain", ".unchieftain"],
		"Waiting": [".help", ".info", ".leave", ".who", ".end", ".kick", ".account", ".ban", ".unban", ".chieftain", ".unchieftain"],
	}

    componentDidMount() {
        if (process.env.NODE_ENV === "production") {
            this.connection = new WebSocket("wss://server.thrustin.maxrchung.com")
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
	
    handleClose = () => {
        this.setMessage("Yo the connection broke so that probably means you were inactive too long or the server blew up. Try refreshing maybe.");
	};

 	handleKeyDown = e => {
        const isWhitedModifier = e.getModifierState("Control") || e.getModifierState("Meta") || e.key == "PageDown" || e.key == "PageUp";
        if (document.activeElement !== this.typeahead && !isWhitedModifier) {
            this.typeahead.focus();
        }

        const value = this.typeahead.getInput().value;
        if (value) {
            if (e.key == "Enter") {
                if (value.length <= MAX_INPUT) {
                    // Hash passwords if detected
                    const hash = this.matchPassword(value);
                    this.connection.send(hash);
                }
                else {
                    this.setMessage("BRO CHILLOUT that message is too long my man.");
                }
                this.typeahead.clear();
            } else if (e.key == "Escape") {
                this.typeahead.clear();
            }
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
        const shouldScroll = this.shouldScroll();
		const message = JSON.parse(data);
		// Playerstate is not updated when chatting (undefined playerstate)
        const playerState = message.state ? message.state : this.state.playerState;
        
        this.handleMessageMax();
        this.setState({
			playerState,
            messages: this.state.messages.concat(
                <Message bg={message.bg} fg={message.fg} key={this.updateMessageCounter()}>
                    <MessageText bg={message.bg} content={message.message} fg={message.fg} from={message.from} level={message.level}/>
                </Message>
            )
        });

        if (shouldScroll) {
            this.scrollToDummy();
		}
    }

    setMessage = message => {
        const data = JSON.stringify({
            from: "THRUSTY",
            message: message
        });
       this.setJSON(data);
    }

    shouldScroll = () => {
        const scrollHeight = this.messages.scrollHeight;
        const clientHeight = this.messages.clientHeight;
        const scrollTop = this.messages.scrollTop;
        const distFromBottom = scrollHeight - clientHeight - scrollTop;
        // Only scroll to bottom if we are 100 pixels away from the bottom
        const shouldScroll = distFromBottom < 100;
        return shouldScroll;
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
            return (
                <Message>
                    <div id="ellipsis">...</div>
                </Message>
            );
        }
        return (
            <Message>
                <h1 className="mb-0"><img alt="THRUSTIN" src="favicon-96.png"/></h1>
            </Message>
        );
	}

    render() {
        return (
            <Container fluid>
                <div id="messages" ref={el => this.messages = el}>
                    {this.renderTop()}
                    {this.state.messages}
                    <div ref={el => this.dummy = el} />
                </div>
                
                <CommandBar
                    onInputChange={this.handleInputChange}
                    ref={commandBar => {if (commandBar) this.typeahead = commandBar.typeahead}} 
					type={this.state.inputType}
					options={this.stateToOptions[this.state.playerState]}
                />
            </Container>
        );
    }
}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
