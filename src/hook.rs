use crate::os::{Command, Path};
use crate::util::Error;
use std::error::Error;
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum HookErrorKind {
    #[error("could not execute hook")]
    Failed(
        #[from]
        #[source]
        IoError,
    ),
}

#[derive(Error, Debug)]
#[error("{kind} '{hook}' ({:?})", kind.source())]
pub struct HookError {
    hook: String,
    kind: HookErrorKind,
}

impl HookError {
    fn new(hook: &str, kind: HookErrorKind) -> Self {
        HookError {
            hook: hook.to_string(),
            kind,
        }
    }
}

pub struct Hook<'a> {
    name: &'a str,
    path: Path,
}

impl<'a> Hook<'a> {
    pub fn new(name: &'a str, hook_dir: &Path) -> Self {
        Hook {
            name,
            path: hook_dir.join(name),
        }
    }

    pub fn execute(&self) -> Result<(), HookError> {
        Command::new(self.path.as_ref())
            .spawn()
            .map_err(|error| HookError::new(self.name, error.into()))?;

        Ok(())
    }
}
