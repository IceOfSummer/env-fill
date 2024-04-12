use std::fmt::Display;

/// 
/// 预期之外的错误
///
#[derive(Debug)]
pub enum UnExpectedError {
    Argument(String),
    IO(std::io::Error)
}

impl std::error::Error for UnExpectedError { }

impl Display for UnExpectedError {

    fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        use self::UnExpectedError::*;
        match &self {
            Argument(msg) => write!(f,"{}",msg),
            IO(e) => write!(f, "{}", e)
        }
    }

}

impl From<std::io::Error> for UnExpectedError {
    fn from(io_error: std::io::Error)->Self {
        UnExpectedError::IO(io_error)
    }
}

