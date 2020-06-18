use std::collections::HashMap;

use actix::Addr;
use crate::ws::server::game::state::third_round::{ThirdRoundGameState, AskQuestionState, AnswerState, ChooseAnswererState};
use crate::ws::server::GameServer;
pub use crate::ws::server::game::player::*;
pub use crate::ws::server::game::host::*;
use crate::ws::server::game::GameState::ThirdRound;
use rand::seq::SliceRandom;
use std::sync::mpsc;

mod player;
mod host;
pub mod event;
pub mod state;

#[derive(Debug)]
pub enum GameState {
    ThirdRound(ThirdRoundGameState),
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub players: HashMap<usize, Player>,
    pub host: Host,
    pub state: GameState,
    pub server: Addr<GameServer>,
    pub room: String,
    pub child_thread_sender: Option<mpsc::Sender<()>>,
}

impl Game {
    pub fn get_answerer_id(&self) -> Option<usize> {
        match &self.state {
            GameState::ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::AskQuestion(AskQuestionState { old_answerer_id, answerer_id })
                    | ThirdRoundGameState::Answer(AnswerState { old_answerer_id, answerer_id, question: _ })
                    | ThirdRoundGameState::ChooseAnswerer(ChooseAnswererState { old_answerer_id, chooser_id: answerer_id }) => Some(*answerer_id),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn get_random_player_id(&self) -> usize {
        let player_ids: Vec<&usize> = self.players.keys().collect();
        let answerer_id = **player_ids.choose(&mut rand::thread_rng()).unwrap();
        // self.state = ThirdRound(ThirdRoundGameState::AskQuestion(answerer_id));
        answerer_id
    }

    fn handle_answer(&mut self, is_valid: bool, answerer_id: usize) {
        if let Some(player) = self.players.get_mut(&answerer_id) {
            if is_valid {
                player.points += 10; // todo how many points, finish game (rules)
            } else {
                player.chances -= 1;
                if player.chances == 0 {
                    player.is_active = false;
                }
            }
        } else {
            panic!("the disco");
        }
    }
}