use crate::os::{self, ExecuteError, ExitStatus, Path, PathBuf};
use crate::util::Error;
use std::error::Error;

#[derive(Error, Debug)]
pub enum HookErrorKind {
    #[error("could not execute hook")]
    ExecuteFailed(
        #[from]
        #[source]
        ExecuteError,
    ),

    #[error("could not successfully execute hook")]
    // TODO: Display the exit status
    NonZeroStatus(ExitStatus),
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

        let (output, status) = os::execute(&self.path, [], &variables)
            .map_err(|error| HookError::new(self.name, error.into()))?;
        if !status.success() {
            return Err(HookError::new(
                self.name,
                HookErrorKind::NonZeroStatus(status),
            ));
        }

        Ok(String::from_utf8_lossy(&output).into())
    }
}

#[test]
fn test_execution() {
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
fn test_path() {
    let hook_dir_path = Path::new("/root/.config/foo/hooks");
    let hook = Hook::new("hook", &hook_dir_path);

    assert_eq!(hook.path, Path::new("/root/.config/foo/hooks/hook"));
}
