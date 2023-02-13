use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => processes::check_password(token),
        Err(msg) => Err(msg),
    }
}
