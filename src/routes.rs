use crate::app;
use actix_web::web;
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/users").route("", web::post().to(app::user::api::signup))),
    );
}
