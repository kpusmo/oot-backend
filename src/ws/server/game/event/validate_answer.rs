use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::Game;
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{ThirdRoundGameState, ChooseAnswererState};

pub struct AnswerValidated {
    pub is_valid: bool,
}

impl GameTick<AnswerValidated> for Game {
    fn tick(&mut self, data: AnswerValidated) -> TickResult {
        let result;
        self.state = match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::ValidateAnswer(game_state) => {
                        let answerer_id = game_state.answerer_id;
                        let old_answerer_id = game_state.old_answerer_id;
                        self.handle_answer(data.is_valid, game_state.answerer_id);
                        result = TickResult::AnswerValidated(data.is_valid, answerer_id);
                        ThirdRound(ThirdRoundGameState::ChooseAnswerer(ChooseAnswererState {
                            old_answerer_id,
                            chooser_id: answerer_id,
                        }))
                    },
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!(),
        };
        result
    }
}