use actix_web::{web, Responder, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(index))
    );
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("no elo")
}