use actix::{Context, Handler, Message};
use serde::Serialize;
use crate::ws::server::GameServer;
use crate::ws::server::game::event::{AnswerTimeout as GameAnswerTimeoutEvent, GameTick, TickResult};
use crate::ws::server::game::GameState;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct AnswerTimeout {
    pub game_id: usize,
}

/// Message sent to all players in game
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AnswerTimeoutResponse<'a> {
    game_id: usize,
    room: &'a str,
    chosen_id: usize,
    state: &'a str,
}

impl Handler<AnswerTimeout> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: AnswerTimeout, ctx: &mut Context<Self>) -> Self::Result {
        println!("answer timeout command");
        if let Some(game) = self.games.get_mut(&msg.game_id) {
            match game.tick(GameAnswerTimeoutEvent {}) {
                TickResult::AnswerTimeout(maybe_answerer_id, state) => {
                    if let Some(answerer_id) = maybe_answerer_id {
                        let room = game.room.clone();
                        self.send_message(&room, &AnswerTimeoutResponse {
                            game_id: msg.game_id,
                            room: &room,
                            chosen_id: answerer_id,
                            state: &state,
                        }, 0);
                    }
                },
                _ => {}
            }
        }
    }
}