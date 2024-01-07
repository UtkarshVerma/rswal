use crate::os::{self, Path, ReadError, WriteError};
use crate::renderer::{RenderError, Renderer, Serialize};
use crate::util::Error;
use std::error::Error;

#[derive(Error, Debug)]
enum TemplateErrorKind {
    #[error("could not read template")]
    Read(
        #[from]
        #[source]
        ReadError,
    ),

    #[error("could not render template")]
    Render(
        #[from]
        #[source]
        RenderError,
    ),

    #[error("could not write template")]
    Write(
        #[from]
        #[source]
        WriteError,
    ),
}

#[derive(Error, Debug)]
#[error("{kind} '{template}' ({:?})", kind.source())]
pub struct TemplateError {
    template: String,
    kind: TemplateErrorKind,
}

impl TemplateError {
    fn new(template: String, kind: TemplateErrorKind) -> Self {
        TemplateError { template, kind }
    }
}

#[derive(Debug)]
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

    // TODO: Can the boilerplate be reduced?
    pub fn render<T: Serialize>(&self, renderer: &Renderer<T>) -> Result<(), TemplateError> {
        let contents = os::read_file(&self.source)
            .map_err(|error| TemplateError::new(self.name.to_string(), error.into()))?;

        let rendered = renderer
            .render(&contents)
            .map_err(|error| TemplateError::new(self.name.to_string(), error.into()))?;

        os::write_to_file(&self.target, &rendered)
            .map_err(|error| TemplateError::new(self.name.to_string(), error.into()))?;

        Ok(())
    }
}
