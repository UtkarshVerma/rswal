mod cli;
mod colors;
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
use crate::os::{ExitCode, Path, ReadDirError};
use crate::renderer::{context, Renderer, Value};
use crate::template::Template;
use crate::theme::Theme;
use crate::util::HashMap;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

// TODO: Think of saving colorscheme to sequences for cross-terminal support
fn main() -> ExitCode {
    Logger::init();

    run().map(|_| ExitCode::SUCCESS).unwrap_or_else(|err| {
        error!("{err}");

        ExitCode::FAILURE
    })
}

fn run() -> Result<(), String> {
    let cli_args = Args::parse();
    let config_dir = &cli_args.config_dir;
    let dirs = Directories::new(&config_dir);

    if cli_args.list_themes {
        list_themes(&dirs.theme_dir)?;

        return Ok(());
    }

    let config = Config::new(&config_dir).map_err(|err| err.to_string())?;

    let theme_name = cli_args
        .theme
        .or(config.theme)
        .ok_or("no theme specified")?;
    let theme = Theme::new(&theme_name, &dirs.theme_dir).map_err(|err| err.to_string())?;

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

    render_templates(&templates, &theme, &variables)?;
    execute_hooks(&hooks, &variables);

    Ok(())
}

fn list_themes(theme_dir: &Path) -> Result<(), String> {
    let files = os::read_dir(theme_dir).map_err(|err| match err {
        ReadDirError::DirectoryDoesNotExist => "theme directory does not exist".to_string(),
        ReadDirError::PermissionDenied => "permission denied for theme directory".to_string(),
        ReadDirError::Other(error) => error.to_string(),
    })?;

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

    for theme in themes {
        println!("{theme}");
    }

    Ok(())
}

fn render_templates(
    templates: &[Template],
    theme: &Theme,
    variables: &HashMap<String, Value>,
) -> Result<(), String> {
    let context = context!({
        "variables": variables,
        "colors": theme,
    });
    let renderer = Renderer::new(&context);

    templates.iter().for_each(|template| {
        template
            .render(&renderer)
            .unwrap_or_else(|err| warn!("{err}"))
    });

    Ok(())
}

fn execute_hooks(hooks: &[Hook], variables: &HashMap<String, Value>) {
    let variables = variables
        .iter()
        .map(|(k, v)| (k.as_str(), parser::to_string(v).unwrap_or_default()))
        .collect::<Vec<(&str, String)>>();

    hooks.iter().for_each(|hook| {
        let output = hook.execute(&variables).unwrap_or_else(|err| {
            warn!("{err}");

            "".to_string()
        });

        if !output.is_empty() {
            println!("{output}");
        }
    });
}
