mod create;

use super::path::Path;
use actix_web::web;

pub fn user_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/user"),
        backend: true,
    };

    app.route(
        &base_path.define(String::from("/create")),
        web::post().to(create::create),
    );
}
