use serde::{Deserialize, Serialize};

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
pub struct Colorscheme {
    pub special: SpecialColors,
    pub normal: AnsiColors,
    pub bright: AnsiColors,
}

impl Colorscheme {
    pub fn new(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
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
  magenta: "#fc618d"
  red: "#fd9353"
  white: "#bab6c0"
  yellow: "#fce566"

bright:
  black: "#69676c"
  blue: "#948ae3"
  cyan: "#5ad4e6"
  green: "#7bd88f"
  magenta: "#fc618d"
  red: "#fd9353"
  white: "#f7f1ff"
  yellow: "#fce566"
"##;
        let colorscheme = Colorscheme::new(input).unwrap();

        assert_eq!(colorscheme.special.background, "#222222");
        assert_eq!(colorscheme.special.foreground, "#f7f1ff");
        assert_eq!(colorscheme.special.cursor, "#f7f1ff");

        assert_eq!(colorscheme.normal.black, "#363537");
        assert_eq!(colorscheme.normal.blue, "#948ae3");
        assert_eq!(colorscheme.normal.cyan, "#5ad4e6");
        assert_eq!(colorscheme.normal.green, "#7bd88f");
        assert_eq!(colorscheme.normal.magenta, "#fc618d");
        assert_eq!(colorscheme.normal.red, "#fd9353");
        assert_eq!(colorscheme.normal.white, "#bab6c0");
        assert_eq!(colorscheme.normal.yellow, "#fce566");

        assert_eq!(colorscheme.bright.black, "#69676c");
        assert_eq!(colorscheme.bright.blue, "#948ae3");
        assert_eq!(colorscheme.bright.cyan, "#5ad4e6");
        assert_eq!(colorscheme.bright.green, "#7bd88f");
        assert_eq!(colorscheme.bright.magenta, "#fc618d");
        assert_eq!(colorscheme.bright.red, "#fd9353");
        assert_eq!(colorscheme.bright.white, "#f7f1ff");
        assert_eq!(colorscheme.bright.yellow, "#fce566");
    }
}
