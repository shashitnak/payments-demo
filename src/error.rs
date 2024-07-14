use actix_web::ResponseError;
use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    DB(crate::db::Error),
    #[from]
    IO(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DB(e) => e.fmt(f),
            Error::IO(e) => e.fmt(f),
        }
    }
}

impl ResponseError for Error {}
