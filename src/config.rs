use serde::Deserialize;
use serde_yaml::{Error, Mapping};

#[derive(Deserialize)]
pub struct Templates {
    pub source: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub theme: Option<String>,
    pub variables: Option<Mapping>,
    pub templates: Option<Vec<Templates>>,
}

impl Config {
    pub fn new(config: &str) -> Result<Self, Error> {
        serde_yaml::from_str(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
theme: monokai

variables:
  alpha: 0.1

templates:
  - source: dunstrc
    target: ~/.config/dunst/dunstrc
  - source: colors.rasi
    target: ~/.config/rofi/colors.rasi
"#;
        let config = Config::new(input).unwrap();
        let templates = config.templates.unwrap();
        let variables = config.variables.unwrap();

        let dunstrc = templates.get(0).unwrap();
        assert_eq!(dunstrc.source, "dunstrc".to_string());
        assert_eq!(dunstrc.target, "~/.config/dunst/dunstrc".to_string());

        let rofi_colors = templates.get(1).unwrap();
        assert_eq!(rofi_colors.source, "colors.rasi".to_string());
        assert_eq!(rofi_colors.target, "~/.config/rofi/colors.rasi".to_string());

        assert_eq!(variables.get("alpha").unwrap(), 0.1);
        assert_eq!(config.theme.unwrap(), "monokai");
    }
}
