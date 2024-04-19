use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidGraph(String),
    Default(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Default(s) => write!(f, "Error: {}", s),
            Error::InvalidGraph(s) => write!(f, "Invalid graph: {}", s),
        }
    }
}

impl Error {
    pub fn invalid_graph(s: &str) -> Error {
        Error::InvalidGraph(s.to_string())
    }

    pub fn default(s: &str) -> Error {
        Error::Default(s.to_string())
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        e.to_string()
    }
}

impl From<Error> for Box<dyn std::error::Error> {
    fn from(e: Error) -> Box<dyn std::error::Error> {
        Box::new(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Default(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::Default(s.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Default(e.to_string())
    }
}
