use crate::error::DkitError;
use crate::query::parser::{Path, Segment};
use crate::value::Value;

/// 경로를 사용하여 Value에서 데이터를 추출
pub fn evaluate_path(value: &Value, path: &Path) -> Result<Value, DkitError> {
    let mut results = vec![value.clone()];

    for segment in &path.segments {
        let mut next_results = Vec::new();

        for val in &results {
            match segment {
                Segment::Field(name) => match val {
                    Value::Object(map) => match map.get(name) {
                        Some(v) => next_results.push(v.clone()),
                        None => {
                            return Err(DkitError::PathNotFound(format!(
                                "field '{}' not found",
                                name
                            )));
                        }
                    },
                    _ => {
                        return Err(DkitError::PathNotFound(format!(
                            "cannot access field '{}' on non-object value",
                            name
                        )));
                    }
                },
                Segment::Index(idx) => match val {
                    Value::Array(arr) => {
                        let resolved = resolve_index(*idx, arr.len())?;
                        next_results.push(arr[resolved].clone());
                    }
                    _ => {
                        return Err(DkitError::PathNotFound(format!(
                            "cannot index non-array value with [{}]",
                            idx
                        )));
                    }
                },
                Segment::Iterate => match val {
                    Value::Array(arr) => {
                        next_results.extend(arr.iter().cloned());
                    }
                    _ => {
                        return Err(DkitError::PathNotFound(
                            "cannot iterate over non-array value".to_string(),
                        ));
                    }
                },
            }
        }

        results = next_results;
    }

    // 이터레이션이 있었으면 배열로 반환, 아니면 단일 값
    let has_iterate = path.segments.iter().any(|s| matches!(s, Segment::Iterate));
    if has_iterate {
        Ok(Value::Array(results))
    } else {
        // 단일 결과
        match results.len() {
            0 => Err(DkitError::PathNotFound("empty result".to_string())),
            1 => Ok(results.into_iter().next().unwrap()),
            _ => Ok(Value::Array(results)),
        }
    }
}

/// 음수 인덱스를 양수로 변환
fn resolve_index(index: i64, len: usize) -> Result<usize, DkitError> {
    let resolved = if index < 0 {
        let positive = (-index) as usize;
        if positive > len {
            return Err(DkitError::PathNotFound(format!(
                "index {} out of bounds (array length: {})",
                index, len
            )));
        }
        len - positive
    } else {
        let idx = index as usize;
        if idx >= len {
            return Err(DkitError::PathNotFound(format!(
                "index {} out of bounds (array length: {})",
                index, len
            )));
        }
        idx
    };
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::parser::parse_query;
    use indexmap::IndexMap;

    fn eval(value: &Value, query_str: &str) -> Result<Value, DkitError> {
        let query = parse_query(query_str).unwrap();
        evaluate_path(value, &query.path)
    }

    fn sample_data() -> Value {
        // {
        //   "name": "dkit",
        //   "version": 1,
        //   "users": [
        //     {"name": "Alice", "age": 30},
        //     {"name": "Bob", "age": 25},
        //     {"name": "Charlie", "age": 35}
        //   ],
        //   "config": {"database": {"host": "localhost", "port": 5432}}
        // }
        let mut data = IndexMap::new();
        data.insert("name".to_string(), Value::String("dkit".to_string()));
        data.insert("version".to_string(), Value::Integer(1));

        let users = vec![
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Alice".to_string()));
                u.insert("age".to_string(), Value::Integer(30));
                Value::Object(u)
            },
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Bob".to_string()));
                u.insert("age".to_string(), Value::Integer(25));
                Value::Object(u)
            },
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Charlie".to_string()));
                u.insert("age".to_string(), Value::Integer(35));
                Value::Object(u)
            },
        ];
        data.insert("users".to_string(), Value::Array(users));

        let mut db = IndexMap::new();
        db.insert("host".to_string(), Value::String("localhost".to_string()));
        db.insert("port".to_string(), Value::Integer(5432));
        let mut config = IndexMap::new();
        config.insert("database".to_string(), Value::Object(db));
        data.insert("config".to_string(), Value::Object(config));

        Value::Object(data)
    }

    // --- 루트 접근 ---

    #[test]
    fn test_root() {
        let data = sample_data();
        let result = eval(&data, ".").unwrap();
        assert_eq!(result, data);
    }

    // --- 필드 접근 ---

    #[test]
    fn test_field_access() {
        let data = sample_data();
        let result = eval(&data, ".name").unwrap();
        assert_eq!(result, Value::String("dkit".to_string()));
    }

    #[test]
    fn test_nested_field() {
        let data = sample_data();
        let result = eval(&data, ".config.database.host").unwrap();
        assert_eq!(result, Value::String("localhost".to_string()));
    }

    #[test]
    fn test_nested_field_integer() {
        let data = sample_data();
        let result = eval(&data, ".config.database.port").unwrap();
        assert_eq!(result, Value::Integer(5432));
    }

    #[test]
    fn test_field_not_found() {
        let data = sample_data();
        let err = eval(&data, ".nonexistent").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    #[test]
    fn test_field_on_non_object() {
        let data = sample_data();
        let err = eval(&data, ".name.sub").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    // --- 배열 인덱싱 ---

    #[test]
    fn test_array_index_zero() {
        let data = sample_data();
        let result = eval(&data, ".users[0]").unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
    }

    #[test]
    fn test_array_index_last() {
        let data = sample_data();
        let result = eval(&data, ".users[-1]").unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("name"), Some(&Value::String("Charlie".to_string())));
    }

    #[test]
    fn test_array_index_with_field() {
        let data = sample_data();
        let result = eval(&data, ".users[0].name").unwrap();
        assert_eq!(result, Value::String("Alice".to_string()));
    }

    #[test]
    fn test_array_index_negative_two() {
        let data = sample_data();
        let result = eval(&data, ".users[-2].name").unwrap();
        assert_eq!(result, Value::String("Bob".to_string()));
    }

    #[test]
    fn test_array_index_out_of_bounds() {
        let data = sample_data();
        let err = eval(&data, ".users[10]").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    #[test]
    fn test_array_index_negative_out_of_bounds() {
        let data = sample_data();
        let err = eval(&data, ".users[-10]").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    #[test]
    fn test_index_on_non_array() {
        let data = sample_data();
        let err = eval(&data, ".name[0]").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    // --- 배열 이터레이션 ---

    #[test]
    fn test_iterate() {
        let data = sample_data();
        let result = eval(&data, ".users[]").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_iterate_with_field() {
        let data = sample_data();
        let result = eval(&data, ".users[].name").unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("Alice".to_string()),
                Value::String("Bob".to_string()),
                Value::String("Charlie".to_string()),
            ])
        );
    }

    #[test]
    fn test_iterate_with_field_integer() {
        let data = sample_data();
        let result = eval(&data, ".users[].age").unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::Integer(30),
                Value::Integer(25),
                Value::Integer(35),
            ])
        );
    }

    #[test]
    fn test_iterate_empty_array() {
        let data = Value::Object({
            let mut m = IndexMap::new();
            m.insert("items".to_string(), Value::Array(vec![]));
            m
        });
        let result = eval(&data, ".items[]").unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_iterate_on_non_array() {
        let data = sample_data();
        let err = eval(&data, ".name[]").unwrap_err();
        assert!(matches!(err, DkitError::PathNotFound(_)));
    }

    // --- 루트 배열 ---

    #[test]
    fn test_root_array_index() {
        let data = Value::Array(vec![
            Value::Integer(10),
            Value::Integer(20),
            Value::Integer(30),
        ]);
        let result = eval(&data, ".[0]").unwrap();
        assert_eq!(result, Value::Integer(10));
    }

    #[test]
    fn test_root_array_iterate() {
        let data = Value::Array(vec![
            Value::Integer(10),
            Value::Integer(20),
            Value::Integer(30),
        ]);
        let result = eval(&data, ".[]").unwrap();
        assert_eq!(result, data);
    }

    // --- 중첩 이터레이션 ---

    #[test]
    fn test_nested_iterate() {
        let data = Value::Object({
            let mut m = IndexMap::new();
            m.insert(
                "groups".to_string(),
                Value::Array(vec![
                    Value::Object({
                        let mut g = IndexMap::new();
                        g.insert(
                            "members".to_string(),
                            Value::Array(vec![
                                Value::String("a".to_string()),
                                Value::String("b".to_string()),
                            ]),
                        );
                        g
                    }),
                    Value::Object({
                        let mut g = IndexMap::new();
                        g.insert(
                            "members".to_string(),
                            Value::Array(vec![Value::String("c".to_string())]),
                        );
                        g
                    }),
                ]),
            );
            m
        });

        let result = eval(&data, ".groups[].members[]").unwrap();
        assert_eq!(
            result,
            Value::Array(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ])
        );
    }

    // --- 프리미티브 값에 대한 루트 접근 ---

    #[test]
    fn test_root_primitive() {
        let data = Value::String("hello".to_string());
        let result = eval(&data, ".").unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_root_null() {
        let data = Value::Null;
        let result = eval(&data, ".").unwrap();
        assert_eq!(result, Value::Null);
    }
}
