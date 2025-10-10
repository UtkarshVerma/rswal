use crate::os::{Path, PathBuf};

const TEMPLATE_DIR: &str = "templates";
const THEME_DIR: &str = "themes";
const HOOK_DIR: &str = "hooks";

pub struct Directories {
    pub template_dir: PathBuf,
    pub theme_dir: PathBuf,
    pub hook_dir: PathBuf,
}

impl Directories {
    pub fn new(config_dir: &Path) -> Self {
        Directories {
            template_dir: config_dir.join(TEMPLATE_DIR),
            theme_dir: config_dir.join(THEME_DIR),
            hook_dir: config_dir.join(HOOK_DIR),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        let config_dir = "/root/.config/foo/";
        let config_dir_path = Path::new(config_dir);
        let dirs = Directories::new(config_dir_path);

        assert_eq!(
            dirs.template_dir.to_str().unwrap(),
            config_dir.to_string() + TEMPLATE_DIR
        );
        assert_eq!(
            dirs.theme_dir.to_str().unwrap(),
            config_dir.to_string() + THEME_DIR
        );
        assert_eq!(
            dirs.hook_dir.to_str().unwrap(),
            config_dir.to_string() + HOOK_DIR
        );
    }
}
