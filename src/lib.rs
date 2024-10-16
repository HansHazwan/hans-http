mod error;
mod prelude;
mod http_request;
mod http_response;

pub use crate::http_request::*;
pub use crate::http_response::*;
pub use crate::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_method_to_string() {
        let request_method = RequestMethod::POST;

        assert_eq!(request_method.to_string(), "POST");
    }
    #[test]
    fn create_http_request_from_builder() {
        let http_request = HttpRequestBuilder::new()
            .set_request_method(RequestMethod::POST)
            .set_url("/users")
            .add_header(("Content-Type", "application/json"))
            .set_body("{\"username\": \"Landa\"}")
            .build()
            .unwrap()
            .to_string();

        println!("{}", http_request);
        assert_eq!(http_request, "POST /users HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"username\": \"Landa\"}");
    }
    #[test]
    fn create_http_response_from_builder() {
        let http_response = HttpResponseBuilder::new()
            .set_status_code(StatusCode::Ok)
            .add_header(("Content-Type", "application/json"))
            .set_body("{\"username\": \"Landa\"}")
            .build()
            .unwrap()
            .to_string();

        println!("{}", http_response);
        assert_eq!(http_response, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"username\": \"Landa\"}");
    }
}
