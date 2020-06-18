pub mod third_round {
    #[derive(Debug)]
    pub enum ThirdRoundGameState {
        Start,
        AskQuestion(AskQuestionState),
        Answer(AnswerState),
        ValidateAnswer(ValidateAnswerState),
        ChooseAnswerer(ChooseAnswererState),
        Finished,
    }

    #[derive(Debug)]
    pub struct AskQuestionState {
        pub old_answerer_id: Option<usize>,
        pub answerer_id: usize,
    }

    #[derive(Debug)]
    pub struct AnswerState {
        pub old_answerer_id: Option<usize>,
        pub answerer_id: usize,
        pub question: String,
    }

    #[derive(Debug)]
    pub struct ValidateAnswerState {
        pub old_answerer_id: Option<usize>,
        pub answerer_id: usize,
        pub answer: String,
    }

    #[derive(Debug)]
    pub struct ChooseAnswererState {
        pub old_answerer_id: Option<usize>,
        pub chooser_id: usize,
    }
}