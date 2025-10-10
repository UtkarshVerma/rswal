use crate::os;
use crate::renderer::Value;
use crate::yaml_parser::YamlParser;
use clap::Parser as ArgParser;
use std::env;
use std::path::{Path, PathBuf};

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

// TODO: Make clap follow the app's error reporting style
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
    #[arg(short, long, num_args = 1.., value_parser = parse_key_value_pair)]
    pub variables: Option<Vec<(String, Value)>>,
}

impl Args {
    pub fn parse() -> Self {
        <Args as ArgParser>::parse()
    }
}

fn default_config_dir() -> PathBuf {
    let config_dir = env::var("XDG_CONFIG_DIR").unwrap_or("~/.config".to_string());
    Path::new(&config_dir).join(BINARY_NAME)
}

fn parse_config_dir(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    let resolved = os::resolve_path(&path);

    resolved.ok_or("could not resolve the provided path".to_string())
}

fn parse_key_value_pair(pair: &str) -> Result<(String, Value), String> {
    let (key, value) = pair.split_once('=').unwrap_or_default();
    if key.is_empty() {
        return Err("variables should be specified as 'key=value' pairs".to_string());
    }

    Ok((
        key.to_string(),
        YamlParser::parse(value).map_err(|err| err.to_string())?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        use clap::CommandFactory;

        Args::command().debug_assert()
    }
}
