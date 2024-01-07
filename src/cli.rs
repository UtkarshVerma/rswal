use crate::BINARY_NAME;
use clap::Parser;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
}

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
    #[arg(long, default_value_t = format!("~/.config/{BINARY_NAME}"))]
    pub config_dir: String,

    /// Define variables as key:value pairs.
    #[arg(short = 'v', long = "variable", num_args = 1.., value_parser = parse_variable)]
    pub variables: Vec<Variable>,
}

impl Args {
    pub fn parse() -> Self {
        <Args as Parser>::parse()
    }
}

fn parse_variable(variable: &str) -> Result<Variable, String> {
    let (key, value) = variable.split_once('=').unwrap_or_default();

    if key.is_empty() || value.is_empty() {
        return Err("variables should be specified as 'key=value' pairs".to_string());
    }

    Ok(Variable {
        name: key.to_string(),
        value: value.to_string(),
    })
}

#[test]
fn test_cli() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
