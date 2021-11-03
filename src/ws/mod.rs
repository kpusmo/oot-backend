use std::time::Instant;

use actix_web::{Error, HttpResponse, web, HttpRequest};
use actix_web_actors::ws;
use serde::de::{self, Deserialize, Deserializer, Unexpected};

use crate::state::State;
use crate::ws::session::WsSession;

pub mod server;
pub mod error;
mod game;
mod session;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ws/{name}")
            .route(web::get().to(ws_route))
    );
}

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    println!("siema siema o tej porze każden wypić może");
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            name: req.match_info().get("name").unwrap_or("anon").parse().unwrap(),
            server: state.get_ref().server.clone(),
        },
        &req,
        stream,
    )
}

pub fn bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.to_ascii_lowercase().as_ref() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"true/1 or false/0",
        )),
    }
}
