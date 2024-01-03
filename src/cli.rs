use clap::Parser;

use crate::parser::Parser as YamlParser;
use crate::renderer::Value;
use crate::BINARY_NAME;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value,
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
        <Args as clap::Parser>::parse()
    }
}

fn parse_variable(variable: &str) -> Result<Variable, String> {
    let (key, value) = variable.split_once('=').unwrap_or_default();

    if key.is_empty() || value.is_empty() {
        return Err("variables should be specified as 'key=value' pairs".to_string());
    }

    let value = YamlParser::parse(value)
        .map_err(|_| format!("invalid value specified for variable '{key}'"))?;

    Ok(Variable {
        name: key.into(),
        value,
    })
}

#[test]
fn test_parser() {
    use clap::CommandFactory;

    Args::command().debug_assert()
}
