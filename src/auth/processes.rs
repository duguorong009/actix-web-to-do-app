use super::jwt;
use actix_web::dev::ServiceRequest;

pub fn check_password(password: String) -> Result<String, &'static str> {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(m) => Err(m),
    }
}

pub fn extract_header_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match request.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(processed_password) => Ok(String::from(processed_password)),
            Err(_) => Err("there was an error processing token"),
        },
        None => Err("there is no token"),
    }
}

#[cfg(test)]
mod check_credentials_tests {
    use super::super::jwt::JwtToken;
    use super::*;
    use actix_web::test;

    #[test]
    async fn correct_check_password() {
        let correct_password = JwtToken::encode(32);
        let res = check_password(correct_password);
        match res {
            Ok(m) => assert_eq!(m, "passed"),
            Err(_) => panic!("It should pass the correct password"),
        }
    }

    #[test]
    async fn incorrect_check_password() {
        let incorrect_password = String::from("test");
        let res = check_password(incorrect_password);
        match res {
            Err(m) => assert_eq!(m, "Could not decode"),
            Ok(_) => panic!("It should not pass the incorrect password"),
        }
    }

    #[test]
    async fn no_token_in_extract_header_token() {
        let mock_request = test::TestRequest::default()
            .append_header(("token", "test"))
            .to_srv_request();
        let res = extract_header_token(&mock_request);
        match res {
            Err(m) => assert_eq!(m, "there is no token"),
            Ok(_) => panic!("It should not extract token that does not exist"),
        }
    }

    #[test]
    async fn correct_token_in_extract_header_token() {
        let mock_request = test::TestRequest::default()
            .append_header(("user-token", "test"))
            .to_srv_request();
        let res = extract_header_token(&mock_request);
        match res {
            Ok(v) => assert_eq!(v, "test"),
            Err(_) => panic!("It should extract the token when existing"),
        }
    }
}
