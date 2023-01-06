#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl<'s> From<&'s str> for ParseError {
    fn from(message: &'s str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for ParseError {}
