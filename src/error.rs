use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseError {
    NotFound,
    Syntax(String),
    Extension(String),
    Io(String),
}

impl DatabaseError {
    pub fn message(&self) -> &str {
        match self {
            DatabaseError::NotFound => "key not found",
            DatabaseError::Syntax(msg) => msg.as_str(),
            DatabaseError::Extension(msg) => msg.as_str(),
            DatabaseError::Io(msg) => msg.as_str(),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {}", self.message())
    }
}

impl std::error::Error for DatabaseError {}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::Io(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_format() {
        assert_eq!(DatabaseError::NotFound.to_string(), "ERROR: key not found");
        assert_eq!(
            DatabaseError::Syntax("unknown command: FOO".to_string()).to_string(),
            "ERROR: unknown command: FOO"
        );
        assert_eq!(
            DatabaseError::Extension("invalid CPF".to_string()).to_string(),
            "ERROR: invalid CPF"
        );
        assert_eq!(
            DatabaseError::Io("read failure".to_string()).to_string(),
            "ERROR: read failure"
        );
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken pipe");
        let db_err: DatabaseError = io_err.into();
        assert_eq!(db_err, DatabaseError::Io("broken pipe".to_string()));
    }
}
