use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::{Path as StdPath, PathBuf};
pub use std::process::exit;

pub struct Path {
    buffer: PathBuf,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let path = shellexpand::tilde(path).to_string();

        Path {
            buffer: PathBuf::from(&path),
        }
    }

    pub fn join<T: AsRef<StdPath>>(&self, path: T) -> Self {
        Path {
            buffer: self.buffer.join(path),
        }
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

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.buffer.display())
    }
}

#[derive(Debug)]
pub enum ReadError {
    FileNotFound,
    PermissionDenied,
    InvalidData,
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

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self {
            ReadError::FileNotFound => "file not found".to_string(),
            ReadError::PermissionDenied => "permission denied".to_string(),
            ReadError::InvalidData => "file contents are not valid utf-8".to_string(),
            ReadError::Other(error) => format!("{error}"),
        };

        write!(f, "{reason}")
    }
}

#[derive(Debug)]
pub enum WriteError {
    DirectoryDoesNotExist,
    PermissionDenied,
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

impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self {
            WriteError::DirectoryDoesNotExist => "directory does not exist".to_string(),
            WriteError::PermissionDenied => "permission denied".to_string(),
            WriteError::Other(error) => format!("{error}"),
        };

        write!(f, "{message}")
    }
}

#[derive(Debug)]
pub enum ReadDirError {
    DirectoryDoesNotExist,
    PermissionDenied,
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

    // TODO: This has to be thought out
    // pub fn create(&self) -> io::Result<()> {
    // fs::create_dir_all(&self.theme_dir)?;
    // fs::create_dir_all(&self.template_dir)?;
    // fs::create_dir_all(&self.hook_dir)?;
    // Ok(())
    // }
}

pub fn read_file(path: &Path) -> Result<String, ReadError> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_to_file(path: &Path, contents: &str) -> Result<(), WriteError> {
    Ok(fs::write(path, contents)?)
}

pub fn read_dir(path: &Path) -> Result<Vec<Path>, ReadDirError> {
    Ok(fs::read_dir(path)?
        .map(|entry| entry.unwrap().path().into())
        .collect())
}

// TODO: Write tests
