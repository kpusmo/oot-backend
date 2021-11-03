use serde::Serialize;
use actix::{Handler, Message};
use crate::ws::server::GameServer;

#[derive(Debug, Serialize, Message)]
#[serde(rename_all = "snake_case")]
#[rtype(result = "()")]
pub struct AnswerTimedOut {
    pub new_state: String,
    pub game_id: usize
}

impl Handler<AnswerTimedOut> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: AnswerTimedOut, _ctx: &mut Self::Context) -> Self::Result {
        println!("GameServer::handle::<AnswerTimedOut>, game_id: {}", msg.game_id);
        if let Some((_, room)) = self.games.get(&msg.game_id) {
            self.send_message(room, &msg, 0);
        }
    }
}