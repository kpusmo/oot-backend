use std::collections::{HashMap, HashSet};

use actix::{Actor, Addr, Context, Recipient};
use serde::Serialize;

use crate::ws::game::Game;
use crate::ws::server::message::ServerMessage;
use crate::ws::session::message::SessionMessage;

pub mod message;

#[derive(Debug)]
pub struct GameServer {
    rooms: HashMap<String, Room>,
    sessions: HashMap<usize, Connection>,
    session_counter: usize,
    game_counter: usize,
    /// (game_addr, room_name)
    games: HashMap<usize, (Addr<Game>, String)>,
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Default for GameServer {
    fn default() -> Self {
        GameServer {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            session_counter: 0,
            game_counter: 0,
            games: HashMap::new(),
        }
    }
}

impl GameServer {
    /// Send a success message to all users in the room
    /// # Arguments
    /// * `room` - name of the room to send the message to
    /// * `skip_id` - can be used to filter out a room member (e.g. when they're the one who sends the message); 0 can be used in order not to filter anyone out
    fn send_message<T: Serialize>(&self, room: &str, message: &T, skip_id: usize) {
        if let Some(room) = self.rooms.get(room) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
            for id in &room.members {
                if *id != skip_id {
                    if let Some(connection) = self.sessions.get(id) {
                        let _ = connection.addr.do_send(msg.clone());
                    }
                }
            }
        }
    }

    /// Send a success message to given user
    /// # Arguments
    /// * `receiver_id` - id of the session to send message to
    /// * `message` - a serializable message
    fn send_message_to<T: Serialize>(&self, receiver_id: usize, message: &T) {
        if let Some(connection) = self.sessions.get(&receiver_id) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::success(message)).unwrap());
            let _ = connection.addr.do_send(msg);
        }
    }

    /// Send a failure message to given user
    /// # Arguments
    /// * `receiver_id` - id of the session to send message to
    /// * `message` - a serializable message
    fn fail_to<T: Serialize>(&self, receiver_id: usize, message: &T) {
        if let Some(connection) = self.sessions.get(&receiver_id) {
            let msg = SessionMessage::from(serde_json::to_string(&ServerMessage::failure(message)).unwrap());
            let _ = connection.addr.do_send(msg);
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub id: usize,
    pub name: String,
    pub addr: Recipient<SessionMessage>,
    pub rooms: HashSet<String>,
}

#[derive(Debug)]
pub struct Room {
    pub members: HashSet<usize>,
    pub game_ids: HashSet<usize>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            members: HashSet::new(),
            game_ids: HashSet::new(),
        }
    }
}
