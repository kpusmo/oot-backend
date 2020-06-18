use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};

use crate::ws::session::default_id;
use crate::ws::bool_deserializer;
use crate::ws::server::game::event::{TickResult, AnswerValidated, GameTick};
use crate::ws::server::GameServer;
use std::ops::Deref;
use crate::ws::server::game::{Player, Game};

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ValidateAnswer {
    #[serde(default = "default_id")]
    pub id: usize,
    game_id: usize,
    room: String,
    #[serde(deserialize_with = "bool_deserializer")]
    is_answer_valid: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateAnswerResponse<'a> {
    player: &'a Player,
    answer_valid: bool,
}

impl Handler<ValidateAnswer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ValidateAnswer, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(mut game) = self.games.remove(&msg.game_id) {
            if game.host.id != msg.id {
                todo!("error")
            }
            let response;
            match game.tick(AnswerValidated {
                is_valid: msg.is_answer_valid,
            }) {
                TickResult::AnswerValidated(answer_valid, answerer_id) => {
                    let player = game.players.get(&answerer_id).unwrap();
                    response = Some(ValidateAnswerResponse {
                        player,
                        answer_valid,
                    });
                    self.send_message(&msg.room, &response, 0);
                    self.games.insert(msg.game_id, game);
                }
                _ => unimplemented!(),
            }
        } else {}
    }
}