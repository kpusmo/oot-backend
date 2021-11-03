use actix::{Context, Handler, Message};

use crate::ws::game::Game;
use crate::ws::game::state::{AskQuestionState, GameState};
use crate::ws::game::message::{GameMessageResult, GameMessageResponse, GameMessageError};

pub struct Start;

impl Message for Start {
    type Result = GameMessageResult;
}

impl Handler<Start> for Game {
    type Result = GameMessageResult;

    fn handle(&mut self, _: Start, _: &mut Context<Self>) -> Self::Result {
        match &self.state {
            GameState::Starting => {
                let new_answerer_id = self.get_random_player_id();
                self.state = GameState::AskQuestion(AskQuestionState {
                    previous_answerer_id: None,
                    answerer_id: new_answerer_id,
                });
                Ok(GameMessageResponse::Started)
            },
            state => Err(GameMessageError::with_state(&format!("Received Start message, but the game state is {:?}", state), self.id, state.to_owned()))
        }
    }
}