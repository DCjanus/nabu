use failure::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Failed to parse query string")]
pub struct QSParseError;

#[derive(Debug, Fail)]
#[fail(display = "Failed to parse response: {}", text)]
pub struct WrongResponse {
    pub text: String,
}
