mod cli;
mod config;
mod directories;
mod hook;
mod logger;
mod os;
mod parser;
mod renderer;
mod template;
mod theme;
mod util;

use crate::cli::Args;
use crate::config::Config;
use crate::directories::Directories;
use crate::hook::Hook;
use crate::logger::{error, warn, Logger};
use crate::os::{exit, Path, ReadDirError};
use crate::parser::Parser;
use crate::renderer::{context, Renderer, Value};
use crate::template::Template;
use crate::theme::Theme;
use crate::util::{anyhow, Result};

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    Logger::init();

    run().unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
}

// TODO: Remove `anyhow` mentions
fn run() -> Result<()> {
    let cli_args = Args::parse();
    let dirs = Directories::new(&cli_args.config_dir);

    if cli_args.list_themes {
        list_themes(&dirs.theme_dir)?;

        return Ok(());
    }

    let config_file = Path::new(&cli_args.config_dir).join("config.yaml");
    let config = Config::new(&config_file)?;

    render_templates(&dirs, &cli_args, &config)?;
    execute_hooks(&dirs.hook_dir, &cli_args, &config);

    Ok(())
}

fn list_themes(theme_dir: &Path) -> Result<()> {
    let files = os::read_dir(theme_dir).map_err(|error| {
        let message = match error {
            ReadDirError::DirectoryDoesNotExist => "theme directory does not exist".to_string(),
            ReadDirError::PermissionDenied => "permission denied for theme directory".to_string(),
            ReadDirError::Other(error) => format!("{error}"),
        };

        anyhow!(message)
    })?;

    let mut themes = files
        .iter()
        .filter(|file| file.extension().unwrap_or_default() == "yaml")
        .map(|file| file.file_stem().unwrap_or_default())
        .collect::<Vec<&str>>();
    themes.sort();

    for theme in themes {
        println!("{theme}");
    }

    Ok(())
}

fn render_templates(dirs: &Directories, cli_args: &Args, config: &Config) -> Result<()> {
    let theme_name = cli_args
        .theme
        .as_deref()
        .or(config.theme.as_deref())
        .ok_or(anyhow!("no theme specified"))?;
    let theme_file = dirs.theme_dir.join(format!("{theme_name}.yaml"));
    let theme = Theme::new(&theme_file)?;

    // TODO: Avoid cloning config hashmap
    let mut variables = config.variables.clone().unwrap_or_default();
    let cli_variables = cli_args
        .variables
        .iter()
        .map(|v| {
            let value = Parser::parse::<Value>(&v.value)?;

            Ok((v.name.to_string(), value))
        })
        .collect::<Result<Vec<(String, Value)>>>()?;
    variables.extend(cli_variables);

    let context = context!({
        "variables": variables,
        "colors": theme,
    });
    let renderer = Renderer::new(&context);

    config
        .templates
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|template| Template::new(&template.source, &template.target, &dirs.template_dir))
        .for_each(|template| {
            template
                .render(&renderer)
                .unwrap_or_else(|error| warn!("{error}"))
        });

    Ok(())
}

fn execute_hooks(hook_dir: &Path, cli_args: &Args, config: &Config) {
    cli_args
        .hooks
        .as_deref()
        .unwrap_or(config.hooks.as_deref().unwrap_or_default())
        .iter()
        .map(|hook| Hook::new(hook, hook_dir))
        .for_each(|hook| {
            hook.execute().unwrap_or_else(|error| warn!("{error}"));
        });
}
