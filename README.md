# jklm-wordbomb-bot
A bot that plays JKLM.fun's word bomb game. Written in rust


## How it works

JKLM's game passes data via websockets. 

Messages such as:
(recieve)

- "setPlayerWord" - Sets the word for the current player
- "nextTurn" - moves to next player and gives new letters
- "nextRound" - moves to next round
- "correctWord" - tells client that the word was in the dictionary and not already said, updates the bonus letters
- "failWord" - tells client that either the word was "notInDictionary", "mustContainSyllable" or "alreadyUsed"
- "livesLost" - states the lives lost for player turn loss

(send)

todo




## Creating a connection

2 sockets are created. One is for the chat on the side and the other is for the game.

for the game connection

establish websocket and send `40` followed by `{"joinGame", "bombParty", code, token}

