#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{dev::Service, App, HttpServer};

mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                if *&req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("token is passable"),
                        Err(msg) => println!("token error: {}", msg),
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
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
