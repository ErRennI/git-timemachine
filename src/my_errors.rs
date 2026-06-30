use std::fmt;

#[derive(Debug)]
pub enum TimeMachineError {
    Git(git2::Error),
    Io(std::io::Error),
    EmptyRepository,
}

impl fmt::Display for TimeMachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeMachineError::Git(e) => write!(f, "Git error: {}", e),
            TimeMachineError::Io(e) => write!(f, "I/O error: {}", e),
            TimeMachineError::EmptyRepository => {
                write!(f, "Error: This repository has no commits!")
            }
        }
    }
}

impl std::error::Error for TimeMachineError {}

impl From<git2::Error> for TimeMachineError {
    fn from(err: git2::Error) -> Self {
        TimeMachineError::Git(err)
    }
}

impl From<std::io::Error> for TimeMachineError {
    fn from(err: std::io::Error) -> Self {
        TimeMachineError::Io(err)
    }
}
