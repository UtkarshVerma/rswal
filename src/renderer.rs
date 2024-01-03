use handlebars::{handlebars_helper, Handlebars, RenderError, RenderErrorReason};
pub use serde::Serialize;
pub use serde_json::json as context;
pub use serde_yaml::Value;
use std::fmt::{Display, Formatter, Result as FmtResult};

handlebars_helper!(hex: |number: u8| format!("{:x}", number));
handlebars_helper!(div: |dividend: f32, divisor: f32| {
    dividend / divisor
});
handlebars_helper!(mul: |multiplicand: f32, multiplier: f32| {
    multiplicand * multiplier
});
handlebars_helper!(int: |number: f32| number as u32);

enum ErrorKind {
    TemplateError,
    InvalidParamType(String),
    MissingVariable(Option<String>),
    ParamNotFound(String, String),
    PartialNotFound(String),
    UndefinedHelper(String),
    UndefinedDecorator(String),
    InvalidLoggingLevel(String),
    Unhandled(RenderError),
}

pub struct Error {
    line: Option<usize>,
    column: Option<usize>,
    kind: ErrorKind,
}

// HACK: The handlebars-rs crate should expose the RenderErrorReason string
impl From<RenderError> for Error {
    fn from(error: RenderError) -> Self {
        let mut line = error.line_no;
        let mut column = error.column_no;

        let kind = match error.reason() {
            RenderErrorReason::TemplateError(error) => {
                if let Some((l, c)) = error.pos() {
                    line = Some(l);
                    column = Some(c);
                }

                ErrorKind::TemplateError
            }
            RenderErrorReason::InvalidParamType(param) => {
                ErrorKind::InvalidParamType(param.to_string())
            }
            RenderErrorReason::MissingVariable(name) => ErrorKind::MissingVariable(name.clone()),
            RenderErrorReason::ParamNotFoundForName(helper, param) => {
                ErrorKind::ParamNotFound(helper.to_string(), param.to_string())
            }
            RenderErrorReason::PartialNotFound(name) => {
                ErrorKind::PartialNotFound(name.to_string())
            }
            RenderErrorReason::HelperNotFound(name) => ErrorKind::UndefinedHelper(name.to_string()),
            RenderErrorReason::InvalidLoggingLevel(level) => {
                ErrorKind::InvalidLoggingLevel(level.to_string())
            }
            RenderErrorReason::DecoratorNotFound(name) => {
                ErrorKind::UndefinedDecorator(name.to_string())
            }
            _ => ErrorKind::Unhandled(error),
            // TODO: Untested
            // RenderErrorReason::ParamNotFoundForIndex(_, _) => todo!(),
            // RenderErrorReason::ParamTypeMismatchForName(x, y, z) => todo!(),
            // RenderErrorReason::HashTypeMismatchForName(_, _, _) => todo!(),
            // RenderErrorReason::CannotIncludeSelf => todo!(),
            // RenderErrorReason::BlockContentRequired => todo!(),
            // RenderErrorReason::InvalidJsonPath(_) => todo!(),
            // RenderErrorReason::InvalidJsonIndex(_) => todo!(),
            // RenderErrorReason::SerdeError(_) => todo!(),
            // RenderErrorReason::NestedError(_) => todo!(),
        };

        Error { line, column, kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match &self.kind {
            ErrorKind::TemplateError => "invalid syntax".to_string(),
            ErrorKind::InvalidParamType(param) => {
                format!("invalid param type, expected '{param}' ")
            }
            ErrorKind::MissingVariable(name) => {
                let name = name
                    .as_ref()
                    .map(|name| format!(" '{}'", name.clone()))
                    .unwrap_or_default();
                format!("missing variable{name}")
            }
            ErrorKind::ParamNotFound(helper, param) => {
                format!("invalid '{param}' passed to '{helper}'")
            }
            ErrorKind::PartialNotFound(name) => format!("partial '{name}' not found"),
            ErrorKind::UndefinedHelper(name) => format!("undefined helper '{name}'"),
            ErrorKind::InvalidLoggingLevel(level) => {
                format!("invalid logging level '{level}'")
            }
            ErrorKind::UndefinedDecorator(name) => format!("undefined decorator '{name}'"),
            ErrorKind::Unhandled(error) => format!("unhandled error occured ({error})"),
        };

        let location = self
            .line
            .map(|l| {
                format!(
                    " at line {l}{}",
                    self.column
                        .map(|c| format!(" column {c}"))
                        .unwrap_or_default()
                )
            })
            .unwrap_or_default();

        write!(f, "{message}{location}")
    }
}

pub struct Renderer<'a, T> {
    registry: Handlebars<'a>,
    data: &'a T,
}

impl<'a, T: Serialize> Renderer<'a, T> {
    pub fn new(data: &'a T) -> Self {
        let mut registry = Handlebars::new();

        registry.set_strict_mode(true);
        registry.register_helper("hex", Box::new(hex));
        registry.register_helper("div", Box::new(div));
        registry.register_helper("mul", Box::new(mul));
        registry.register_helper("int", Box::new(int));

        Renderer { registry, data }
    }

    pub fn render(&self, template: &str) -> Result<String, Error> {
        Ok(self.registry.render_template(template, self.data)?)
    }
}
