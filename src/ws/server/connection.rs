use actix::{Recipient, Addr};
use crate::ws::session::message::SessionMessage;
use crate::ws::session::WsSession;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Connection {
    pub id: usize,
    pub name: String,
    pub addr: Recipient<SessionMessage>,
    pub rooms: HashSet<String>
}