import "./main.scss";
import React from 'react';
import ReactDOM from 'react-dom';
import Container from 'react-bootstrap/Container';
import CommandBar from "./CommandBar";
import Message from "./Message";
import MessageText from "./MessageText";

const MAX_INPUT = 6669;
const MAX_MSGS = 696;

class Client extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            messageCounter: 1,
            messages: [
                <Message from="THRUSTY" key={0}>
                    <MessageText 
                        content="Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to THRUSTING! Yeah aite THRUSTIN is a really neat THRUST in the blank game and you can make your own THRUSTS and play them in this console terminal shell interface web format in lobbies. Yeah it's lit thanks to our swaggy devs <a href=&quot;https://osu.ppy.sh/users/1472763&quot;>max</a>, <a href=&quot;https://osu.ppy.sh/users/2747929&quot;>royce</a>, <a href=&quot;https://osu.ppy.sh/users/3071659&quot;>alex</a>. Ok enter '.help' below if you need some more help (obviously). If you wanta keep up with our development check us out on <a href=&quot;https://github.com/maxrchung/THRUSTIN/&quot;>GitHub.com</a> and <a href=&quot;https://twitter.com/THRUSTIN_rs?iloveducks&quot;>Twitter.com</a>."
                        from="THRUSTY" 
                    />
                </Message>
			],
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
        document.addEventListener("keydown", this.handleKeyDown);
    }

    componentWillUnmount() {
        document.removeEventListener("keydown", this.handleKeyDown);
    }
    
    getHintVal() {
		return document.getElementsByClassName('rbt-input-hint')[0].value;
	}
	
    handleClose = () => {
        this.setMessage("Yo the connection broke so that probably means you were inactive too long or the server blew up. Try refreshing maybe.");
    };

    handleKeyDown = (e) => {
        var isWhitedModifier = e.getModifierState("Control") || e.getModifierState("Meta") || e.key == "PageDown" || e.key == "PageUp";

        if (document.activeElement !== this.typeahead && !isWhitedModifier) {
            this.typeahead.focus();
        }
		if (e.key == "Enter" && this.inputBar.value !== "") {
			const hintVal = this.getHintVal(); // Autocomplete check

			if (hintVal) {
				this.connection.send(hintVal);
			}
			else {
				this.handleMessageMax();
				const command = this.inputBar.value;
				if (command.length <= MAX_INPUT) { 
					this.connection.send(command);
				}
				else {
					this.setMessage("BRO CHILLOUT that message is too long my man.");
				}
				
			}

			this.typeahead.clear();
			this.scrollToDummy();
		}
    }

    handleMessage = (e) => {
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

    scrollToDummy = () => {
        this.dummy.scrollIntoView();
    }

    setMessage = (message) => {
        const data = JSON.stringify({
            from: "THRUSTY",
            message: message
        });
       this.setJSON(data);
    }

    setJSON = (data) => {
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

    renderTop() {
        if (this.state.messageCounter > MAX_MSGS) {
            return <div id="ellipsis" className="py-2">...</div>;
        }
        return (
            <>
                <Message from="THRUSTY">
                    <img src="favicon-96.png"/>
                </Message>
            </>
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
					ref={(comBar) => {
						if(comBar) {
							this.typeahead = comBar.wrappedTypeahead;
							this.inputBar = comBar.wrappedTypeahead.getInput();
						}
					}}
				/>
            </Container>
        );
    }

    updateMessageCounter = () => {
        const counter = this.state.messageCounter;
        this.setState({
            messageCounter: this.state.messageCounter + 1
        });
        return counter;
    }

}

ReactDOM.render(
    <Client />,
    document.getElementById("root")
);
