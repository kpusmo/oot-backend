pub use start::*;
pub use ask_question::*;
pub use answer::*;
pub use validate_answer::*;
pub use choose_answerer::*;
pub use answer_timeout::*;
use crate::ws::server::game::Player;

mod start;
mod ask_question;
mod answer;
mod validate_answer;
mod choose_answerer;
mod answer_timeout;

pub enum TickResult {
    Started,
    QuestionAsked(usize),
    QuestionAnswered(usize, String, String),
    AnswerValidated(bool, usize),
    AnswererChosen(usize),
    AnswerTimeout(Option<usize>, String),
}

pub trait GameTick<T> {
    fn tick(&mut self, data: T) -> TickResult;
}