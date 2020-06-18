use actix::{Context, Handler, Message};
use serde::{Deserialize, Serialize};
use crate::ws::server::GameServer;
use crate::ws::server::game::event::{TickResult, GameTick, AskedQuestion};
use crate::ws::session::default_id;

#[derive(Message, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct AskQuestion {
    #[serde(default = "default_id")]
    pub id: usize,
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

    fn handle(&mut self, msg: AskQuestion, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(game) = self.games.get_mut(&msg.game_id) {
            if game.host.id != msg.id {
                todo!("error")
            }
            match game.tick(AskedQuestion {
                contents: &msg.contents,
            }) {
                TickResult::QuestionAsked(answerer_id) => {
                    self.send_message(&msg.room, &AskQuestionResponse {
                        question: &msg.contents,
                        answerer_id,
                    }, 0);
                },
                _ => unimplemented!(),
            }
        }
    }
}