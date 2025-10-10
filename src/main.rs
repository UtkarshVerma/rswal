mod cli;
mod color;
mod config;
mod directories;
mod hook;
mod logger;
mod os;
mod renderer;
mod template;
mod theme;
mod yaml_parser;

use cli::Args;
use config::{Config, ConfigError};
use directories::Directories;
use hook::Hook;
use logger::{error, Logger};
use os::{Path, ReadDirError};
use renderer::{context, Renderer, Value};
use std::{collections::HashMap, process::ExitCode};
use template::Template;
use theme::{Theme, ThemeError};
use thiserror::Error;

#[derive(Debug, Error)]
enum ListThemesError {
    #[error("could not read directory -> {0}")]
    ReadDir(#[from] ReadDirError),
}

#[derive(Debug, Error)]
enum AppError {
    #[error("could not read config -> {0}")]
    Config(#[from] ConfigError),

    #[error("could not list themes -> {0}")]
    ListThemes(#[from] ListThemesError),

    #[error("no theme specified")]
    NoThemeSpecified,

    #[error("invalid theme -> {0}")]
    Theme(#[from] ThemeError),
}

fn main() -> ExitCode {
    Logger::init();

    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            error!("{e}");
            ExitCode::FAILURE
        }
    }
}

// TODO: Think of saving colorscheme to sequences for cross-terminal support
fn run() -> Result<(), AppError> {
    let cli_args = Args::parse();
    let config_dir = &cli_args.config_dir;
    let dirs = Directories::new(&config_dir);

    if cli_args.list_themes {
        return Ok(list_themes(&dirs.theme_dir)?);
    }

    let config = Config::new(&config_dir)?;
    let theme_name = cli_args
        .theme
        .or(config.theme)
        .ok_or(AppError::NoThemeSpecified)?;
    let theme = Theme::new(&theme_name, &dirs.theme_dir)?;

    let mut variables = HashMap::new();
    if let Some(config_vars) = config.variables {
        variables.extend(config_vars.into_iter());
    }
    if let Some(cli_vars) = cli_args.variables {
        variables.extend(cli_vars.into_iter());
    }

    let templates = config.templates.unwrap_or_default();
    let templates = templates
        .iter()
        .map(|template| Template::new(&template.source, &template.target, &dirs.template_dir))
        .collect::<Vec<Template>>();

    let hooks = cli_args.hooks.unwrap_or(config.hooks.unwrap_or_default());
    let hooks = hooks
        .iter()
        .map(|hook| Hook::new(hook, &dirs.hook_dir))
        .collect::<Vec<Hook>>();

    render_templates(&templates, &theme, &variables);
    execute_hooks(&hooks, &variables);

    Ok(())
}

fn list_themes(theme_dir: &Path) -> Result<(), ListThemesError> {
    let files = os::read_dir(theme_dir)?;

    let mut themes = files
        .iter()
        .filter(|file| file.extension().unwrap_or_default() == "yaml")
        .map(|file| {
            file.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
        })
        .collect::<Vec<&str>>();
    themes.sort();

    themes.into_iter().for_each(|theme| println!("{theme}"));

    Ok(())
}

fn render_templates(templates: &[Template], theme: &Theme, variables: &HashMap<String, Value>) {
    let context = context!({
        "variables": variables,
        "colors": theme,
    });
    let renderer = Renderer::new(&context);

    templates.iter().for_each(|template| {
        template
            .render(&renderer)
            .unwrap_or_else(|err| error!("could not render template '{}' -> {err}", template.name));
    });
}

fn execute_hooks(hooks: &[Hook], variables: &HashMap<String, Value>) {
    let variables = variables
        .iter()
        .map(|(k, v)| (k.as_str(), yaml_parser::to_string(v).unwrap_or_default()))
        .collect::<Vec<(&str, String)>>();

    hooks.iter().for_each(|hook| {
        let output = hook.execute(&variables).unwrap_or_else(|err| {
            error!("could not execute hook '{}': {err}", hook.name);

            "".to_string()
        });

        if !output.is_empty() {
            println!("{output}");
        }
    });
}
