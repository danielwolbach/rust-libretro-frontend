pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Library(libloading::Error),
    Api(libloading::Error),
    Content(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Library(inner) => write!(f, "Failed to load library: {}", inner),
            Error::Api(inner) => write!(f, "Failed to load Core API: {}", inner),
            Error::Content(inner) => write!(f, "Failed to load content: {}", inner),
        }
    }
}

impl std::error::Error for Error {}
