use actix::{Message, Handler};
use crate::ws::server::GameServer;
use serde::Serialize;

#[derive(Serialize)]
pub struct PlayerLost {
    pub player_id: usize,
    pub game_id: usize,
    pub points: usize,
}

impl Message for PlayerLost {
    type Result = ();
}

impl Handler<PlayerLost> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: PlayerLost, _ctx: &mut Self::Context) -> Self::Result {
        if let Some((_, room_name)) = self.games.get(&msg.game_id) {
            self.send_message(room_name, &msg, 0);
        }
    }
}