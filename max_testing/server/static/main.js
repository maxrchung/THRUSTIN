var ws = new WebSocket("ws://192.168.0.4:3012/");

ws.onopen = function () {
    // Web Socket is connected, send data using send()
    ws.send("onopen");
};

ws.onclose = function () {
    // websocket is closed.
};

ws.onmessage = function (e) {
    appendLine("THRUSTY: " + e.data);
};

function send() {
    var command = document.getElementById("command");
    var text = command.value;
    appendLine("You: " + text);
    ws.send(text);
    command.value = "";
}

function clear() {
    console.log("clear()");
    var response = document.getElementById("response");
    response.innerHTML = "";
}

function handleEnter(e){
    if(e.keyCode === 13){
        e.preventDefault(); // Ensure it is only this code that rusn
        send();
    }
}

function appendLine(text) {
    var line = document.createElement("p");
    var content = document.createTextNode(text);
    line.appendChild(content);

    var response = document.getElementById("response");
    response.appendChild(line);
}

var sendButton = document.getElementById("send");
sendButton.addEventListener("click", send, false);

var clearButton = document.getElementById("clear");
clearButton.addEventListener("click", clear, false);

var command = document.getElementById("command");
command.addEventListener("keypress", handleEnter, false);