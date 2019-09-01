[![THRUSTIN](https://raw.githubusercontent.com/maxrchung/THRUSTIN/master/logo/THRUSTIN-3000.png)](https://THRUSTIN.rs)

# The Purpose
Okay. This was originally a wiki reference for all the commands but I'm just gonna move it to the front yeah. Yeah THRUSTIN is like an online Cards Against Humanity fill in the blank game that's like in the form of a console and you type and give a bunch of commands to it in order to interact with it. I think this is really hopefully gonna be a great way to layout what we have designed and will have been designing for the past and future. You can also use this as a guide or tutorial or instruction manual for all the commands and their uses. You can also use this as a guide or tutorial if you wish to create your own client to connect to the THRUSTIN.rs server. Hit us up with a web socket connection on port 3011.

Wait I don't really know why I'm doing this. We already have `.help` commands in the game. I don't know.

# Some Points of Interests
There's probably gonna be some confusion around some things. Let's explain things of that nature and possibly confuse you further.

## THRUSTEE vs THRUSTER vs THRUST What Is The Difference?
What is the difference? Let's think of this in terms of Cards Against Humanity cards. In Cards Against Humanity the card game of which is the basis of this game, Black cards have underscores that White cards can be inserted into. Let's do an example. Black card: I'm so f~~uck~~ing ____________________. White card: Cool. When put together, you create I'm so f~~uck~~ing <u>Cool.</u>. In THRUSTIN, everything is about THRUSTING in the blank, AKA THRUST in the blank, AKA THRUST in, AKA THRUSTin AKA THRUSTIN. A THRUSTEE is a Black card, AKA I'm so f---ing _____. A THRUSTER is a White card, AKA cool. The act of putting the THRUSTER into the THRUSTEE is called a THRUST. QED when you THRUST cool into I'm so f---ing _____. that forms I'm so f---ing <u>cool</u>. This is slightly a bit more complicated because in our terminology we also call the person who is choosing a THRUSTEE also as the THRUSTEE and the person who is choosing a THRUSTER also as the THRUSTER also because our English is completely brokened.

Also THRUST could also mean adding to your own personal THRUSTEE and THRUSTER collections.




Yeah when you get selected as the best THRUSTER, you get a point and if you get the max points you win.

Yeah anything with THRUST in it has to be capitalized.

I'm sorry about the rude language, like in general, really, I don't know what's wrong with me.

## Your THRUSTS ##
Each person can add their own custom THRUSTEE and THRUSTERS. When a game is started, their personal THRUSTS are added into the collective whole to be played around with. There is a default set of cards, we call this the house cards, that a lobby by default will use by default. You can toggle this off using `.house`.

## Aliases
Oh yeah here's a point to keep note. Aliases can be used to shorten commands, as long if there are no duplicates. AKA if you have a command like `.help` then you can just use `.h` to shorten it down. However, there are cases where this would not make sense, can you think of a case? Well, this can happen if you have another command that starts with an `h`. AKA if you have `.help` and `.house`, we have some weird rules for that where I think most likely in this case `.house` gets shortened down to `.ho`. (<- Ignore this last period it's just part of English.) Okay I decided I'm going to show the alias separation using parentheses next to the command, e.g. `.h(elp)`.

## Overloading
Some commands are overloaded, meaning putting 0, 1, 2 arguments for a command could lead to different things. I'll try and show this too using (parentheses). There are gonna be repeated commands too since some commands are the same across different phases. I guess I'll just copy paste something for that.

## Chief(tain)
Chief is the host of a lobby. We have a micromeme where we say "Chief is callin'" periodically, and this is an attempt to link our life into this game so we took the chance and replaced references of lobby host with Chief. Respect.

We also have Chieftains. Yeah, I realize this is a bit confusing, but I think it's kind of cool. Chieftains are global admins of the server and can do Chief commands on top of a few other things.

## Endless Lobby
We have a thing called an endless lobby that anyone can more easily join and play in. This lobby kind of hangs around mostly forever, until someone gets 256 points, and anyone is pretty much free to join and play unless 2 billion people join.

## They Are Not Case Sensitive
You can do `.HELP` or `.help` it doesn't matter. We keep the `.` though, for command. We are gonna support chat so we want some differentiation, and `.` is fairly easier to type on mobile. Do we really care about mobile though? This whole thing is not very mobile friendly or friendly at all.

## How Bout That We Finally Got To Making Accounts For The Database (MongoDB)
Yeah.

We're gonna start using the database for some nice things. Once you register an account with a username and password, you'll save some things on next login. What we're gonna do: saved THRUSTEES and THRUSTERS you've added; game stats (milestone 4.20), admin accounts (milestone 69), banning (milestone -69).

## Technological Stack

Some technologies we using? HTML, CSS, SASS, JavaScript, Bootstrap, React, webpack, web sockets, Rust, MongoDB, NGINX, Let's Encrypt, Git, TortoiseGit, SourceTree, GitHub, VSCode, emacs, Trello, brain

# Phases
Now let us dive into all the commands you can use. We shall divide each section up by the respective phase they can be used in then go from there. I don't know how much detail I'm going to put into explaining each command's usage. This might be pretty long so I'm not sure. I'm not sure man.

## Name
In this phase you're responsible for giving yourself a name. We want you to give yourself a name before you get into game.

##### `.h(elp)`
Get help explanation for this section.

##### `.n(ame) SWAGGYSWAG`
Give yourself the name SWAGGYSWAG. Note that you probably shouldn't put spaces in there I'm not sure what's gonna happen probably bad.

##### `.l(ogin) SecretUser SecretPassword!!!!`
Login to a registered account. Yo you get some epic perks for logging in...

##### `.r(egister) SecretUser SecretPassword!!!! SecretPassword!!!!`
Register an account to get some sick as hell features like saved THRUSTS and stats (eventually).

## Out of Lobby
The zone outside of lobbies where you can chill and organize yourself.

##### `.h(elp)`
[See](#help)

##### `.j(oin) 1`
Join lobby 1.

##### `.l(ist)`
Show list of all lobies.

##### `.m(ake) (super_________SECRET___________P@$$WoRD`
Make a new lobby. Optionally, provide a password for your new lobby.

##### `.n(ame) LEETMAKERiamTHIS`
Change your name to LEETMAKERiamTHIS.

##### `.p(lay)`
Join the endless lobby.

##### `.w(ho)`
Shows everyone out and in lobbies.

##### `.a(ccount)`
If logged in, view account information and game stats that are relevant to your account, of course.

##### `.c(olor) 000 ffd1dc`
Set your chat colors. Must be hexadecimal colors. Must be 3 or 6 characters. Here, 000 (black) is set to the background color and ffd1dc (pastel faded out dull pinkish color) is set to the text/foreground color.

##### `.T(HRUST) ("I'm going to f_____unk a THRUSTEE" "I'm going to add a THRUSTER" "I'm going to add another THRUSTER")`
If logged in, if you do .THRUST by itself, it'll show all your current THRUSTS. If you add quoted arguments, THRUSTEES and THRUSTERS  are added into your personal pile.

##### `.U(NTHRUST)`
If logged in, destroy all your personal THRUSTS lol.

##### `.u(ser)n(ame) SomeOtherUserName__ SomeOtherUserName__`
If logged in, changes your username to SomeOtherUserName__. It's kind of weird the more I think about it how Username and Name are so closely related. I could have done something to make them closer to the same, but in the end I decided not to. I personally find the use case of just logging in with a standard User/Pass account, then making up some random Name to troll with, quite appealing so yeah that's that.

##### `.p(ass)w(ord) Y0L0Ep1C420420420 Y0L0Ep1C420420420`
If logged in, changes your password to Y0L0Ep1C420420420, so you know, if you're a hacker you can screw the hacked guys' account over lol.

##### `.b(an) (69.69.69.69)`
(Chieftain-only) Bans an IP address from a server. If no argument is provided, a list of banned IP addresses will become listed to the chieftain that produced this command.

##### `.u(n)b(an) (69.69.69.69)`
(Chieftain-only) Unbans the given IP address from the THRUSTIN server.

##### `.c(hief)t(ain) (NewAdministratorHere)`
(Chieftain-only) By itself, `.chieftain` shall show a list of all chieftains in the server. With an argument, you will attempt to appoint the targetted Name as a Chieftain. Note that we use `.ct` because `.c` is for `.chief` and damn I'm no so sure if I want to make these too closely related anymore, but whatever.

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
(Chieftain-only) Removes the selected Chieftain as a chieftain.

## In Lobby
Commands when you are in a lobby. Note that now you'll see some (Chief-Only) commands that only chiefs may do.

##### `.h(elp)`
[See](#help)

##### `.i(nfo)`
Shows info about this lobby.

##### `.l(eave)`
Leave this lobby.

##### `.w(ho)`
Shows everyone in this lobby.

##### `.c(hief) TheOtherSwagger`
(Chief-Only) Set TheOtherSwagger as the chief.

##### `.ho(use) 69`
(Chief-Only) Set how many house THRUSTS to use. We divide the number in half to split for THRUSTEES and THRUSTERS. Do `.ho 0` for zero house THRUSTS.

##### `.k(ick) HeyMan`
(Chief-Only) Kick HeyMan from the lobby.

##### `.p(ass)w(ord) YOOOOoooo0000`
(Chief-Only) Set YOOOOoooo0000 as the password for the lobby.

##### `.pl(ayers) 3`
(Chief-Only) Set the max amount of players to 3.

##### `.po(ints) 3`
(Chief-Only) Set the points max to 3.

##### `.s(tart)`
(Chief-Only) Start the game barring there's no issues.

##### `.(THRUSTE)E(S) 3`
(Chief-Only) Set the number of THRUSTEE choices to 3.

##### `.(THRUSTE)R(S) 3`
(Chief-Only) Set the number of THRUSTERS you can have to 3.

##### `.a(ccount)`
[See](#account)

##### `.T(HRUST) ("yeah _____")`
[See](#thrust-im-going-to-f_____unk-a-thrustee-im-going-to-add-a-thruster-im-going-to-add-another-thruster)

##### `.U(NTHRUST)`
[See](#unthrust)

##### `.b(an) (69.69.69.69)`
[See](#ban-69696969)

##### `.u(n)b(an) (69.69.69.69)`
[See](#unban-69696969)

##### `.c(hief)t(ain) (NewAdministratorHere)`
[See](#chieftain-newadministratorhere)

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
[See](#unchieftain-newadministratornolonger)

## In Game (Choosing THRUSTEE)
When you are the THRUSTEE choosing a THRUSTEE for THRUSTERS to THRUST into.

##### `.h(elp)`
[See](#help)

##### `.i(nfo)`
[See](#info)

##### `.l(eave)`
[See](#leave)

##### `.T(HRUST) 1`
Choose the THRUSTEE at index 1 to use.

##### `.w(ho)`
Show who's got how many points in the lobby and also who's in this lobby yo.

##### `.e(nd)`
(Chief-Only) End the game.

##### `.k(ick) YoloDWAG`
[See](#kick-heyman)

##### `.a(ccount)`
[See](#account)

##### `.c(hief)t(ain)(NewAdministratorHere)`
[See](#chieftain)

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
[See](#unchieftain)

## In Game (Waiting for THRUSTEE)
When you are a THRUSTER waiting for THRUSTEE to be chosen.

##### `.h(elp)`
[See](#help)

##### `.i(nfo)`
[See](#info)

##### `.l(eave)`
[See](#leave)

##### `.w(ho)`
[See](#who-2)

##### `.e(nd)`
[See](#end)

##### `.k(ick) YoloDWAG`
[See](#kick-heyman)

##### `.a(ccount)`
[See](#account)

##### `.b(an) (69.69.69.69)`
[See](#ban-69696969)

##### `.u(n)b(an) (69.69.69.69)`
[See](#unban-69696969)

##### `.c(hief)t(ain) (NewAdministratorHere)`
[See](#chieftain-newadministratorhere)

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
[See](#unchieftain-newadministratornolonger)

## In Game (THRUSTING into chosen THRUSTEE)
When a THRUSTEE is chosen and you are a THRUSTER ready to THRUST one of your THRUSTERS into the THRUSTEE.

##### `.h(elp)`
[See](#help)

##### `.i(nfo)`
[See](#info)

##### `.l(eave)`
[See](#leave)

##### `.T(HRUST) 1`
Choose to use your THRUSTER at index 1 to THRUST into the chosen THRUSTEE. After THRUSTING, you cannot THRUST again. Note that for a THRUSTEE that requires multiple THRUSTERS, you can THRUST the same THRUSTER into it multiple times. I wanted to do this so that you can have more options in the digital era of video games where you are not reliant on physical cards and can have possibilities such as this one.

##### `.w(ho)`
[See](#who-2)

##### `.e(nd)`
[See](#end)

##### `.k(ick) YoloDWAG`
[See](#kick-heyman)

##### `.a(ccount)`
[See](#account)

##### `.b(an) (69.69.69.69)`
[See](#ban-69696969)

##### `.u(n)b(an) (69.69.69.69)`
[See](#unban-69696969)

##### `.c(hief)t(ain) (NewAdministratorHere)`
[See](#chieftain-newadministratorhere)

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
[See](#unchieftain-newadministratornolonger)

## In Game (Deciding best THRUSTER)
When you are the THRUSTEE and ready to pick your favorite THRUST.

##### `.h(elp)`
[See](#help)

##### `.i(nfo)`
[See](#info)

##### `.l(eave)`
[See](#leave)

##### `.T(HRUST) 1`
Okay, you select the completed THRUST at index 1 as the best one.

##### `.w(ho)`
[See](#who-2)

##### `.e(nd)`
[See](#end)

##### `.k(ick) YoloDWAG`
[See](#kick-heyman)

##### `.a(ccount)`
[See](#account)

##### `.b(an) (69.69.69.69)`
[See](#ban-69696969)

##### `.u(n)b(an) (69.69.69.69)`
[See](#unban-69696969)

##### `.c(hief)t(ain) (NewAdministratorHere)`
[See](#chieftain-newadministratorhere)

##### `.u(n)c(hieftain) NewAdministratorNoLonger`
[See](#unchieftain-newadministratornolonger)
