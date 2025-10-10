use crate::os::{self, Path, PathBuf, ReadError, WriteError};
use crate::renderer::{RenderError, Renderer, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("read failed -> {0}")]
    Read(#[from] ReadError),

    #[error("render failed -> {0}")]
    Render(#[from] RenderError),

    #[error("write failed -> {0}")]
    Write(#[from] WriteError),
}

#[derive(Debug)]
pub struct Template<'a> {
    pub name: &'a str,
    source: PathBuf,
    target: &'a Path,
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
        let contents = os::read_file(&self.source)?;
        let rendered = renderer.render(&contents)?;
        os::write_to_file(&self.target, &rendered)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::context;
    use tempfile::tempdir;

    #[test]
    fn render() {
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
}
