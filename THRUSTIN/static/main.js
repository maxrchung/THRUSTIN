var ws = new WebSocket("ws://192.168.0.2:8000/");

ws.onmessage = function (e) {
    addMessage(e.data);
};

function send() {
    var command = document.getElementById("command");
    var text = command.value;
    addMessage(text, true);
    ws.send(text);
    command.value = "";
}

function clear() {
    console.log("clear()");
    var messages = document.getElementById("messages");
    messages.innerHTML = "";
}

function handleEnter(e){
    if(e.keyCode === 13){
        e.preventDefault(); // Ensure it is only this code that rusn
        send();
    }
}

function addMessage(text, fromSelf) {
    var line = document.createElement("p");
    if (fromSelf) {
        line.classList.add("from-self");
    }
    var date = new Date();
    var content = document.createTextNode("[" + date.toLocaleTimeString() + "] " + text);
    line.append(content);

    var messages = document.getElementById("messages");
    messages.prepend(line);
}

var sendButton = document.getElementById("send");
sendButton.addEventListener("click", send, false);

var clearButton = document.getElementById("clear");
clearButton.addEventListener("click", clear, false);

var command = document.getElementById("command");
command.addEventListener("keypress", handleEnter, false);

addMessage("Welcome to THRUSTIN! I'm THRUSTY, your trusty guide to thrusting!");
