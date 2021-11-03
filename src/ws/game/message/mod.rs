use serde::Serialize;
use crate::ws::game::state::GameState;
use std::error::Error;
use core::fmt;
use std::fmt::Formatter;

pub mod start;
pub mod ask_question;
pub mod answer;
pub mod validate_answer;
pub mod choose_answerer;

pub type GameMessageResult = Result<GameMessageResponse, GameMessageError>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GameMessageResponse {
    Started,
    /// (answerer_id)
    QuestionAsked(usize),
    /// (answerer_id, answer, question)
    QuestionAnswered(usize, String, String),
    /// (answerer_id, is_answer_valid)
    AnswerValidated(usize, bool),
    /// (new_answerer_id)
    AnswererChosen(usize),
}

#[derive(Serialize, Debug)]
pub struct GameMessageError {
    message: String,
    game_id: usize,
    game_state: Option<GameState>,
}

impl fmt::Display for GameMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "game_id: {}, game_state: {:?}: {}", self.game_id, self.game_state, self.message)
    }
}

impl GameMessageError {
    pub fn with_state(message: &str, game_id: usize, game_state: GameState) -> Self {
        Self {
            game_id,
            game_state: Some(game_state),
            message: message.to_owned(),
        }
    }

    pub fn new(message: &str, game_id: usize) -> Self {
        Self {
            game_id,
            game_state: None,
            message: message.to_owned(),
        }
    }
}

impl Error for GameMessageError {}

// impl From<GameMessageError> for WsError {
//     fn from(game_error: GameMessageError) -> Self {
//         Self::message(&format!("Game error: {}; game_id ({}), answerer_id ({:?}), host_id({:?}", game_error.message, game_error.game_id, game_error.answerer_id, game_error.host_id))
//     }
// }