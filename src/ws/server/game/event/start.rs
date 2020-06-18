use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::Game;
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{ThirdRoundGameState, AskQuestionState};
use rand::seq::SliceRandom;

pub struct Start;

impl GameTick<Start> for Game {
    fn tick(&mut self, _data: Start) -> TickResult {
        let result;
        self.state = match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::Start => {
                        result = TickResult::Started;
                        let new_answerer_id = self.get_random_player_id();
                        ThirdRound(ThirdRoundGameState::AskQuestion(AskQuestionState {
                            old_answerer_id: None,
                            answerer_id: new_answerer_id,
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