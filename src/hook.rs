use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitStatus;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("non zero exit status: {0}")]
    NonZeroStatus(ExitStatus),

    #[error("does not exist")]
    DoesNotExist,

    #[error("permission denied")]
    PermissionDenied,

    #[error("{0}")]
    Other(IoError),
}

pub struct Hook<'a> {
    pub name: &'a str,
    path: PathBuf,
}

impl<'a> Hook<'a> {
    pub fn new(name: &'a str, hook_dir: &Path) -> Self {
        Hook {
            name,
            path: hook_dir.join(name),
        }
    }

    pub fn execute<K: ToString, V: ToString>(
        &self,
        variables: &[(K, V)],
    ) -> Result<String, HookError> {
        let variables = variables
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string().to_ascii_uppercase().replace("-", "_"),
                    v.to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();

        let output =
            Command::new(&self.path)
                .envs(variables)
                .output()
                .map_err(|error| match error.kind() {
                    IoErrorKind::NotFound => HookError::DoesNotExist,
                    IoErrorKind::PermissionDenied => HookError::PermissionDenied,
                    _ => HookError::Other(error),
                })?;
        let status = output.status;
        if !status.success() {
            return Err(HookError::NonZeroStatus(status));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os;

    #[test]
    fn execute() {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        use tempfile::tempdir;

        let hook_dir = tempdir().unwrap();
        let hook_dir_path = hook_dir.path();

        let hook = "hook.sh";
        let hook_file = hook_dir_path.join(hook);
        os::write_to_file(
            &hook_file,
            "#!/bin/sh

echo ${NAME}",
        )
        .unwrap();
        fs::set_permissions(&hook_file, fs::Permissions::from_mode(0o755)).unwrap();

        let hook = Hook::new(hook, &hook_dir_path);
        let variables = vec![("name", "John")];
        assert_eq!(hook.execute(&variables).unwrap(), "John\n");

        os::write_to_file(
            &hook_file,
            "#!/bin/sh

echo ${FIRST_NAME}",
        )
        .unwrap();
        let variables = vec![("first-name", "John")];
        assert_eq!(hook.execute(&variables).unwrap(), "John\n");
    }

    #[test]
    fn path() {
        let hook_dir_path = Path::new("/root/.config/foo/hooks");
        let hook = Hook::new("hook", &hook_dir_path);

        assert_eq!(hook.path, Path::new("/root/.config/foo/hooks/hook"));
    }
}
