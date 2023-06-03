
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum Json {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
    None,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} strings", &args[0]);
        return Ok(());
    }

    let raw = &args[1];
    let json: Json = serde_json::from_str(&raw)?;
    let text = serde_json::to_string(&json)?;

    println!("input: {}", &raw);
    println!("parse: {:?}", &json);

    assert_eq!(&json, &serde_json::from_str(&text)?);
    println!("json: {}", &text);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let s = r#""abc""#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::String("abc".to_string()), j);
    }

    #[test]
    fn test_int() {
        let s = r#"1"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Int(1), j);
    }

    #[test]
    fn test_float() {
        let s = r#"0.1"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Float(0.1), j);
    }

    #[test]
    fn test_bool() {
        let s = r#"true"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Bool(true), j);
    }

    #[test]
    fn test_null() {
        let s = r#"null"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::None, j);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let s = r#"abcd"#;
        let _: Json = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn test_array() {
         let s = r#"[0, 1]"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Array(vec![Json::Int(0), Json::Int(1)]), j);
    }

    #[test]
    fn test_dict() {
        let s = r#"{"x": 1, "y": "abcd"}"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::String("abcd".to_string()))])), j);
    }

    #[test]
    fn test_any_json() {
        let s = r#"[{"x": 1, "y": "abcd", "z": {} , "a": []}]"#;
        let j: Json = serde_json::from_str(&s).unwrap();
        assert_eq!(Json::Array(vec![Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::String("abcd".to_string())), ("z".to_string(), Json::Object(HashMap::new())), ("a".to_string(), Json::Array(Vec::new()))]))]), j);
    }

    #[test]
    fn test_poly() {
        let s1 = r#""abc""#;
        let s2 = r#"1"#;
        let s3 = r#"0.1"#;
        let s4 = r#"true"#;
        let s5 = r#"null"#;
        let s6 = r#"[0, 1]"#;
        let s7 = r#"{"x": 1, "y": "abcd"}"#;
        let s8 = r#"[{"x": 1, "y": "abcd", "z": {} , "a": []}]"#;

        let j1: Json = serde_json::from_str(&s1).unwrap();
        let j2: Json = serde_json::from_str(&s2).unwrap();
        let j3: Json = serde_json::from_str(&s3).unwrap();
        let j4: Json = serde_json::from_str(&s4).unwrap();
        let j5: Json = serde_json::from_str(&s5).unwrap();
        let j6: Json = serde_json::from_str(&s6).unwrap();
        let j7: Json = serde_json::from_str(&s7).unwrap();
        let j8: Json = serde_json::from_str(&s8).unwrap();

        let mut a = Vec::new();

        a.push(j1);
        a.push(j2);
        a.push(j3);
        a.push(j4);
        a.push(j5);
        a.push(j6);
        a.push(j7);
        a.push(j8);

        let b = serde_json::to_string(&a).unwrap();

        println!("{:?}", a);
        println!("{}", b);
    }
}
