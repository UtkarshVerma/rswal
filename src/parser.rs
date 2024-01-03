pub use serde::Deserialize;
pub use serde_yaml::Error;

pub struct Parser;

impl Parser {
    pub fn parse<'a, T: Deserialize<'a>>(contents: &'a str) -> Result<T, Error> {
        Ok(serde_yaml::from_str(contents)?)
    }
}
