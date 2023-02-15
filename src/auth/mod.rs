use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match processes::extract_header_token(request) {
        Ok(token) => processes::check_password(token),
        Err(msg) => Err(msg),
    }
}

#[cfg(test)]
mod process_token_tests {
    use super::{jwt::JwtToken, *};
    use actix_web::test::TestRequest;

    #[test]
    fn no_token_process_token() {
        let mock_request = TestRequest::default()
            .append_header(("test", "test"))
            .to_srv_request();
        let res = process_token(&mock_request);
        match res {
            Err(m) => assert_eq!(m, "there is no token"),
            Ok(_) => panic!("It should not process the token that does not exist"),
        }
    }

    #[test]
    fn incorrect_token() {
        let mock_request = TestRequest::default()
            .append_header(("user-token", "test"))
            .to_srv_request();
        let res = process_token(&mock_request);
        match res {
            Err(m) => assert_eq!(m, "Could not decode"),
            Ok(_) => panic!("It should not decode the invalid token"),
        }
    }

    #[test]
    fn correct_token() {
        let encoded_token = JwtToken::encode(32);
        let mock_request = TestRequest::default()
            .append_header(("user-token", encoded_token))
            .to_srv_request();
        let res = process_token(&mock_request);
        match res {
            Err(_) => panic!("It should not error the valid token"),
            Ok(m) => assert_eq!(m, "passed"),
        }
    }
}
