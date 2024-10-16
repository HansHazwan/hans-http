#[derive(Debug)]
pub enum Error {
    Custom(String),
    NotSetRequestMethod,
    NotSetUrl,
    NotSetBody,
    NotSetStatusCode,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(message) => write!(f, "Custom Error: {}", message),
            Error::NotSetRequestMethod => write!(f, "Error Create Http Request: Not Set Request Method."),
            Error::NotSetUrl => write!(f, "Error Create Http Request: Not Set Url."),
            Error::NotSetBody => write!(f, "Error Create Http Response: Not Set Body"),
            Error::NotSetStatusCode => write!(f, "Error Create Http Response: Not Set Status Code"),
        }
    }
}

impl std::error::Error for Error { }
