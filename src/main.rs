use actix::Actor;
use actix_cors::Cors;
use actix_web::{App, http, HttpServer, web};
use listenfd::ListenFd;

mod api;
mod ws;
mod state;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let state = web::Data::new(state::State {
        server: ws::server::GameServer::default().start(),
    });
    let mut http_server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://oot.local.net:8888")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            .app_data(state.clone())
            .configure(api::routes)
            .configure(ws::routes)
    });
    http_server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        http_server.listen(l).unwrap()
    } else {
        http_server.bind("0.0.0.0:80").unwrap()
    };

    http_server.run().await
}
