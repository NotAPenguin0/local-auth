use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    NoAuthCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for Error {

}