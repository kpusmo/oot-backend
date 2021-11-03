use actix::{ContextFutureSpawner, fut, Handler, Message, WrapFuture, ActorFuture};
use serde::{Deserialize, Serialize};

use crate::ws::bool_deserializer;
use crate::ws::game::message::{GameMessageResponse, GameMessageError};
use crate::ws::game::message::validate_answer::ValidateAnswer as GameValidateAnswer;
use crate::ws::server::GameServer;

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct ValidateAnswer {
    #[serde(default)]
    pub session_id: usize,
    game_id: usize,
    room: String,
    #[serde(deserialize_with = "bool_deserializer")]
    is_answer_valid: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateAnswerResponse {
    answerer_id: usize,
    is_answer_valid: bool,
}

impl Handler<ValidateAnswer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ValidateAnswer, ctx: &mut Self::Context) -> Self::Result {
        if let Some((game, _)) = self.games.get(&msg.game_id) {
            game.send(GameValidateAnswer {
                is_valid: msg.is_answer_valid,
                host_id: msg.session_id,
            })
                .into_actor(self)
                .then(move |res, act, _ctx| {
                    match res.unwrap_or_else(|err| Err(GameMessageError::new(&format!("{:?}", err), msg.game_id))) {
                        Ok(res) => {
                            if let GameMessageResponse::AnswerValidated(answerer_id, is_answer_valid) = res {
                                act.send_message(&msg.room, &ValidateAnswerResponse {
                                    answerer_id,
                                    is_answer_valid,
                                }, 0);
                            } else {
                                act.fail_to(msg.session_id, &GameMessageError::new(&format!("expected GameMessageResult::AnswerValidated, received {:?}", res), msg.game_id));
                            }
                        },
                        Err(err) => act.fail_to(msg.session_id, &err)
                    }
                    fut::ready(())
                })
                .wait(ctx);
        } else {
            self.fail_to(msg.session_id, &GameMessageError::new("Invalid game_id", msg.game_id));
        }
    }
}