use crate::os::{self, Path, ReadError};
use crate::renderer::Serialize;
use crate::yaml_parser::{Deserialize, ParseError, YamlParser};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("read failed -> {0}")]
    Read(#[from] ReadError),

    #[error("parse failed -> {0}")]
    Parse(#[from] ParseError),
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
    pub fn new(name: &str, theme_dir: &Path) -> Result<Self, ThemeError> {
        let file = Path::new(theme_dir).join(Path::new(name).with_extension("yaml"));
        let contents = os::read_file(&file)?;

        Ok(Self::try_from(contents.as_str())?)
    }
}

impl TryFrom<&str> for Theme {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        YamlParser::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        use tempfile::tempdir;

        let theme_dir = tempdir().unwrap();
        let theme_dir_path = theme_dir.path();
        let theme_name = "monokai";
        let theme_file = theme_dir_path.join(format!("{theme_name}.yaml"));
        os::write_to_file(
            &theme_file,
            "
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
",
        )
        .unwrap();

        let theme = Theme::new(theme_name, theme_dir_path).unwrap();

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
}
