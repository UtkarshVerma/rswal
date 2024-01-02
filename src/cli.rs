use clap::Parser;

use crate::os;
use crate::BINARY_NAME;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set the theme.
    #[arg(short, long)]
    pub theme: Option<String>,

    /// List available themes.
    #[arg(long)]
    pub list_themes: bool,

    /// Set the configuration directory.
    #[arg(long, default_value_t = format!("~/.config/{BINARY_NAME}"), value_parser = parse_path)]
    pub config_dir: String,

    /// Define variables as key:value pairs.
    #[arg(short, long, num_args = 1..)]
    pub variables: Option<Vec<String>>,
}

// TODO: Validate path
fn parse_path(path: &str) -> Result<String, std::io::Error> {
    Ok(os::resolve_path(path))
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[test]
fn test_parser() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
