#[derive(Debug, Fail)]
#[fail(display = "Failed to parse query string")]
pub struct QSParseError;
