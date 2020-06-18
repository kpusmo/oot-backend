use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::ws::server::command::game::AnswerTimeout;
use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::Game;
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{AnswerState, ThirdRoundGameState, ValidateAnswerState};

pub struct QuestionAnswered {
    pub contents: String,
}

impl GameTick<QuestionAnswered> for Game {
    fn tick(&mut self, data: QuestionAnswered) -> TickResult {
        let result;
        self.state = match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::Answer(game_state) => {
                        if let Some(sender) = &self.child_thread_sender {
                            sender.send(());
                            self.child_thread_sender = None;
                        }
                        result = TickResult::QuestionAnswered(game_state.answerer_id, data.contents.clone(), game_state.question.clone());
                        ThirdRound(ThirdRoundGameState::ValidateAnswer(ValidateAnswerState {
                            old_answerer_id: game_state.old_answerer_id,
                            answerer_id: game_state.answerer_id,
                            answer: data.contents,
                        }))
                    }
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!(),
        };
        result
    }
}