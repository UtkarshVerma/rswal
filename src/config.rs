use crate::os::{self, Path, ReadError};
use crate::parser::{Deserialize, Error as ParseError, Parser};
use crate::renderer::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    ParseFailed(ParseError),
    ReadFailed(ReadError),
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseFailed(error)
    }
}

impl From<ReadError> for Error {
    fn from(error: ReadError) -> Self {
        Error::ReadFailed(error)
    }
}

#[derive(Deserialize)]
pub struct Template {
    pub source: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub theme: Option<String>,
    pub variables: Option<HashMap<String, Value>>,
    pub templates: Option<Vec<Template>>,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self, Error> {
        os::read_file(path)?.as_str().try_into()
    }
}

impl TryFrom<&str> for Config {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Parser::parse(value)?)
    }
}

#[test]
fn test_parser() {
    let input = "
theme: monokai

variables:
  alpha: 0.1

templates:
  - source: dunstrc
    target: ~/.config/dunst/dunstrc
  - source: colors.rasi
    target: ~/.config/rofi/colors.rasi
";
    let config: Config = input.try_into().unwrap();
    let templates = config.templates.unwrap_or_default();
    let variables = config.variables.unwrap_or_default();

    let dunstrc = templates.get(0).unwrap();
    assert_eq!(dunstrc.source, "dunstrc".to_string());
    assert_eq!(dunstrc.target, "~/.config/dunst/dunstrc".to_string());

    let rofi_colors = templates.get(1).unwrap();
    assert_eq!(rofi_colors.source, "colors.rasi".to_string());
    assert_eq!(rofi_colors.target, "~/.config/rofi/colors.rasi".to_string());

    assert_eq!(variables.get("alpha").unwrap(), 0.1);
    assert_eq!(config.theme.unwrap(), "monokai");
}
