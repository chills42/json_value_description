extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_json::{Value};

fn json_type(val: &Value) -> &'static str {
    match val {
        Value::Object(_data) => { "\"Object\"" },
        Value::Bool(_) => { "\"Bool\"" },
        Value::Number(_) => { "\"Number\"" },
        Value::String(_) => { "\"String\"" },
        Value::Array(_) => { "\"Array\"" },
        Value::Null => { "\"Null\"" },
    }
}

pub fn json_object_description(json_value: &Value) -> String {
    if let Some(object) = json_value.as_object() {
        let mut obj_key = vec![];
        for (key, value) in object {
            let field_description = if value.is_object() {
                json_object_description(value)
            } else {
                json_type(value).into()
            };
            obj_key.push(format!("\"{}\": {}", key, field_description));
        };
        format!("{{ {} }}", obj_key.join(", "))
    } else {
        json_type(json_value).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::json_object_description;

    use serde_json::{Value};

    const EMPTY: &'static str = r#"{}"#;
    const ARR: &'static str = r#"[]"#;
    const COMPLEX: &'static str = r#"{ "hello": "world", "count": 42, "question": true, "nest": { "one": { "two": "three", "example": null } }}"#;

    #[test]
    fn empty() {
        let data: Value = serde_json::from_str(EMPTY).unwrap();
        assert_eq!(self::json_object_description(&data), "{  }");
    }

    #[test]
    fn array() {
        let data: Value = serde_json::from_str(ARR).unwrap();
        assert_eq!(self::json_object_description(&data), "\"Array\"");
    }

    #[test]
    fn complex() {
        let data: Value = serde_json::from_str(COMPLEX).unwrap();
        assert_eq!(self::json_object_description(&data), "{ \"count\": \"Number\", \"hello\": \"String\", \"nest\": { \"one\": { \"example\": \"Null\", \"two\": \"String\" } }, \"question\": \"Bool\" }");
    }
}
