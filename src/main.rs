mod cli;
mod config;
mod os;
mod templater;
mod theme;

use std::fs;
use std::path::PathBuf;
use std::process::exit;

use os::Paths;

use crate::config::Config;
use crate::templater::Templater;
use crate::theme::Theme;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(serde::Serialize)]
struct Context {
    pub variables: serde_yaml::Mapping,
    pub colors: Theme,
}

// TODO: Need to support both yaml and yml
// TODO: Tidy the unwrap hell
fn list_themes(theme_dir: &PathBuf) {
    fs::read_dir(theme_dir).unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        if path.extension().unwrap_or_default() != "yaml" {
            return;
        }

        println!("{}", path.file_stem().unwrap().to_str().unwrap());
    });
}

fn main() {
    let args = cli::parse_args();
    let paths = Paths::new(&args.config_dir);

    // TODO: This should be handled on installation
    paths.create_dirs().unwrap_or_else(|_| {
        eprintln!("error: could not create application directories");
        exit(1);
    });

    if args.list_themes {
        list_themes(&paths.theme_dir);
        return;
    }

    let contents = fs::read_to_string(paths.config_file).unwrap_or_default();
    let config = Config::new(&contents).unwrap_or_else(|_| {
        eprintln!("error: invalid config syntax");
        exit(1);
    });

    let mut variables = config.variables.unwrap_or_default();
    if let Some(cli_vars) = args.variables {
        cli_vars.iter().for_each(|var| {
            let (key, value) = var.split_once(':').unwrap();

            // TODO: Tidy this
            if let Ok(float) = value.parse::<f32>() {
                variables.insert(key.into(), float.into());
            } else {
                variables.insert(key.into(), value.into());
            }
        })
    }

    let theme_name = args.theme.or(config.theme).unwrap_or_else(|| {
        eprintln!("error: no theme specified");
        exit(1);
    });
    let theme_file = paths.theme_dir.join(theme_name.to_string() + ".yaml");
    let contents = fs::read_to_string(theme_file).unwrap_or_else(|_| {
        eprintln!(r#"error: theme "{theme_name}" does not exist"#);
        exit(1);
    });
    let theme = Theme::new(&contents).unwrap_or_else(|error| {
        eprintln!(r#"error: theme "{theme_name}" has invalid syntax"#);
        eprintln!("> {error}");
        exit(1);
    });

    let context = Context {
        // TODO: Remove hardcode
        variables,
        colors: theme,
    };
    let templater = Templater::new(&context);
    if let Some(templates) = config.templates {
        templates.iter().for_each(|template| {
            let template_file = paths.template_dir.join(&template.source);
            let rendered = fs::read_to_string(template_file)
                .map(|contents| {
                    templater
                        .render(&contents)
                        .map_err(|error| {
                            eprintln!(r#"warn: could not render template "{}""#, template.source);
                            eprintln!("> {error}");
                        })
                        .ok()
                })
                .unwrap_or_else(|_| {
                    eprintln!(r#"warn: template "{}" does not exist"#, template.source);
                    None
                });

            if let Some(rendered) = rendered {
                let target = os::resolve_path(&template.target);

                // TODO: Partial writes?
                fs::write(target, rendered).unwrap_or_else(|_| {
                    eprintln!(
                        r#"warn: target directory for template "{}" is inaccessible"#,
                        template.source
                    );
                });
            }
        });
    }
}
