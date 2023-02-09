use actix_web::web;

mod auth;
mod path;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
}
