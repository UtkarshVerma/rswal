pub use serde::{de, Deserialize, Deserializer};
pub use serde_yaml::{to_string, Error as ParseError};

/// A generic YAML parser.
pub struct YamlParser;

impl YamlParser {
    pub fn parse<'a, T: Deserialize<'a>>(contents: &'a str) -> Result<T, ParseError> {
        serde_yaml::from_str(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::Value;
    use std::collections::HashMap;

    #[test]
    fn test_parser() {
        let input = "
name: John
age: 21
";
        let parsed: HashMap<String, Value> = YamlParser::parse(input).unwrap();
        assert_eq!(parsed.get("name").unwrap(), "John");
        assert_eq!(parsed.get("age").unwrap(), 21);
    }
}
