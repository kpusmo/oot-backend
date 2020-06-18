use crate::ws::server::game::event::{GameTick, TickResult};
use crate::ws::server::game::GameState::ThirdRound;
use crate::ws::server::game::state::third_round::{ThirdRoundGameState, AnswerState};
use crate::ws::server::game::Game;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use crate::ws::server::command::game::AnswerTimeout;
use std::thread::sleep;

pub struct AskedQuestion<'a> {
    pub contents: &'a str,
}

impl<'a> GameTick<AskedQuestion<'a>> for Game {
    fn tick(&mut self, data: AskedQuestion) -> TickResult {
        let result;
        self.state = match &self.state {
            ThirdRound(state) => {
                match state {
                    ThirdRoundGameState::AskQuestion(game_state) => {
                        let (tx, rx) = mpsc::channel();
                        self.child_thread_sender = Some(tx);
                        let server_addr = self.server.clone();
                        let game_id = self.id;
                        thread::spawn(move || {
                            sleep(Duration::from_secs(5));
                            match rx.try_recv() {
                                Err(TryRecvError::Empty) => server_addr.do_send(AnswerTimeout {
                                    game_id,
                                }),
                                _ => {}
                            }
                        });
                        result = TickResult::QuestionAsked(game_state.answerer_id);
                        ThirdRound(ThirdRoundGameState::Answer(AnswerState {
                            old_answerer_id: game_state.old_answerer_id,
                            answerer_id: game_state.answerer_id,
                            question: data.contents.to_owned(),
                        }))
                    },
                    _ => unimplemented!()
                }
            },
            _ => unimplemented!(),
        };
        // let thr = thread::spawn(|| {
        //     thread::sleep(Duration::new(5, 0));
        // });
        // thr.
        result
    }
}