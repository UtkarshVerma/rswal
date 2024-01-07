pub use serde::Deserialize;
pub use serde_yaml::Error as ParseError;

pub struct Parser;

impl Parser {
    pub fn parse<'a, T: Deserialize<'a>>(contents: &'a str) -> Result<T, ParseError> {
        serde_yaml::from_str(contents)
    }
}

#[test]
fn test_parser() {
    use serde_yaml::Value;
    use std::collections::HashMap;

    let input = "
name: John
age: 21
";
    let parsed: HashMap<String, Value> = Parser::parse(input).unwrap();
    assert_eq!(parsed.get("name").unwrap(), "John");
    assert_eq!(parsed.get("age").unwrap(), 21);
}
