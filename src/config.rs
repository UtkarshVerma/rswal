use crate::os::{self, Path, PathBuf, ReadError};
use crate::parser::{de, Deserialize, Deserializer, ParseError, Parser};
use crate::renderer::Value;
use crate::util::{Error, HashMap};

const CONFIG_FILE: &str = "config.yaml";

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

    // TODO: This could use a rename to #[parser(...)]
    #[serde(deserialize_with = "resolve_path")]
    pub target: PathBuf,
}

fn resolve_path<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
    let path = PathBuf::deserialize(deserializer)?;

    os::resolve_path(&path).ok_or(de::Error::custom(format!(
        "could not resolve path '{}'",
        path.display()
    )))
}

#[derive(Deserialize)]
pub struct Config {
    pub theme: Option<String>,
    pub hooks: Option<Vec<String>>,
    pub variables: Option<HashMap<String, Value>>,
    pub templates: Option<Vec<Template>>,
}

impl Config {
    pub fn new(config_dir: &Path) -> Result<Self, ConfigError> {
        let config_file = Path::new(config_dir).join(CONFIG_FILE);
        let contents = os::read_file(&config_file)?;

        Ok(contents.as_str().try_into()?)
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
    use tempfile::tempdir;

    let config_dir = tempdir().unwrap();
    let config_dir_path = config_dir.path();
    let config_file = config_dir_path.join(CONFIG_FILE);
    os::write_to_file(
        &config_file,
        "
theme: monokai

hooks:
  - set-wallpaper.sh

variables:
  alpha: 0.1

templates:
  - source: dunstrc
    target: ~/.config/dunst/dunstrc
  - source: colors.rasi
    target: ~/.config/rofi/colors.rasi
",
    )
    .unwrap();

    let homedir = home::home_dir().unwrap();

    let config = Config::new(config_dir_path).unwrap();
    let templates = config.templates.unwrap_or_default();
    let variables = config.variables.unwrap_or_default();
    let hooks = config.hooks.unwrap_or_default();

    let dunstrc = templates.get(0).unwrap();
    assert_eq!(dunstrc.source, "dunstrc");
    assert_eq!(dunstrc.target, homedir.join(".config/dunst/dunstrc"));

    let rofi_colors = templates.get(1).unwrap();
    assert_eq!(rofi_colors.source, "colors.rasi");
    assert_eq!(rofi_colors.target, homedir.join(".config/rofi/colors.rasi"));

    assert_eq!(variables.get("alpha").unwrap(), 0.1);
    assert_eq!(config.theme.unwrap(), "monokai");

    assert_eq!(hooks.get(0).unwrap(), "set-wallpaper.sh");
}
