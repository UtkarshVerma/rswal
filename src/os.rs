use directories::BaseDirs;
use std::path::PathBuf;
use std::{fs, io};

use crate::BINARY_NAME;

pub struct Directories {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub template_dir: PathBuf,
    pub colorscheme_dir: PathBuf,
}

impl Directories {
    pub fn new() -> Option<Self> {
        BaseDirs::new().map(|base_dirs| {
            let config_dir = base_dirs.config_dir().join(BINARY_NAME);

            Directories {
                config_dir: config_dir.clone(),
                cache_dir: base_dirs.cache_dir().join(BINARY_NAME),
                template_dir: config_dir.clone().join("templates"),
                colorscheme_dir: config_dir.join("colorschemes"),
            }
        })
    }

    pub fn create(&self) -> io::Result<()> {
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(&self.colorscheme_dir)?;
        fs::create_dir_all(&self.template_dir)?;

        Ok(())
    }
}

pub fn resolve_path(path: &str) -> String {
    shellexpand::tilde(path).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() {
        let dirs = Directories::new();
        let config_dir = dirs.unwrap().config_dir;

        // TODO: A better way to test this?
        assert!(config_dir.ends_with(BINARY_NAME));
    }
}
