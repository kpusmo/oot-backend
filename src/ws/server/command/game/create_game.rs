use actix::{Message, Handler, Context, AsyncContext};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use crate::ws::server::game::{Player, Game, GameState, Host};
use crate::ws::server::GameServer;
use crate::ws::server::game::state::third_round::ThirdRoundGameState;
use crate::ws::server::game::event::{GameTick, Start};
use crate::ws::session::default_id;

/// Response has the same data as command
#[derive(Message, Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
#[rtype(result = "()")]
pub struct CreateGame {
    #[serde(default = "default_id")]
    pub id: usize,
    room: String,
    players: HashSet<usize>,
    host: usize,
}

impl Handler<CreateGame> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: CreateGame, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(room) = self.rooms.get(&msg.room) {
            if !room.members.contains(&msg.host) {
                // todo error
            }
            if room.members.difference(&msg.players).count() != 0 {
                // todo error
            }
            self.game_counter += 1;
            let mut players = HashMap::new();
            for id in &msg.players {
                players.insert(*id, Player {
                    id: *id,
                    points: 0,
                    chances: 3, // todo
                    is_active: true,
                });
            }
            self.games.insert(self.game_counter, Game {
                id: self.game_counter,
                server: ctx.address(),
                state: GameState::ThirdRound(ThirdRoundGameState::Start), // todo choose round
                players,
                room: msg.room.clone(),
                host: Host {
                    id: msg.host,
                },
                child_thread_sender: None,
            });
            self.games.get_mut(&self.game_counter).unwrap().tick(Start {});
            self.send_message(&msg.room, &msg, 0);
        } else {
            // todo error
        }
    }
}