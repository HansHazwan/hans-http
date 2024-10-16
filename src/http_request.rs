use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl ToString for RequestMethod {
    fn to_string(&self) -> String {
        match self {
            RequestMethod::GET => String::from("GET"),
            RequestMethod::POST => String::from("POST"),
            RequestMethod::PUT => String::from("PUT"),
            RequestMethod::DELETE => String::from("DELETE"),
            RequestMethod::PATCH => String::from("PATCH"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub request_method: RequestMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let request_method = self.request_method.to_string();
        let headers = self.headers.iter().map(|header| format!("{}: {}", header.0, header.1)).collect::<Vec<String>>();
        let headers = headers.join("\r\n");

        if let Some(body) = &self.body {
            format!("{} {} HTTP/1.1\r\n{}\r\n\r\n{}", request_method, self.url, headers, body)
        } else {
            format!("{} {} Http/1.1\r\n{}", request_method, self.url, headers)
        }
    }
}

#[derive(Debug)]
pub struct HttpRequestBuilder {
    request_method: Option<RequestMethod>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        Self {
            request_method: None,
            url: None,
            headers: Vec::new(),
            body: None,
        }
    }
    pub fn set_request_method(mut self, method: RequestMethod) -> Self {
        self.request_method = Some(method);
        self
    }
    pub fn set_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
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
    pub fn build(self) -> Result<HttpRequest> {
        Ok(HttpRequest {
            request_method: self.request_method.ok_or(Error::NotSetRequestMethod)?,
            url: self.url.ok_or(Error::NotSetUrl)?,
            headers: self.headers,
            body: self.body,
        })
    }
}
