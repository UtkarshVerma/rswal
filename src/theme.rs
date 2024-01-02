use std::fmt::{self, Debug};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ParseError {
    line: Option<usize>,
    column: Option<usize>,
    reason: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(column)) => write!(f, "({}, {}): {}", line, column, self.reason),
            _ => write!(f, "{}", self.reason),
        }
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
    pub fn new(yaml: &str) -> Result<Self, ParseError> {
        serde_yaml::from_str(yaml).map_err(|error| {
            let location = error.location();
            ParseError {
                line: location.as_ref().map(|l| l.line()),
                column: location.as_ref().map(|l| l.column()),
                reason: error.to_string(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = r##"
special:
  background: "#222222"
  foreground: "#f7f1ff"
  cursor: "#f7f1ff"

normal:
  black: "#363537"
  blue: "#948ae3"
  cyan: "#5ad4e6"
  green: "#7bd88f"
  magenta: "#fd9353"
  red: "#fc618d"
  white: "#bab6c0"
  yellow: "#fce566"

bright:
  black: "#69676c"
  blue: "#948ae3"
  cyan: "#5ad4e6"
  green: "#7bd88f"
  magenta: "#fd9353"
  red: "#fc618d"
  white: "#f7f1ff"
  yellow: "#fce566"
"##;
        let theme = Theme::new(input).unwrap();

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
