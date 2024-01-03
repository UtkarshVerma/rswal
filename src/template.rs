use crate::os::{self, Path, ReadError, WriteError};
use crate::renderer::{Error as RenderError, Renderer, Serialize};

pub enum Error {
    ReadFailed(ReadError),
    RenderFailed(RenderError),
    WriteFailed(WriteError),
}

impl From<ReadError> for Error {
    fn from(error: ReadError) -> Self {
        Error::ReadFailed(error)
    }
}

impl From<RenderError> for Error {
    fn from(error: RenderError) -> Self {
        Error::RenderFailed(error)
    }
}

impl From<WriteError> for Error {
    fn from(error: WriteError) -> Self {
        Error::WriteFailed(error)
    }
}

pub struct Template {
    pub name: String,
    pub source: Path,
    pub target: Path,
}

impl Template {
    pub fn new(source: &str, target: &str, template_dir: &Path) -> Self {
        Template {
            name: source.to_string(),
            source: template_dir.join(source),
            target: Path::new(target),
        }
    }

    pub fn render<T: Serialize>(&self, renderer: &Renderer<T>) -> Result<(), Error> {
        let contents = os::read_file(&self.source)?;
        let rendered = renderer.render(&contents)?;

        // TODO: Partial writes?
        Ok(os::write_to_file(&self.target, &rendered)?)
    }
}
