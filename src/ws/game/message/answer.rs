use actix::{AsyncContext, Handler, Message};

use crate::ws::game::Game;
use crate::ws::game::message::{GameMessageResponse, GameMessageResult, GameMessageError};
use crate::ws::game::state::{GameState, ValidateAnswerState};

pub struct AnswerQuestion {
    pub contents: String,
    pub answerer_id: usize,
}

impl Message for AnswerQuestion {
    type Result = GameMessageResult;
}

impl Handler<AnswerQuestion> for Game {
    type Result = GameMessageResult;

    fn handle(&mut self, msg: AnswerQuestion, ctx: &mut Self::Context) -> Self::Result {
        self.verify_answerer_id(msg.answerer_id, "AnswerQuestion")?;
        match &self.state {
            GameState::Answer(game_state) => {
                if let Some(spawn_handle) = self.timeout_spawn_handle {
                    ctx.cancel_future(spawn_handle);
                }
                let question = game_state.question.to_owned();
                self.state = GameState::ValidateAnswer(ValidateAnswerState {
                    answerer_id: msg.answerer_id,
                    previous_answerer_id: game_state.previous_answerer_id,
                    answer: msg.contents.clone(),
                });
                Ok(GameMessageResponse::QuestionAnswered(msg.answerer_id, msg.contents, question))
            }
            state => Err(GameMessageError::with_state(&format!("Received AnswerQuestion message, but the game state is {:?}", state), self.id, state.to_owned()))
        }
    }
}