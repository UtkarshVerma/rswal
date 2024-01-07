use crate::os::{self, Path, ReadError};
use crate::parser::{Deserialize, ParseError, Parser};
use crate::renderer::Value;
use crate::util::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not parse config ({0})")]
    ParseFailed(#[from] ParseError),

    #[error("could not read config ({0})")]
    ReadFailed(#[from] ReadError),
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
    pub fn new(path: &Path) -> Result<Self, ConfigError> {
        let contents = os::read_file(path)?;

        Ok(Self::try_from(contents.as_str())?)
    }
}

impl TryFrom<&str> for Config {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Parser::parse(value)
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
