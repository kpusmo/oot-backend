use actix::{Message, Handler};
use crate::ws::game::Game;
use crate::ws::game::state::{GameState, ChooseAnswererState, AskQuestionState};
use crate::ws::game::message::{GameMessageResponse, GameMessageResult, GameMessageError};
use crate::ws::server::message::game_message::answer_timeout::AnswerTimedOut;
use crate::ws::server::message::game_message::player_lost::PlayerLost;

pub struct ValidateAnswer {
    pub is_valid: bool,
    pub host_id: usize,
}

impl Message for ValidateAnswer {
    type Result = GameMessageResult;
}

impl Handler<ValidateAnswer> for Game {
    type Result = GameMessageResult;

    fn handle(&mut self, msg: ValidateAnswer, _ctx: &mut Self::Context) -> Self::Result {
        self.verify_host_id(msg.host_id, "ValidateAnswer")?;
        match &self.state {
            GameState::ValidateAnswer(game_state) => {
                let answerer_id = game_state.answerer_id;
                let maybe_previous_answerer_id = game_state.previous_answerer_id;
                self.handle_answer(msg.is_valid, answerer_id);
                let chooser_id = if msg.is_valid {
                    answerer_id
                } else if let Some(previous_answerer_id) = maybe_previous_answerer_id {
                    previous_answerer_id
                } else {
                    let answerer_id = self.get_random_player_id();
                    self.state = GameState::AskQuestion(AskQuestionState {
                        answerer_id,
                        previous_answerer_id: None,
                    });
                    return Ok(GameMessageResponse::AnswerValidated(answerer_id, msg.is_valid))
                };
                self.state = GameState::ChooseAnswerer(ChooseAnswererState {
                    previous_answerer_id: Some(answerer_id),
                    chooser_id,
                });
                Ok(GameMessageResponse::AnswerValidated(answerer_id, msg.is_valid))
            },
            state => Err(GameMessageError::with_state(&format!("Received ValidateAnswer message, but the game state is {:?}", state), self.id, state.to_owned())),
        }
    }
}

impl Game {
    fn handle_answer(&mut self, is_valid: bool, answerer_id: usize) {
        if let Some(player) = self.players.get_mut(&answerer_id) {
            if is_valid {
                player.points += self.config.valid_answer_reward;
            } else {
                player.chances -= 1;
                if player.chances == 0 {
                    player.is_active = false;
                    self.server.do_send(PlayerLost {
                        game_id: self.id,
                        player_id: player.session_id,
                        points: player.points,
                    });
                }
            }
        } else {
            unimplemented!("jak do tego doszło - nie wiem")
        }
    }


    pub fn handle_timeout(&mut self) {
        println!("Game::handle_timeout, {:?}", self.state);
        match &self.state {
            GameState::Answer(game_state) => {
                let maybe_chooser_id = game_state.previous_answerer_id;
                let answerer_id = game_state.answerer_id;
                self.handle_answer(false, answerer_id);
                if let Some(chooser_id) = maybe_chooser_id {
                    self.state = GameState::ChooseAnswerer(ChooseAnswererState {
                        previous_answerer_id: Some(chooser_id),
                        chooser_id,
                    });
                    self.server.do_send(AnswerTimedOut {
                        new_state: "ChooseAnswerer".to_owned(),
                        game_id: self.id,
                    });
                } else {
                    let answerer_id = self.get_random_player_id();
                    self.state = GameState::AskQuestion(AskQuestionState {
                        previous_answerer_id: None,
                        answerer_id,
                    });
                    self.server.do_send(AnswerTimedOut {
                        new_state: "AskQuestion".to_owned(),
                        game_id: self.id,
                    });
                }
            },
            // do nothing for other states - suppose answerer answered question anyway, but a future didn't receive cancel signal/got it too late
            _ => {}
        }
    }
}