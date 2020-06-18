# OOT
This is the backend-side of a _Jeden z Dziesięciu_-like game app. There is currently no frontend-side of it.

# Run
There is a [traefik](https://hub.docker.com/_/traefik) configuration commented out in `docker-compose.yml`.
If you have `traefik` configured on your system, you can uncomment and customize that configuration. Port `80` in the 
`oot-back` container is mapped to the port `3001` on the host.

```bash
docker-compose up
```

# API
The game server accepts WebSocket connections on `/ws/{name}` route, where `name` is the name of connected user.
The server interacts with the client through the commands listed below. The command are WebSocket messages containing 
the command name followed by the json encoded data.

## Server structure
### Users
The server accepts WebSocket connections and stores them as the users with unique ids assigned.

### Rooms
Users can create and join rooms by the following command:
```
join {"room": "room name"}
```
If the room of given name does not exist, it is created. Users can send chat messages to other room members and 
create games, by specifying the Game Master and players by their ids.

### Game
Game consists of the Game Master and the players. Game Master is responsible for asking the questions to the players and
for validating their answers. A game is created in the room by its members and only the room members can 
participate in the game. There is always one Game Master in a game. A Game Master cannot be a player.

### Flow
Users join the room. One of them creates the game with players and Game Master specified. The players are created with
0 points and 3 chances (ToDo - should be configurable). The game randomly chooses the first answerer.
The Game Master asks a question to the answerer, the answerer answers it. Answerer has 5 seconds to answer the question.
The Game Master validates the answer. The answerer chooses the next answerer (ToDo - only if the answer was correct).

## Commands
### Join/Create room
```
join {
    "room": string
}
```

### Send message to room members
```
message {
    "room": string,
    "contents": string
}
```

### Create game
```
create_game {
    "room": string,
    "players": int[],
    "host": int
}
```
`players` is an array of player ids, `host` is the id of the Game Master. The first answerer is chosen randomly.

ToDo: `room` is redundant in the following commands, as they contain `game_id`.
### Ask question
Can only be run by the Game Master.
```
ask_question {
    "room": string,
    "game_id": int,
    "contents": string
}
```

### Answer the question
Can only be run by the current answerer.
```
answer {
    "room": string,
    "game_id": int,
    "contents": string
}
```

### Answer validation
Can only be run by the Game Master.
```
validate_answer {
    game_id: int,
    room: string,
    is_answer_valid: boolean
}
```

### Choose answerer
Can only be run by the current answerer.
```
choose_answerer {
    game_id: int,
    room: string,
    new_answerer_id: int,
}
```

# ToDo's
* Client side of application.
* Server responses must contain the type of response, so the client can recognize what was the command the response is for.
* End game conditions.
* The `Game` structure refactor. It probably should implement the `Actor` interface. The game state should not be kept 
in the game mode.
* Organize the way the server interacts with the games. It probably would be helpful if the `Game` class implemented the
`Actor` interface.
* Validations, `unimplemented!`s and `todo!`s
* Some tests for Christ's sake.

# Client
For testing purposes `client.py` was kept in the project. It allows to connect to the server and send commands in the 
primitive way - for example, it has ids of the players and the Game Master hardcoded. However, it allows to play the game
if 3 users are in the room. You can run the client with
```
python client.py --host 127.0.0.1 --port 3001
``` 
Here are examples of invoking the commands in the `client.py` script:
```
# join `test-room` room
join test-room

# message to other `test-room` room members
message test-room Hello members

# create a game, game master's id and player ids are hardcoded to 1, [2, 3]
create_game test-room

# ask a question, run by the game master
ask_question test-room Surely you are not serious.

# answer the question, run by the answerer
answer test I am serious, and don't call me Shirley.

# validate the answer, run by the game master; valid values are {1|0|true|false}
validate_answer test-room 1

# choose a answerer by id, run by the previous answerer
choose_answerer test-room 3
```