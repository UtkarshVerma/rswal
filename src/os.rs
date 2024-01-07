use crate::util::Error;
use std::fs;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path as StdPath, PathBuf};
pub use std::process::{exit, Command};

#[derive(Debug)]
pub struct Path {
    buffer: PathBuf,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let path = shellexpand::tilde(path);

        Path {
            buffer: PathBuf::from(path.as_ref()),
        }
    }

    pub fn join<T: AsRef<StdPath>>(&self, path: T) -> Self {
        Path {
            buffer: self.buffer.join(path),
        }
    }

    pub fn file_stem(&self) -> Option<&str> {
        self.as_ref().file_stem()?.to_str()
    }

    pub fn extension(&self) -> Option<&str> {
        self.as_ref().extension()?.to_str()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.buffer.eq(&other.buffer)
    }
}

impl AsRef<StdPath> for Path {
    fn as_ref(&self) -> &StdPath {
        self.buffer.as_ref()
    }
}

impl From<PathBuf> for Path {
    fn from(path: PathBuf) -> Self {
        Path { buffer: path }
    }
}

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

pub fn read_file(path: &Path) -> Result<String, ReadError> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_to_file(path: &Path, contents: &str) -> Result<(), WriteError> {
    Ok(fs::write(path, contents)?)
}

pub fn read_dir(path: &Path) -> Result<Vec<Path>, ReadDirError> {
    let entries = fs::read_dir(path)?
        .map(|entry| Ok(entry?.path().into()))
        .collect::<Result<Vec<Path>, IoError>>()?;

    Ok(entries)
}

#[test]
fn test_path() {
    let path = Path::new("/");

    let path = path.join("root");
    assert_eq!(path, Path::new("/root"));

    let path = path.join("file.ext");
    assert_eq!(path.file_stem().unwrap(), "file");
    assert_eq!(path.extension().unwrap(), "ext");
}

#[test]
fn test_io() {
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let dir_path = dir.path().to_path_buf().into();
    let files = read_dir(&dir_path).unwrap();
    assert_eq!(files.len(), 0);

    let file: Path = dir_path.join("file.ext");
    write_to_file(&file, "Hello").unwrap();
    assert_eq!(read_file(&file).unwrap(), "Hello");

    let files = read_dir(&dir_path).unwrap();
    assert_eq!(files.len(), 1);

    assert_eq!(file.file_stem().unwrap(), "file");
    assert_eq!(file.extension().unwrap(), "ext");
}
