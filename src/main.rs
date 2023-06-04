
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;

type JsonArray = Vec<Json>;
type JsonObject = HashMap<String, Json>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

impl Json {
    fn from<T: Serialize>(x: &T) -> Result<Self, Box<dyn Error>> {
        let s = serde_json::to_string(x)?;
        let j: Self = serde_json::from_str(&s)?;
        Ok(j)
    }

    fn from_str(x: &str) -> Result<Self, Box<dyn Error>> {
        let j: Self = serde_json::from_str(x)?;
        Ok(j)
    }

    fn to<T: for<'de> Deserialize<'de>>(self: &Self) -> Result<T, Box<dyn Error>> {
        let s = serde_json::to_string(self)?;
        let t: T = serde_json::from_str(&s)?;
        Ok(t)
    }

    fn to_string(self: &Self) -> Result<String, Box<dyn Error>> {
        let s = serde_json::to_string(self)?;
        Ok(s)
    }
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

    #[test]
    fn test_get() {
        let data = r#"
        {
            "version": "1.0.0",
            "meta": {
                "date": "2023-6-1",
                "author": "aofusa",
                "license": "apache 2.0"
            },
            "datalist": [
                {
                    "name": "name1",
                    "components": [
                        {
                            "name": "component1",
                            "plugins": [
                            {
                                "name": "plugin1",
                                "args": ["arg1", "arg2"]
                            }
                            ]
                        },
                        {
                            "name": "component2",
                            "plugins": []
                        }
                    ]
                },
                {
                    "name": "name2",
                    "components": [
                        1,
                        2,
                        3
                    ]
                }
            ],
            "specs": {
                "data1": {},
                "data2": {},
                "data3": {}
            }
        }
        "#;

        let json: Json = serde_json::from_str(&data).unwrap();

        if let Json::Object(x) = &json {
            let version = x.get("version").unwrap();
            assert_eq!(&Json::String("1.0.0".to_string()), version);
        };

        if let Json::Object(x) = &json {
            let meta = x.get("meta").unwrap();
            if let Json::Object(y) = &meta {
                let author = y.get("author").unwrap();
                assert_eq!(&Json::String("aofusa".to_string()), author);
            }
        };

        if let Json::Object(x) = &json {
            let datalist = x.get("datalist").unwrap();
            if let Json::Array(y) = &datalist {
                let datalist_1 = &y[1];
                assert_eq!(&Json::Object(HashMap::from([("name".to_string(), Json::String("name2".to_string())), ("components".to_string(), Json::Array(vec![Json::Int(1), Json::Int(2), Json::Int(3)]))])), datalist_1);
            }
        };
    }

    #[test]
    fn test_set() {
        let data = r#"
        {
            "version": "1.0.0",
            "meta": {
                "date": "2023-6-1",
                "author": "aofusa",
                "license": "apache 2.0"
            },
            "datalist": [
                {
                    "name": "name1",
                    "components": [
                        {
                            "name": "component1",
                            "plugins": [
                            {
                                "name": "plugin1",
                                "args": ["arg1", "arg2"]
                            }
                            ]
                        },
                        {
                            "name": "component2",
                            "plugins": []
                        }
                    ]
                },
                {
                    "name": "name2",
                    "components": [
                        1,
                        2,
                        3
                    ]
                }
            ],
            "specs": {
                "data1": {},
                "data2": {},
                "data3": {}
            }
        }
        "#;

        let json: Json = serde_json::from_str(&data).unwrap();

        if let Json::Object(x) = &json {
            let meta = x.get("meta").unwrap();
            if let Json::Object(y) = &meta {
                let license = y.get("license").unwrap();
                assert_eq!(&Json::String("apache 2.0".to_string()), license);
            }
        };

        let json = {
            let mut t = json.clone();
            if let Json::Object(ref mut x) = t {
                let meta = x.get_mut("meta").unwrap();
                if let Json::Object(ref mut y) = meta {
                    y.insert("license".to_string(), Json::String("mit".to_string()));
                }
            };
            t
        };

        if let Json::Object(x) = &json {
            let meta = x.get("meta").unwrap();
            if let Json::Object(y) = &meta {
                let license = y.get("license").unwrap();
                assert_eq!(&Json::String("mit".to_string()), license);
            }
        };
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_struct() {
        let point_struct = Point { x: 1, y: 2};
        let point_str = serde_json::to_string(&point_struct).unwrap();
        let point_json: Json = serde_json::from_str(&point_str).unwrap();
        assert_eq!(r#"{"x":1,"y":2}"#, &point_str);
        assert_eq!(Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::Int(2))])), point_json);
    }

    #[test]
    fn test_from() {
        let point_struct = Point { x: 1, y: 2};
        let point_json = Json::from(&point_struct).unwrap();
        assert_eq!(Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::Int(2))])), point_json);
    }

    #[test]
    fn test_from_str() {
        let point_str = r#"{"x":1,"y":2}"#;
        let point_json = Json::from_str(&point_str).unwrap();
        assert_eq!(Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::Int(2))])), point_json);
    }

    #[test]
    fn test_to() {
        let point_json = Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::Int(2))]));
        let point_struct: Point = point_json.to().unwrap();
        assert_eq!(Point { x: 1, y: 2}, point_struct);
    }

    #[test]
    fn test_to_string() {
        let point_json = Json::Object(HashMap::from([("x".to_string(), Json::Int(1)), ("y".to_string(), Json::Int(2))]));
        let _point_str = point_json.to_string().unwrap();
        // assert_eq!(r#"{"x":1,"y":2}"#, &point_str);
    }

    #[test]
    fn test_get_object() {
        let point_struct = Point { x: 1, y: 2};
        let point_json = Json::from(&point_struct).unwrap();

        let o = point_json.to::<JsonObject>().unwrap();

        let x = o.get("x");
        assert_eq!(Some(&Json::Int(1)), x);

        let z = o.get("z");
        assert_eq!(None, z);
    }

    #[test]
    fn test_get_array() {
        let data = vec![0, 1, 2];
        let j = Json::from(&data).unwrap();

        let a = j.to::<JsonArray>().unwrap();

        let x1 = a.get(1);
        assert_eq!(Some(&Json::Int(1)), x1);

        let x2 = a.get(3);
        assert_eq!(None, x2);
    }
}
