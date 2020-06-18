use serde::Serialize;

use actix::{Context, Handler, Message};
use crate::ws::server::GameServer;

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DisconnectResponse {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SomeoneDisconnectedResponse<'a> {
    disconnected_id: usize,
    room: &'a str,
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        if let Some(connection) = self.sessions.remove(&msg.id) {
            println!("id {} disconnected", msg.id);
            self.send_message_to(connection.id, &DisconnectResponse {});
            let mut rooms: Vec<String> = vec![];
            for (name, room) in &mut self.rooms {
                if room.members.remove(&msg.id) {
                    rooms.push(name.clone());
                }
            }
            for room in rooms {
                self.send_message(&room, &SomeoneDisconnectedResponse {
                    disconnected_id: connection.id,
                    room: &room,
                }, 0);
            }
        }
    }
}
