use crate::os::{self, Path, PathBuf};
use crate::parser::Parser;
use crate::renderer::Value;
use crate::BINARY_NAME;
use clap::Parser as ArgParser;

#[derive(ArgParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set the theme.
    #[arg(short, long)]
    pub theme: Option<String>,

    /// List available themes.
    #[arg(long)]
    pub list_themes: bool,

    /// Set the configuration directory.
    #[arg(long, default_value=default_config_dir().into_os_string(), value_parser = parse_config_dir)]
    pub config_dir: PathBuf,

    /// Specify hooks.
    #[arg(short = 'H', long, num_args = 1..)]
    pub hooks: Option<Vec<String>>,

    /// Define variables as key=value pairs.
    #[arg(short = 'v', num_args = 1.., value_parser = parse_variable)]
    pub variables: Option<Vec<(String, Value)>>,
}

impl Args {
    pub fn parse() -> Self {
        <Args as ArgParser>::parse()
    }
}

fn default_config_dir() -> PathBuf {
    Path::new("~/.config").join(BINARY_NAME)
}

fn parse_config_dir(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    let resolved = os::resolve_path(&path);

    resolved.ok_or("could not resolve the provided path".to_string())
}

fn parse_variable(variable: &str) -> Result<(String, Value), String> {
    let (key, value) = variable.split_once('=').unwrap_or_default();
    if key.is_empty() || value.is_empty() {
        return Err("variables should be specified as 'key=value' pairs".to_string());
    }

    Ok((
        key.to_string(),
        Parser::parse(value).map_err(|err| err.to_string())?,
    ))
}

#[test]
fn test_parser() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
