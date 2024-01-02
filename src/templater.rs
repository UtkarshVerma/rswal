use std::fmt;

use handlebars::{handlebars_helper, Handlebars};
use serde::Serialize;

handlebars_helper!(hex: |number: u8| format!("{:x}", number));
handlebars_helper!(div: |dividend: f32, divisor: f32| {
    dividend / divisor
});
handlebars_helper!(mul: |multiplicand: f32, multiplier: f32| {
    multiplicand * multiplier
});
handlebars_helper!(int: |number: f32| number as u32);

#[derive(Debug, Clone)]
pub struct RenderError {
    line: Option<usize>,
    column: Option<usize>,

    reason: String,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(column)) => write!(f, "({}, {}): {}", line, column, self.reason),
            _ => write!(f, "{}", self.reason),
        }
    }
}

pub struct Templater<'a, T> {
    registry: Handlebars<'a>,
    data: &'a T,
}

impl<'a, T> Templater<'a, T> {
    pub fn new(data: &'a T) -> Self {
        let mut registry = Handlebars::new();
        registry.register_helper("hex", Box::new(hex));
        registry.register_helper("div", Box::new(div));
        registry.register_helper("mul", Box::new(mul));
        registry.register_helper("int", Box::new(int));

        Templater { registry, data }
    }
}

impl<'a, T: Serialize> Templater<'a, T> {
    pub fn render(&self, template: &str) -> Result<String, RenderError> {
        self.registry
            .render_template(template, self.data)
            .map_err(|error| RenderError {
                line: error.line_no,
                column: error.column_no,
                reason: error.reason().to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Data {
        name: String,
        age: u8,
    }

    #[test]
    fn test_render() {
        let data = Data {
            name: "John".to_string(),
            age: 20,
        };
        let templater = Templater::new(&data);

        let rendered = templater.render("{{name}} is {{age}} years old.");
        assert_eq!(rendered.unwrap(), "John is 20 years old.");

        let rendered = templater.render("{{hex age}}");
        assert_eq!(rendered.unwrap(), "14");

        let rendered = templater.render("{{div 20 5}}");
        assert_eq!(rendered.unwrap(), "4.0");

        let rendered = templater.render("{{mul 20 5}}");
        assert_eq!(rendered.unwrap(), "100.0");

        let rendered = templater.render("{{int (mul 20 5)}}");
        assert_eq!(rendered.unwrap(), "100");
    }
}
