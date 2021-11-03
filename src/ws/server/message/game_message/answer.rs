use actix::{Context, Handler, Message, WrapFuture, fut, ContextFutureSpawner, ActorFuture};
use serde::{Deserialize, Serialize};
use crate::ws::server::GameServer;
use crate::ws::game::message::{GameMessageResponse, GameMessageError};
use crate::ws::game::message::answer::AnswerQuestion as GameAnswerQuestion;

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct AnswerQuestion {
    #[serde(default)]
    pub session_id: usize,
    game_id: usize,
    room: String,
    contents: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnswerQuestionResponse<'a> {
    answerer_id: usize,
    question: &'a str,
    answer: &'a str,
}


impl Handler<AnswerQuestion> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: AnswerQuestion, ctx: &mut Context<Self>) -> Self::Result {
        if let Some((game, _)) = self.games.get(&msg.game_id) {
            game.send(GameAnswerQuestion {
                contents: msg.contents.clone(),
                answerer_id: msg.session_id
            })
                .into_actor(self)
                .then(move |res, act, _ctx| {
                    match res.unwrap_or_else(|err| Err(GameMessageError::new(&format!("{:?}", err), msg.game_id))) {
                        Ok(res) => {
                            if let GameMessageResponse::QuestionAnswered(answerer_id, answer, question) = res {
                                act.send_message(&msg.room, &AnswerQuestionResponse {
                                    answerer_id,
                                    question: &question,
                                    answer: &answer,
                                }, answerer_id);
                            } else {
                                act.fail_to(msg.session_id, &GameMessageError::new(&format!("expected GameMessageResult::QuestionAnswered, received {:?}", res), msg.game_id));
                            }
                        },
                        Err(err) => act.fail_to(msg.session_id, &err)
                    }
                    fut::ready(())
                })
                .wait(ctx);
        } else {
            self.fail_to(msg.session_id, &GameMessageError::new("Invalid game_id", msg.game_id));
        }
    }
}