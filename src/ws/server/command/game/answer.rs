use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};
use crate::ws::server::GameServer;
use crate::ws::server::game::event::{QuestionAnswered, GameTick, TickResult};
use crate::ws::session::default_id;

#[derive(Debug, Deserialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct Answer {
    #[serde(default = "default_id")]
    pub id: usize,
    game_id: usize,
    room: String,
    contents: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnswerResponse<'a> {
    answerer_id: usize,
    question: &'a str,
    answer: &'a str,
}

impl Handler<Answer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Answer, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(game) = self.games.get_mut(&msg.game_id) {
            if let Some(answerer_id) = game.get_answerer_id() {
                if msg.id != answerer_id {
                    todo!("error")
                }
            } else {
                todo!("error")
            }
            match game.tick(QuestionAnswered {
                contents: msg.contents.clone(),
            }) {
                TickResult::QuestionAnswered(answerer_id, answer, question) => {
                    self.send_message(&msg.room, &AnswerResponse {
                        answerer_id,
                        question: &question,
                        answer: &answer,
                    }, answerer_id);
                },
                _ => unimplemented!(),
            }
        }
    }
}