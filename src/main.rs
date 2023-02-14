#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{dev::Service, App, HttpResponse, HttpServer};
use env_logger;
use futures::future::{ok, Either};
use log;

mod auth;
mod database;
mod json_serialization;
mod models;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                let request_url: String = String::from(*&req.uri().path().clone());

                let passed: bool;
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => passed = true,
                        Err(_msg) => passed = false,
                    }
                } else {
                    passed = true;
                }
                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => Either::Right(ok(req.into_response(
                        HttpResponse::Unauthorized().finish().map_into_boxed_body(),
                    ))),
                };
                async move {
                    let result = end_result.await?;
                    log::info!("{} -> {}", request_url, &result.status());
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
