mod cli;
mod colorscheme;
mod config;
mod os;
mod templater;

use std::fs;
use std::process::exit;

use crate::colorscheme::Colorscheme;
use crate::config::Config;
use crate::os::Directories;
use crate::templater::Templater;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(serde::Serialize)]
struct Context {
    pub alpha: u8,
    pub colors: Colorscheme,
}

// TODO: Need to support both yaml and yml
// TODO: Tidy the unwrap hell
fn list_colorschemes(dirs: &Directories) {
    fs::read_dir(&dirs.colorscheme_dir)
        .unwrap()
        .for_each(|entry| {
            let path = entry.unwrap().path();
            if path.extension().unwrap_or_default() != "yaml" {
                return;
            }

            println!("{}", path.file_stem().unwrap().to_str().unwrap());
        });
}

fn main() {
    let dirs = Directories::new().unwrap_or_else(|| {
        eprintln!("error: could not resolve directory paths");
        exit(1);
    });
    dirs.create().unwrap_or_else(|_| {
        eprintln!("error: could not create directories");
        exit(1);
    });

    let args = cli::parse_args();
    if args.list_colorschemes {
        list_colorschemes(&dirs);
        return;
    }

    let config_file = dirs.config_dir.join("config.yaml");
    let contents = fs::read_to_string(config_file).unwrap_or_default();
    let config = Config::new(&contents).unwrap_or_else(|_| {
        eprintln!("error: invalid config syntax");
        exit(1);
    });

    let colorscheme_name = &args.colorscheme;
    let colorscheme_file = dirs
        .colorscheme_dir
        .join(colorscheme_name.to_string() + ".yaml");

    let contents = fs::read_to_string(colorscheme_file).unwrap_or_else(|_| {
        eprintln!(r#"error: colorscheme "{colorscheme_name}" does not exist"#);
        exit(1);
    });
    let colorscheme = Colorscheme::new(&contents).unwrap_or_else(|_| {
        eprintln!("error: invalid colorscheme syntax");
        exit(1);
    });

    let context = Context {
        alpha: args.alpha.unwrap_or(config.alpha),
        colors: colorscheme,
    };
    let templater = Templater::new(&context);
    config.templates.iter().for_each(|template| {
        let template_file = dirs.template_dir.clone().join(&template.source);
        let rendered = fs::read_to_string(template_file)
            .map(|contents| {
                templater
                    .render(&contents)
                    .map_err(|_| {
                        eprintln!(r#"warn: template "{}" has invalid syntax"#, template.source);
                    })
                    .ok()
            })
            .unwrap_or_else(|_| {
                eprintln!(r#"warn: template "{}" does not exist"#, template.source);
                None
            });

        if let Some(rendered) = rendered {
            let target = os::resolve_path(&template.target);
            fs::write(target, rendered).unwrap_or_else(|_| {
                eprintln!(
                    r#"warn: target directory for template "{}" is inaccessible"#,
                    template.source
                );
            });
        }
    });
}
