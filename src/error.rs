use serde::{Serialize, Deserialize};

// Error type
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    description: String,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn new<T: std::fmt::Display>(message: T) -> Self {
        Self {
            description: message.to_string(),
            source: None,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn 'static + std::error::Error)> {
        self.source.as_ref().map(|s| &**s as &(dyn 'static + std::error::Error))
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

// Result type
pub type Result<T, E = Error> = std::result::Result<T, E>;
