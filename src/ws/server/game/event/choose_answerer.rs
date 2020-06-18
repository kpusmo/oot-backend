use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::Game;
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{AskQuestionState, ThirdRoundGameState};

pub struct AnswererChosen {
    pub answerer_id: usize,
}

impl GameTick<AnswererChosen> for Game {
    fn tick(&mut self, data: AnswererChosen) -> TickResult {
        let result;
        self.state = match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::ChooseAnswerer(game_state) => {
                        result = TickResult::AnswererChosen(data.answerer_id);
                        ThirdRound(ThirdRoundGameState::AskQuestion(AskQuestionState {
                            old_answerer_id: Some(game_state.chooser_id),
                            answerer_id: data.answerer_id,
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