use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => String::from("200 OK"),
            StatusCode::NotFound => String::from("404 Not Found"),
            StatusCode::InternalServerError => String::from("500 Internal Server Error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let status_code = self.status_code.to_string();
        let headers = self.headers.iter().map(|header| format!("{}: {}", header.0, header.1)).collect::<Vec<String>>();
        let header = headers.join("\r\n");

        format!("HTTP/1.1 {}\r\n{}\r\n\r\n{}", status_code, header, self.body)
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpResponseBuilder {
    status_code: Option<StatusCode>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self {
            status_code: None,
            headers: Vec::new(),
            body: None,
        }
    }
    pub fn set_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn add_header(mut self, header: (&str, &str)) -> Self {
        self.headers.push((header.0.to_string(), header.1.to_string()));
        self
    }
    pub fn set_body(mut self, content: &str) -> Self {
        self.body = Some(content.to_string());
        self
    }
    pub fn build(self) -> Result<HttpResponse> {
        Ok(HttpResponse {
            status_code: self.status_code.ok_or(Error::NotSetStatusCode)?,
            headers: self.headers,
            body: self.body.ok_or(Error::NotSetBody)?,
        })
    }
}

