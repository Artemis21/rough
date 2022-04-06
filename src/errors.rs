//! All errors raised by the CLI and renderer.

#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    Tera(tera::Error),
    Yaml(serde_yaml::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::File(error)
    }
}

impl From<tera::Error> for Error {
    fn from(error: tera::Error) -> Self {
        Self::Tera(error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::Yaml(error)
    }
}
