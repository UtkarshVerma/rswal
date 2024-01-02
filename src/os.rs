use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Paths {
    pub config_file: PathBuf,
    pub config_dir: PathBuf,
    pub template_dir: PathBuf,
    pub theme_dir: PathBuf,
}

impl Paths {
    pub fn new(config_dir: &str) -> Self {
        let config_dir = Path::new(config_dir);
        Paths {
            config_dir: config_dir.to_path_buf(),
            config_file: config_dir.join("config.yaml"),
            template_dir: config_dir.join("templates"),
            theme_dir: config_dir.join("themes"),
        }
    }

    pub fn create_dirs(&self) -> io::Result<()> {
        fs::create_dir_all(&self.theme_dir)?;
        fs::create_dir_all(&self.template_dir)?;

        Ok(())
    }
}

pub fn resolve_path(path: &str) -> String {
    shellexpand::tilde(path).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_resolve_path() {
        // TODO: Add test for path resolution
    }
}
