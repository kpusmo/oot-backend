use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum GameState {
    Starting,
    AskQuestion(AskQuestionState),
    Answer(AnswerState),
    ValidateAnswer(ValidateAnswerState),
    ChooseAnswerer(ChooseAnswererState),
    _Finished,
}

#[derive(Serialize, Debug, Clone)]
pub struct AskQuestionState {
    pub previous_answerer_id: Option<usize>,
    pub answerer_id: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct AnswerState {
    pub previous_answerer_id: Option<usize>,
    pub answerer_id: usize,
    pub question: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ValidateAnswerState {
    pub previous_answerer_id: Option<usize>,
    pub answerer_id: usize,
    pub answer: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ChooseAnswererState {
    pub previous_answerer_id: Option<usize>,
    pub chooser_id: usize,
}