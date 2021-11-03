use actix::{Context, ContextFutureSpawner, Handler, Message, WrapFuture, fut, ActorFuture};
use serde::{Deserialize, Serialize};

use crate::ws::game::message::ask_question::AskQuestion as GameAskQuestion;
use crate::ws::game::message::{GameMessageResponse, GameMessageError};
use crate::ws::server::GameServer;

#[derive(Message, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct AskQuestion {
    #[serde(default)]
    pub session_id: usize,
    game_id: usize,
    room: String,
    pub contents: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AskQuestionResponse<'a> {
    question: &'a str,
    answerer_id: usize,
}

impl Handler<AskQuestion> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: AskQuestion, ctx: &mut Context<Self>) -> Self::Result {
        if let Some((game, _)) = self.games.get_mut(&msg.game_id) {
            game.send(GameAskQuestion {
                host_id: msg.session_id,
                contents: msg.contents.clone(),
            })
                .into_actor(self)
                .then(move |res, act, _ctx| {
                    match res.unwrap_or_else(|err| Err(GameMessageError::new(&format!("{:?}", err), msg.game_id))) {
                        Ok(res) => {
                            if let GameMessageResponse::QuestionAsked(answerer_id) = res {
                                act.send_message(&msg.room, &AskQuestionResponse {
                                    question: &msg.contents,
                                    answerer_id,
                                }, 0);
                            } else {
                                act.fail_to(msg.session_id, &GameMessageError::new(&format!("expected GameMessageResult::QuestionAsked, received {:?}", res), msg.game_id));
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
