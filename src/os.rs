use std::fs;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
pub use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("file not found")]
    FileNotFound,

    #[error("permission denied")]
    PermissionDenied,

    #[error("file contents are not valid utf-8")]
    InvalidData,

    #[error("{0}")]
    Other(IoError),
}

impl From<IoError> for ReadError {
    fn from(error: IoError) -> Self {
        match error.kind() {
            IoErrorKind::NotFound => ReadError::FileNotFound,
            IoErrorKind::PermissionDenied => ReadError::PermissionDenied,
            IoErrorKind::InvalidData => ReadError::InvalidData,
            _ => ReadError::Other(error),
        }
    }
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("directory does not exist")]
    DirectoryDoesNotExist,

    #[error("permission denied")]
    PermissionDenied,

    #[error("{0}")]
    Other(IoError),
}

impl From<IoError> for WriteError {
    fn from(error: IoError) -> Self {
        match error.kind() {
            IoErrorKind::NotFound => WriteError::DirectoryDoesNotExist,
            IoErrorKind::PermissionDenied => WriteError::PermissionDenied,
            _ => WriteError::Other(error),
        }
    }
}

#[derive(Error, Debug)]
pub enum ReadDirError {
    #[error("directory does not exist")]
    DirectoryDoesNotExist,

    #[error("permission denied")]
    PermissionDenied,

    #[error("{0}")]
    Other(IoError),
}

impl From<IoError> for ReadDirError {
    fn from(error: IoError) -> Self {
        match error.kind() {
            IoErrorKind::NotFound => ReadDirError::DirectoryDoesNotExist,
            IoErrorKind::PermissionDenied => ReadDirError::PermissionDenied,
            _ => ReadDirError::Other(error),
        }
    }
}

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String, ReadError> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_to_file<T: AsRef<Path>>(path: T, contents: &str) -> Result<(), WriteError> {
    Ok(fs::write(path, contents)?)
}

pub fn read_dir<T: AsRef<Path>>(path: T) -> Result<Vec<PathBuf>, ReadDirError> {
    let entries = fs::read_dir(path)?
        .map(|entry| Ok(entry?.path()))
        .collect::<Result<Vec<_>, IoError>>()?;

    Ok(entries)
}

pub fn resolve_path(path: &Path) -> Option<PathBuf> {
    match path.strip_prefix("~") {
        Ok(subpath) => home::home_dir().map(|home| home.join(subpath)),
        Err(_) => Some(path.to_path_buf()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_resolution() {
        let path = Path::new("~/.config");
        assert_eq!(
            resolve_path(path).unwrap(),
            home::home_dir().unwrap().join(".config")
        );
    }

    #[test]
    fn io() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let dir_path = dir.path();
        let files = read_dir(&dir_path).unwrap();
        assert_eq!(files.len(), 0);

        let file = dir_path.join("file.ext");
        write_to_file(&file, "Hello").unwrap();
        assert_eq!(read_file(&file).unwrap(), "Hello");

        let files = read_dir(&dir_path).unwrap();
        assert_eq!(files.len(), 1);

        assert_eq!(file.file_stem().unwrap(), "file");
        assert_eq!(file.extension().unwrap(), "ext");
    }
}
