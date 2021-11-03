use actix::{Message, Handler};
use crate::ws::game::message::{GameMessageResult, GameMessageResponse, GameMessageError};
use crate::ws::game::Game;
use crate::ws::game::state::{GameState, AskQuestionState};

pub struct ChooseAnswerer {
    pub new_answerer_id: usize,
    pub chooser_id: usize
}

impl Message for ChooseAnswerer {
    type Result = GameMessageResult;
}

impl Handler<ChooseAnswerer> for Game {
    type Result = GameMessageResult;

    fn handle(&mut self, msg: ChooseAnswerer, _ctx: &mut Self::Context) -> Self::Result {
        self.verify_answerer_id(msg.chooser_id, "ChooseAnswerer")?;
        match &self.state {
            GameState::ChooseAnswerer(_game_state) => {
                self.state = GameState::AskQuestion(AskQuestionState {
                    answerer_id: msg.new_answerer_id,
                    previous_answerer_id: Some(msg.chooser_id)
                });
                Ok(GameMessageResponse::AnswererChosen(msg.new_answerer_id))
            }
            state => Err(GameMessageError::with_state(&format!("Received ChooseAnswerer message, but the game state is {:?}", state), self.id, state.to_owned())),
        }
    }
}