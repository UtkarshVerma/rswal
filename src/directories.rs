use crate::os::Path;

pub struct Directories {
    pub template_dir: Path,
    pub theme_dir: Path,
    pub hook_dir: Path,
}

impl Directories {
    pub fn new(config_dir: &str) -> Self {
        let config_dir = Path::new(config_dir);

        Directories {
            template_dir: config_dir.join("templates"),
            theme_dir: config_dir.join("themes"),
            hook_dir: config_dir.join("hooks"),
        }
    }
}
