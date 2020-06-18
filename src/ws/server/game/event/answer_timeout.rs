use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::Game;
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{ChooseAnswererState, ThirdRoundGameState, AskQuestionState};
use crate::ws::server::game::state::third_round::ThirdRoundGameState::{ChooseAnswerer, AskQuestion};

pub struct AnswerTimeout;

impl GameTick<AnswerTimeout> for Game {
    fn tick(&mut self, _data: AnswerTimeout) -> TickResult {
        println!("answer timeout event");
        let new_answerer_id;
        let new_state;
        match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::Answer(game_state) => {
                        let old_answerer_id = game_state.old_answerer_id;
                        let answerer_id = game_state.answerer_id;
                        self.handle_answer(false, answerer_id);
                        match old_answerer_id {
                            Some(answerer_id) => {
                                new_answerer_id = Some(answerer_id);
                                new_state = "ChooseAnswerer".to_owned();
                                self.state = ThirdRound(ChooseAnswerer(ChooseAnswererState {
                                    old_answerer_id,
                                    chooser_id: answerer_id,
                                }))
                            }
                            None => {
                                new_answerer_id = Some(self.get_random_player_id());
                                new_state = "AskQuestion".to_owned();
                                self.state = ThirdRound(AskQuestion(AskQuestionState {
                                    old_answerer_id: None,
                                    answerer_id,
                                }))
                            }
                        }
                    }
                    _ => {
                        // do nothing for other states - suppose answerer answered question anyway and we did not send signal to thread handling timeout
                        new_answerer_id = None;
                        new_state = "".to_owned();
                    }
                }
            }
            _ => unimplemented!()
        };
        TickResult::AnswerTimeout(new_answerer_id, new_state)
    }
}