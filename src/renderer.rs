use crate::colors::Color;
use crate::util::Error;
use handlebars::{
    handlebars_helper, Handlebars, RenderError as HbRenderError,
    RenderErrorReason as HbRenderErrorReason,
};
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
handlebars_helper!(lighten: |color: String, amount: f32| {
    let color = Color::from_hex(&color).unwrap();
    color.lighten(amount).to_hex()
});
handlebars_helper!(darken: |color: String, amount: f32| {
    let color = Color::from_hex(&color).unwrap();
    color.darken(amount).to_hex()
});

#[derive(Error, Debug)]
pub struct RenderError {
    line: Option<usize>,
    column: Option<usize>,
    reason: String,
}

impl From<HbRenderError> for RenderError {
    fn from(error: HbRenderError) -> Self {
        let mut line = error.line_no;
        let mut column = error.column_no;

        let reason = match error.reason() {
            HbRenderErrorReason::TemplateError(error) => {
                if let Some((l, c)) = error.pos() {
                    line = Some(l);
                    column = Some(c);
                }

                error.reason().to_string()
            }
            HbRenderErrorReason::MissingVariable(name) => {
                format!(
                    "missing variable{}",
                    name.as_ref()
                        .map(|name| format!(" '{name}'"))
                        .unwrap_or_default()
                )
            }
            HbRenderErrorReason::ParamTypeMismatchForName(helper, param, param_type) => {
                format!("helper '{helper}' expected '{param_type}' value for param '{param}'")
            }
            HbRenderErrorReason::HelperNotFound(name) => format!("undefined helper '{name}'"),

            reason => reason.to_string(),
        };

        RenderError {
            line,
            column,
            reason,
        }
    }
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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

        write!(f, "{}{location}", self.reason)
    }
}

pub struct Renderer<'a, T> {
    registry: Handlebars<'a>,
    context: &'a T,
}

impl<'a, T: Serialize> Renderer<'a, T> {
    pub fn new(context: &'a T) -> Self {
        let mut registry = Handlebars::new();

        registry.set_strict_mode(true);
        registry.register_helper("hex", Box::new(hex));
        registry.register_helper("div", Box::new(div));
        registry.register_helper("mul", Box::new(mul));
        registry.register_helper("int", Box::new(int));
        registry.register_helper("lighten", Box::new(lighten));
        registry.register_helper("darken", Box::new(darken));

        Renderer { registry, context }
    }

    pub fn render(&self, template: &str) -> Result<String, RenderError> {
        Ok(self.registry.render_template(template, self.context)?)
    }
}

#[test]
fn test_renderer() {
    let context = context!({
        "name": "John",
        "age": 21,
    });
    let renderer = Renderer::new(&context);

    assert_eq!(renderer.render("name: {{name}}").unwrap(), "name: John");
    assert_eq!(renderer.render("age: {{age}}").unwrap(), "age: 21");
}
