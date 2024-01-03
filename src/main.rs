mod cli;
mod config;
mod logger;
mod os;
mod parser;
mod renderer;
mod template;
mod theme;

use crate::cli::Args;
use crate::config::{Config, Error as ConfigError};
use crate::logger::{error, warn, Logger};
use crate::os::{exit, Directories, Path, ReadDirError};
use crate::renderer::{context, Renderer};
use crate::template::{Error as TemplateError, Template};
use crate::theme::{Error as ThemeError, Theme};

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    Logger::init();
    let cli_args = Args::parse();
    let dirs = Directories::new(&cli_args.config_dir);

    if cli_args.list_themes {
        list_themes(&dirs.theme_dir).unwrap_or_else(|error| {
            let message = match error {
                ReadDirError::DirectoryDoesNotExist => "theme directory does not exist".to_string(),
                ReadDirError::PermissionDenied => {
                    "permission denied for theme directory".to_string()
                }
                ReadDirError::Other(error) => format!("{error}"),
            };

            error!("could not list themes ({message})")
        });
        return;
    }

    let config_file = Path::new(&cli_args.config_dir).join("config.yaml");
    let config = Config::new(&config_file).unwrap_or_else(|error| {
        match error {
            ConfigError::ParseFailed(error) => error!("could not parse config ({error})"),
            ConfigError::ReadFailed(error) => error!("could not read config ({error})"),
        }
        exit(1);
    });

    let theme_name = cli_args.theme.or(config.theme).unwrap_or_else(|| {
        error!("no theme specified");
        exit(1);
    });
    let theme = read_theme(&theme_name, &dirs.theme_dir).unwrap_or_else(|error| {
        match error {
            ThemeError::ReadFailed(error) => {
                error!("could not read theme '{theme_name}' ({error})")
            }
            ThemeError::ParseFailed(error) => {
                error!("could not parse theme '{theme_name}' ({error})")
            }
        }
        exit(1);
    });

    let mut variables = config.variables.unwrap_or_default();
    variables.extend(cli_args.variables.into_iter().map(|v| (v.name, v.value)));

    let context = context!({
        "variables": variables,
        "colors": theme,
    });
    let renderer = Renderer::new(&context);

    let templates = config.templates.unwrap_or_default();
    templates
        .iter()
        .map(|template| Template::new(&template.source, &template.target, &dirs.template_dir))
        .for_each(|ref template| {
            if let Err(error) = template.render(&renderer) {
                match error {
                    TemplateError::ReadFailed(error) => {
                        warn!("could not read template '{}' ({error})", template.name);
                    }
                    TemplateError::RenderFailed(error) => {
                        warn!("could not render template '{}' ({error})", template.name);
                    }
                    TemplateError::WriteFailed(error) => {
                        warn!(
                            "could not write template '{}' to '{}' ({error})",
                            template.name, template.target
                        )
                    }
                }
            }
        });
}

fn list_themes(theme_dir: &Path) -> Result<(), ReadDirError> {
    let files = os::read_dir(theme_dir)?;
    let mut themes: Vec<&str> = files
        .iter()
        .map(|file| file.as_ref())
        .filter(|file| file.extension().unwrap_or_default() == "yaml")
        .map(|file| file.file_stem().unwrap_or_default().to_str().unwrap())
        .collect();
    themes.sort();

    for theme in themes {
        println!("{theme}");
    }

    Ok(())
}

fn read_theme(name: &str, dir: &Path) -> Result<Theme, ThemeError> {
    let theme_file = dir.join(format!("{name}.yaml"));

    Theme::new(&theme_file)
}
