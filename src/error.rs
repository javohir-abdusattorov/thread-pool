use std::fmt;

#[derive(Debug, Clone)]
pub enum ThreadPoolError {
    SenderDropped,
    ReceiversDropeed,
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadPoolError::SenderDropped => "Cannot send: sender dropped".fmt(f),
            ThreadPoolError::ReceiversDropeed => "Cannot send: receivers are dropped".fmt(f),
        }
    }
}