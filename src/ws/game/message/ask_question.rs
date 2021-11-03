use actix::{Handler, Context, Message, AsyncContext};
use crate::ws::game::Game;
use crate::ws::game::message::{GameMessageResponse, GameMessageResult, GameMessageError};
use crate::ws::game::state::{GameState, AnswerState};

pub struct AskQuestion {
    pub contents: String,
    pub host_id: usize,
}

impl Message for AskQuestion {
    type Result = GameMessageResult;
}

impl Handler<AskQuestion> for Game {
    type Result = GameMessageResult;

    fn handle(&mut self, msg: AskQuestion, ctx: &mut Context<Self>) -> Self::Result {
        self.verify_host_id(msg.host_id, "AskQuestion")?;
        match &self.state {
            GameState::AskQuestion(game_state) => {
                let answerer_id = game_state.answerer_id;
                self.state = GameState::Answer(AnswerState {
                    answerer_id,
                    previous_answerer_id: game_state.previous_answerer_id,
                    question: msg.contents.to_owned(),
                });
                self.timeout_spawn_handle = Some(ctx.run_later(self.config.timeout, |game, _ctx| game.handle_timeout()));
                Ok(GameMessageResponse::QuestionAsked(answerer_id))
            },
            state => Err(GameMessageError::with_state(&format!("Received AskQuestion message, but the game state is {:?}", state), self.id, state.to_owned()))
        }
    }
}
