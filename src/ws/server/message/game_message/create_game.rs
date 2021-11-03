use std::collections::HashSet;

use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture, fut, ContextFutureSpawner, ActorFuture};
use serde::{Deserialize, Serialize};

use crate::ws::server::GameServer;
use crate::ws::game::Game;
use crate::ws::game::message::start::Start;
use crate::ws::game::message::GameMessageError;
use crate::ws::error::WsError;

#[derive(Message, Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
#[rtype(result = "()")]
pub struct CreateGame {
    #[serde(default)]
    pub session_id: usize,
    room: String,
    players: HashSet<usize>,
    host: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCreatedResponse {
    pub game_id: usize,
    room: String,
    players: HashSet<usize>,
    host: usize,
}

impl Into<GameCreatedResponse> for CreateGame {
    fn into(self) -> GameCreatedResponse {
        GameCreatedResponse {
            room: self.room,
            players: self.players,
            host: self.host,
            game_id: 0,
        }
    }
}

impl Handler<CreateGame> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: CreateGame, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(room) = self.rooms.get_mut(&msg.room) {
            if !room.members.contains(&msg.host) {
                self.fail_to(msg.session_id, &WsError::message(&format!("Host ({}) is not in the room {}", msg.host, msg.room)));
                return;
            }
            if msg.players.difference(&room.members).count() != 0 {
                self.fail_to(msg.session_id, &WsError::message(&format!("Some of given players do not belong to the room {}", msg.room)));
                return;
            }
            self.game_counter += 1;
            let game_id = self.game_counter;
            room.game_ids.insert(game_id);
            let game_addr = Game::new(game_id, &msg.room, ctx.address(), &msg.players, msg.host).start();
            self.games.insert(game_id, (game_addr.clone(), msg.room.clone()));
            game_addr.send(Start {})
                .into_actor(self)
                .then(move |res, act, _ctx| {
                    match res.unwrap_or_else(|err| Err(GameMessageError::new(&format!("{:?}", err), game_id))) {
                        Ok(_res) => {
                            let mut response: GameCreatedResponse = msg.into();
                            response.game_id = act.game_counter;
                            act.send_message(&response.room, &response, 0);
                        },
                        Err(err) => act.fail_to(msg.session_id, &err)
                    }
                    fut::ready(())
                })
                .wait(ctx);
        } else {
            self.fail_to(msg.session_id, &WsError::message(&format!("Invalid room name {}", msg.room)));
        }
    }
}
