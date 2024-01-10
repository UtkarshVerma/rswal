use crate::os::{self, Path, PathBuf, ReadError, WriteError};
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
    fn new(template: &str, kind: TemplateErrorKind) -> Self {
        TemplateError {
            template: template.to_string(),
            kind,
        }
    }
}

#[derive(Debug)]
pub struct Template<'a> {
    pub name: &'a str,
    pub source: PathBuf,
    pub target: &'a Path,
}

impl<'a> Template<'a> {
    pub fn new(source: &'a str, target: &'a Path, template_dir: &Path) -> Self {
        Template {
            name: source,
            source: template_dir.join(source),
            target,
        }
    }

    pub fn render<T: Serialize>(&self, renderer: &Renderer<T>) -> Result<(), TemplateError> {
        let contents = os::read_file(&self.source)
            .map_err(|error| TemplateError::new(self.name, error.into()))?;

        let rendered = renderer
            .render(&contents)
            .map_err(|error| TemplateError::new(self.name, error.into()))?;

        os::write_to_file(&self.target, &rendered)
            .map_err(|error| TemplateError::new(self.name, error.into()))?;

        Ok(())
    }
}

#[test]
fn test_renderer() {
    use crate::renderer::context;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    let source = "source";
    let source_path = dir_path.join(source);
    os::write_to_file(&source_path, "name: {{name}}").unwrap();

    let target = dir_path.join("target");
    let template = Template::new(source, &target, &dir_path);

    let context = context!({
        "name": "John"
    });
    let renderer = Renderer::new(&context);
    template.render(&renderer).unwrap();

    assert_eq!(os::read_file(&target).unwrap(), "name: John");
}
