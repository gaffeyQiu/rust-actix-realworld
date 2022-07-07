#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

mod app;
mod constants;
mod error;
mod middleware;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start server ...");

    let state = {
        let pool = utils::db::establish_connection();
        middleware::state::AppState { pool }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
