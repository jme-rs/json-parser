use crate::parser::Value;

pub struct JsonPrinter {}

impl JsonPrinter {
    pub fn print_json(value: &Value) {
        Self::in_print(value, 0, true);
    }

    fn in_print(value: &Value, depth: usize, line_break: bool) {
        match value {
            Value::Null => print!("null"),
            Value::Bool(b) => print!("{}", b),
            Value::Number(n) => print!("{}", n),
            Value::String(s) => print!("\"{}\"", s),
            Value::Object(object) => {
                println!("{{");
                object.iter().for_each(|(key, value)| {
                    print!("{:indent$}", "", indent = (depth + 1) * 2);
                    print!("\"{}\": ", key);
                    Self::in_print(value, depth + 1, false);
                    println!(",");
                });
                print!("{:indent$}", "", indent = depth * 2);
                print!("}}");
            }
            Value::Array(array) => {
                println!("[");
                array.iter().for_each(|value| {
                    print!("{:indent$}", "", indent = (depth + 1) * 2);
                    Self::in_print(value, depth + 1, false);
                    println!(",");
                });
                print!("{:indent$}", "", indent = depth * 2);
                print!("]");
            }
        }
        if line_break {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn test_print_json() {
        let json = r#"3.14"#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);

        let json = r#""Hello, world.""#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);

        let json = r#"
        {
            "num": 2.71828,
            "name": "exponential"
        }
        "#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);

        let json = r#"
        {
            "num": 2.71828,
            "name": "exponential",
            "other": {
                "num": 3.14,
                "name": "pi"
            }
        }
        "#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);

        let json = r#"
        [
            true,
            false,
            null,
            3.14
        ]
        "#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);

        let json = r#"
        {
            "num": 2.71828,
            "name": [true, false, null, 3.14],
            "other": {
                "num": 3.14,
                "name": "pi"
            }
        }
        "#;
        let value = Parser::new(Lexer::new(json).tokenize().unwrap())
            .parse()
            .unwrap();
        JsonPrinter::print_json(&value);
    }
}
