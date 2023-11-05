use std::fmt::Display;

use aoc_core::AoCError;
use serde_json::{Map, Value};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Value);

fn count_json(json: &Value, ignore_red: bool) -> i64 {
    match json {
        Value::Number(num) => num.as_i64().unwrap_or(0),
        Value::Array(array) => array
            .iter()
            .map(|json| count_json(json, ignore_red))
            .sum(),
        Value::Object(object) => {
            if ignore_red && has_red(object) {
                return 0;
            }
            object
                .values()
                .map(|json| count_json(json, ignore_red))
                .sum()
        }
        _ => 0,
    }
}

fn has_red(obj: &Map<String, Value>) -> bool {
    obj.values().any(|val| match val {
        Value::String(string) => string == "red",
        _ => false,
    })
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let json: Value = serde_json::from_str(&input).map_err(|_| AoCError::Parsing)?;
        Ok(Self(json))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(count_json(&self.0, false))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(count_json(&self.0, true))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "[1,2,3]";
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "6");
        let input = r#"{"a":2,"b":4}"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "6");
        let input = r#"[[[3]]]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "3");
        let input = r#"{"a":{"b":4},"c":-1}"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "3");
        let input = r#"{"a":[-1,1]}"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "0");
        let input = r#"[-1,{"a":1}]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "0");
        let input = r#"[]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "0");
        let input = r#"{}"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_1().unwrap().to_string(), "0");
    }

    #[test]
    fn part_2() {
        let input = r#"[1,2,3]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "6");
        let input = r#"[1,{"c":"red","b":2},3]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "4");
        let input = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "0");
        let input = r#"[1,"red",5]"#;
        let data = Data::try_new(input).unwrap();
        assert_eq!(data.part_2().unwrap().to_string(), "6");
    }
}
