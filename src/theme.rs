use crate::os::{self, Path, ReadError};
use crate::parser::{Deserialize, ParseError, Parser};
use crate::renderer::Serialize;
use crate::util::Error;
use std::error::Error;

#[derive(Error, Debug)]
enum ThemeErrorKind {
    #[error("could not read theme")]
    Read(
        #[from]
        #[source]
        ReadError,
    ),

    #[error("could not parse theme")]
    Parse(
        #[from]
        #[source]
        ParseError,
    ),
}

#[derive(Error, Debug)]
// TODO: Get the source string somehow
#[error("{kind} '{theme} ({:?})", kind.source())]
pub struct ThemeError {
    theme: String,
    kind: ThemeErrorKind,
}

impl ThemeError {
    fn new(theme: String, kind: ThemeErrorKind) -> ThemeError {
        ThemeError { theme, kind }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SpecialColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
}

#[derive(Deserialize, Serialize)]
pub struct AnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Deserialize, Serialize)]
pub struct Theme {
    pub special: SpecialColors,
    pub normal: AnsiColors,
    pub bright: AnsiColors,
}

impl Theme {
    pub fn new(path: &Path) -> Result<Self, ThemeError> {
        // TODO: Handle this
        let name = path.file_stem().unwrap_or_default();
        let contents =
            os::read_file(path).map_err(|error| ThemeError::new(name.to_string(), error.into()))?;

        Self::try_from(contents.as_str())
            .map_err(|error| ThemeError::new(name.to_string(), error.into()))
    }
}

impl TryFrom<&str> for Theme {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Parser::parse(value)
    }
}

#[test]
fn test_parser() {
    let input = "
special:
  background: '#222222'
  foreground: '#f7f1ff'
  cursor: '#f7f1ff'

normal:
  black: '#363537'
  blue: '#948ae3'
  cyan: '#5ad4e6'
  green: '#7bd88f'
  magenta: '#fd9353'
  red: '#fc618d'
  white: '#bab6c0'
  yellow: '#fce566'

bright:
  black: '#69676c'
  blue: '#948ae3'
  cyan: '#5ad4e6'
  green: '#7bd88f'
  magenta: '#fd9353'
  red: '#fc618d'
  white: '#f7f1ff'
  yellow: '#fce566'
";
    let theme: Theme = input.try_into().unwrap();

    assert_eq!(theme.special.background, "#222222");
    assert_eq!(theme.special.foreground, "#f7f1ff");
    assert_eq!(theme.special.cursor, "#f7f1ff");

    assert_eq!(theme.normal.black, "#363537");
    assert_eq!(theme.normal.blue, "#948ae3");
    assert_eq!(theme.normal.cyan, "#5ad4e6");
    assert_eq!(theme.normal.green, "#7bd88f");
    assert_eq!(theme.normal.magenta, "#fd9353");
    assert_eq!(theme.normal.red, "#fc618d");
    assert_eq!(theme.normal.white, "#bab6c0");
    assert_eq!(theme.normal.yellow, "#fce566");

    assert_eq!(theme.bright.black, "#69676c");
    assert_eq!(theme.bright.blue, "#948ae3");
    assert_eq!(theme.bright.cyan, "#5ad4e6");
    assert_eq!(theme.bright.green, "#7bd88f");
    assert_eq!(theme.bright.magenta, "#fd9353");
    assert_eq!(theme.bright.red, "#fc618d");
    assert_eq!(theme.bright.white, "#f7f1ff");
    assert_eq!(theme.bright.yellow, "#fce566");
}
