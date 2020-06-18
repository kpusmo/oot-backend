use actix::Addr;
use crate::ws::server::GameServer;

pub struct State {
    pub server: Addr<GameServer>,
}