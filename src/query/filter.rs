use crate::error::DkitError;
use crate::query::parser::{CompareOp, Comparison, Condition, LiteralValue, Operation};
use crate::value::Value;

/// 연산 목록을 순차적으로 적용
pub fn apply_operations(value: Value, operations: &[Operation]) -> Result<Value, DkitError> {
    let mut current = value;
    for op in operations {
        current = apply_operation(current, op)?;
    }
    Ok(current)
}

/// 단일 연산 적용
fn apply_operation(value: Value, operation: &Operation) -> Result<Value, DkitError> {
    match operation {
        Operation::Where(condition) => apply_where(value, condition),
        Operation::Select(fields) => apply_select(value, fields),
        Operation::Sort { field, descending } => apply_sort(value, field, *descending),
        Operation::Limit(n) => apply_limit(value, *n),
    }
}

/// where 절: 배열의 각 요소에 대해 조건을 평가하고 필터링
fn apply_where(value: Value, condition: &Condition) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let filtered: Vec<Value> = arr
                .into_iter()
                .filter(|item| evaluate_condition(item, condition).unwrap_or(false))
                .collect();
            Ok(Value::Array(filtered))
        }
        _ => Err(DkitError::QueryError(
            "where clause requires an array input".to_string(),
        )),
    }
}

/// select 절: 배열의 각 요소에서 지정된 필드만 추출
fn apply_select(value: Value, fields: &[String]) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let projected: Vec<Value> = arr
                .into_iter()
                .map(|item| select_fields(item, fields))
                .collect();
            Ok(Value::Array(projected))
        }
        Value::Object(_) => Ok(select_fields(value, fields)),
        _ => Err(DkitError::QueryError(
            "select clause requires an array or object input".to_string(),
        )),
    }
}

/// sort 절: 배열의 요소를 지정된 필드 기준으로 정렬
fn apply_sort(value: Value, field: &str, descending: bool) -> Result<Value, DkitError> {
    match value {
        Value::Array(mut arr) => {
            arr.sort_by(|a, b| {
                let va = extract_sort_key(a, field);
                let vb = extract_sort_key(b, field);
                let ord = compare_sort_keys(&va, &vb);
                if descending {
                    ord.reverse()
                } else {
                    ord
                }
            });
            Ok(Value::Array(arr))
        }
        _ => Err(DkitError::QueryError(
            "sort clause requires an array input".to_string(),
        )),
    }
}

/// 정렬용 키 값 추출
fn extract_sort_key(value: &Value, field: &str) -> Option<Value> {
    match value {
        Value::Object(map) => map.get(field).cloned(),
        _ => None,
    }
}

/// 정렬 키 비교 (None은 항상 뒤로)
fn compare_sort_keys(a: &Option<Value>, b: &Option<Value>) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(va), Some(vb)) => compare_value_ordering(va, vb),
    }
}

/// Value 간 순서 비교
fn compare_value_ordering(a: &Value, b: &Value) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
        (Value::Integer(x), Value::Float(y)) => {
            (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal)
        }
        (Value::Float(x), Value::Integer(y)) => {
            x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal)
        }
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        // 타입이 다른 경우 타입 순서로 정렬: Null < Bool < Integer/Float < String
        _ => type_order(a).cmp(&type_order(b)),
    }
}

/// 타입별 정렬 우선순위
fn type_order(v: &Value) -> u8 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Integer(_) | Value::Float(_) => 2,
        Value::String(_) => 3,
        Value::Array(_) => 4,
        Value::Object(_) => 5,
    }
}

/// limit 절: 배열의 처음 N개 요소만 반환
fn apply_limit(value: Value, n: usize) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let limited: Vec<Value> = arr.into_iter().take(n).collect();
            Ok(Value::Array(limited))
        }
        _ => Err(DkitError::QueryError(
            "limit clause requires an array input".to_string(),
        )),
    }
}

/// 오브젝트에서 지정된 필드만 추출
fn select_fields(value: Value, fields: &[String]) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = indexmap::IndexMap::new();
            for field in fields {
                if let Some(v) = map.get(field) {
                    new_map.insert(field.clone(), v.clone());
                }
            }
            Value::Object(new_map)
        }
        _ => value,
    }
}

/// 조건식 평가
fn evaluate_condition(value: &Value, condition: &Condition) -> Result<bool, DkitError> {
    match condition {
        Condition::Comparison(cmp) => evaluate_comparison(value, cmp),
        Condition::And(left, right) => {
            Ok(evaluate_condition(value, left)? && evaluate_condition(value, right)?)
        }
        Condition::Or(left, right) => {
            Ok(evaluate_condition(value, left)? || evaluate_condition(value, right)?)
        }
    }
}

/// 비교식 평가: value의 필드를 리터럴과 비교
fn evaluate_comparison(value: &Value, cmp: &Comparison) -> Result<bool, DkitError> {
    let field_value = match value {
        Value::Object(map) => match map.get(&cmp.field) {
            Some(v) => v,
            None => return Ok(false),
        },
        _ => return Ok(false),
    };

    compare_values(field_value, &cmp.op, &cmp.value)
}

/// Value와 LiteralValue를 비교
fn compare_values(
    field: &Value,
    op: &CompareOp,
    literal: &LiteralValue,
) -> Result<bool, DkitError> {
    match (field, literal) {
        // 정수 비교
        (Value::Integer(a), LiteralValue::Integer(b)) => Ok(apply_compare_op(*a, op, *b)),
        // 정수 필드 vs 부동소수점 리터럴
        (Value::Integer(a), LiteralValue::Float(b)) => Ok(apply_compare_op(*a as f64, op, *b)),
        // 부동소수점 비교
        (Value::Float(a), LiteralValue::Float(b)) => Ok(apply_compare_op(*a, op, *b)),
        // 부동소수점 필드 vs 정수 리터럴
        (Value::Float(a), LiteralValue::Integer(b)) => Ok(apply_compare_op(*a, op, *b as f64)),
        // 문자열 비교
        (Value::String(a), LiteralValue::String(b)) => match op {
            CompareOp::Contains => Ok(a.contains(b.as_str())),
            CompareOp::StartsWith => Ok(a.starts_with(b.as_str())),
            CompareOp::EndsWith => Ok(a.ends_with(b.as_str())),
            _ => Ok(apply_compare_op(a.as_str(), op, b.as_str())),
        },
        // 불리언: == 와 != 만 지원
        (Value::Bool(a), LiteralValue::Bool(b)) => match op {
            CompareOp::Eq => Ok(a == b),
            CompareOp::Ne => Ok(a != b),
            _ => Err(DkitError::QueryError(
                "boolean values only support == and != operators".to_string(),
            )),
        },
        // null 비교: == 와 != 만 지원
        (Value::Null, LiteralValue::Null) => match op {
            CompareOp::Eq => Ok(true),
            CompareOp::Ne => Ok(false),
            _ => Err(DkitError::QueryError(
                "null values only support == and != operators".to_string(),
            )),
        },
        // null vs non-null
        (Value::Null, _) => match op {
            CompareOp::Eq => Ok(false),
            CompareOp::Ne => Ok(true),
            _ => Ok(false),
        },
        // 타입 불일치: false 반환 (== 에서는 false, != 에서는 true)
        (_, _) => match op {
            CompareOp::Eq => Ok(false),
            CompareOp::Ne => Ok(true),
            _ => Ok(false),
        },
    }
}

/// 비교 연산자 적용 (PartialOrd를 구현하는 타입에 대해)
fn apply_compare_op<T: PartialOrd>(a: T, op: &CompareOp, b: T) -> bool {
    match op {
        CompareOp::Eq => a == b,
        CompareOp::Ne => a != b,
        CompareOp::Gt => a > b,
        CompareOp::Lt => a < b,
        CompareOp::Ge => a >= b,
        CompareOp::Le => a <= b,
        CompareOp::Contains | CompareOp::StartsWith | CompareOp::EndsWith => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::parser::parse_query;
    use indexmap::IndexMap;

    fn sample_users() -> Value {
        let users = vec![
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Alice".to_string()));
                u.insert("age".to_string(), Value::Integer(30));
                u.insert("city".to_string(), Value::String("Seoul".to_string()));
                Value::Object(u)
            },
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Bob".to_string()));
                u.insert("age".to_string(), Value::Integer(25));
                u.insert("city".to_string(), Value::String("Busan".to_string()));
                Value::Object(u)
            },
            {
                let mut u = IndexMap::new();
                u.insert("name".to_string(), Value::String("Charlie".to_string()));
                u.insert("age".to_string(), Value::Integer(35));
                u.insert("city".to_string(), Value::String("Seoul".to_string()));
                Value::Object(u)
            },
        ];
        Value::Array(users)
    }

    fn run_where(data: &Value, condition: &Condition) -> Result<Value, DkitError> {
        apply_where(data.clone(), condition)
    }

    fn make_condition(field: &str, op: CompareOp, value: LiteralValue) -> Condition {
        Condition::Comparison(Comparison {
            field: field.to_string(),
            op,
            value,
        })
    }

    // --- where 기본 동작 ---

    #[test]
    fn test_where_eq_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Eq, LiteralValue::Integer(30));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
    }

    #[test]
    fn test_where_ne_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Ne, LiteralValue::Integer(30));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_where_gt_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Gt, LiteralValue::Integer(28));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30), Charlie(35)
    }

    #[test]
    fn test_where_lt_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Lt, LiteralValue::Integer(30));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1); // Bob(25)
    }

    #[test]
    fn test_where_ge_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Ge, LiteralValue::Integer(30));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30), Charlie(35)
    }

    #[test]
    fn test_where_le_integer() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Le, LiteralValue::Integer(30));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30), Bob(25)
    }

    // --- 문자열 비교 ---

    #[test]
    fn test_where_eq_string() {
        let data = sample_users();
        let cond = make_condition(
            "city",
            CompareOp::Eq,
            LiteralValue::String("Seoul".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice, Charlie
    }

    #[test]
    fn test_where_ne_string() {
        let data = sample_users();
        let cond = make_condition(
            "city",
            CompareOp::Ne,
            LiteralValue::String("Seoul".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1); // Bob
    }

    // --- 필드 없는 경우 ---

    #[test]
    fn test_where_missing_field() {
        let data = sample_users();
        let cond = make_condition("nonexistent", CompareOp::Eq, LiteralValue::Integer(1));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    // --- 빈 배열 ---

    #[test]
    fn test_where_empty_array() {
        let data = Value::Array(vec![]);
        let cond = make_condition("age", CompareOp::Gt, LiteralValue::Integer(0));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    // --- 비배열에 대한 where 에러 ---

    #[test]
    fn test_where_on_non_array() {
        let data = Value::Integer(42);
        let cond = make_condition("age", CompareOp::Gt, LiteralValue::Integer(0));
        let result = run_where(&data, &cond);
        assert!(result.is_err());
    }

    // --- 불리언 비교 ---

    #[test]
    fn test_where_eq_bool() {
        let data = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("active".to_string(), Value::Bool(true));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("active".to_string(), Value::Bool(false));
                m
            }),
        ]);
        let cond = make_condition("active", CompareOp::Eq, LiteralValue::Bool(true));
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 1);
    }

    // --- null 비교 ---

    #[test]
    fn test_where_eq_null() {
        let data = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("value".to_string(), Value::Null);
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("value".to_string(), Value::Integer(1));
                m
            }),
        ]);
        let cond = make_condition("value", CompareOp::Eq, LiteralValue::Null);
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 1);
    }

    // --- 부동소수점 비교 ---

    #[test]
    fn test_where_gt_float() {
        let data = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("score".to_string(), Value::Float(3.14));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("score".to_string(), Value::Float(2.71));
                m
            }),
        ]);
        let cond = make_condition("score", CompareOp::Gt, LiteralValue::Float(3.0));
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 1);
    }

    // --- 정수 필드 vs 부동소수점 리터럴 ---

    #[test]
    fn test_where_integer_field_float_literal() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Gt, LiteralValue::Float(29.5));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30), Charlie(35)
    }

    // --- apply_operations 통합 테스트 ---

    #[test]
    fn test_apply_operations_where() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 30").unwrap();

        // 먼저 path 평가
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        // 그 다음 operations 적용
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        );
    }

    #[test]
    fn test_apply_operations_no_ops() {
        let data = Value::Integer(42);
        let result = apply_operations(data.clone(), &[]).unwrap();
        assert_eq!(result, data);
    }

    // --- 타입 불일치 ---

    #[test]
    fn test_where_type_mismatch_returns_false() {
        let data = sample_users();
        // age는 Integer인데 String과 비교
        let cond = make_condition("age", CompareOp::Eq, LiteralValue::String("30".to_string()));
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_where_type_mismatch_ne_returns_true() {
        let data = sample_users();
        let cond = make_condition("age", CompareOp::Ne, LiteralValue::String("30".to_string()));
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 3);
    }

    // --- 문자열 연산자 ---

    fn sample_files() -> Value {
        let files = vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("readme.md".to_string()));
                m.insert(
                    "email".to_string(),
                    Value::String("alice@gmail.com".to_string()),
                );
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("config.json".to_string()));
                m.insert(
                    "email".to_string(),
                    Value::String("bob@yahoo.com".to_string()),
                );
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("data.json".to_string()));
                m.insert(
                    "email".to_string(),
                    Value::String("charlie@gmail.com".to_string()),
                );
                m
            }),
        ];
        Value::Array(files)
    }

    #[test]
    fn test_where_contains() {
        let data = sample_files();
        let cond = make_condition(
            "email",
            CompareOp::Contains,
            LiteralValue::String("@gmail".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_where_starts_with() {
        let data = sample_files();
        let cond = make_condition(
            "name",
            CompareOp::StartsWith,
            LiteralValue::String("config".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_where_ends_with() {
        let data = sample_files();
        let cond = make_condition(
            "name",
            CompareOp::EndsWith,
            LiteralValue::String(".json".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_where_contains_no_match() {
        let data = sample_files();
        let cond = make_condition(
            "email",
            CompareOp::Contains,
            LiteralValue::String("@hotmail".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    // --- 논리 연산자 ---

    #[test]
    fn test_where_and() {
        let data = sample_users();
        let cond = Condition::And(
            Box::new(make_condition(
                "age",
                CompareOp::Gt,
                LiteralValue::Integer(25),
            )),
            Box::new(make_condition(
                "city",
                CompareOp::Eq,
                LiteralValue::String("Seoul".to_string()),
            )),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30, Seoul), Charlie(35, Seoul)
    }

    #[test]
    fn test_where_or() {
        let data = sample_users();
        let cond = Condition::Or(
            Box::new(make_condition(
                "name",
                CompareOp::Eq,
                LiteralValue::String("Alice".to_string()),
            )),
            Box::new(make_condition(
                "name",
                CompareOp::Eq,
                LiteralValue::String("Bob".to_string()),
            )),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_where_and_no_match() {
        let data = sample_users();
        let cond = Condition::And(
            Box::new(make_condition(
                "age",
                CompareOp::Lt,
                LiteralValue::Integer(26),
            )),
            Box::new(make_condition(
                "city",
                CompareOp::Eq,
                LiteralValue::String("Seoul".to_string()),
            )),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0); // Bob is 25 but in Busan
    }

    // --- 통합: 파서 + 필터 ---

    #[test]
    fn test_integration_string_op() {
        let data = sample_files();
        let query = parse_query(".[] | where name ends_with \".json\"").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_integration_logical_and() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 25 and city == \"Seoul\"").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30, Seoul), Charlie(35, Seoul)
    }

    #[test]
    fn test_integration_logical_or() {
        let data = sample_users();
        let query = parse_query(".[] | where age == 25 or age == 35").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Bob(25), Charlie(35)
    }

    // --- select 절 ---

    #[test]
    fn test_select_single_field() {
        let data = sample_users();
        let result = apply_select(data, &["name".to_string()]).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        // 각 요소에 name만 있어야 함
        for item in arr {
            let obj = item.as_object().unwrap();
            assert_eq!(obj.len(), 1);
            assert!(obj.contains_key("name"));
        }
    }

    #[test]
    fn test_select_multiple_fields() {
        let data = sample_users();
        let result = apply_select(data, &["name".to_string(), "age".to_string()]).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        for item in arr {
            let obj = item.as_object().unwrap();
            assert_eq!(obj.len(), 2);
            assert!(obj.contains_key("name"));
            assert!(obj.contains_key("age"));
        }
    }

    #[test]
    fn test_select_preserves_order() {
        let data = sample_users();
        let result = apply_select(data, &["city".to_string(), "name".to_string()]).unwrap();
        let arr = result.as_array().unwrap();
        let obj = arr[0].as_object().unwrap();
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec!["city", "name"]);
    }

    #[test]
    fn test_select_missing_field_skipped() {
        let data = sample_users();
        let result = apply_select(data, &["name".to_string(), "nonexistent".to_string()]).unwrap();
        let arr = result.as_array().unwrap();
        for item in arr {
            let obj = item.as_object().unwrap();
            assert_eq!(obj.len(), 1);
            assert!(obj.contains_key("name"));
        }
    }

    #[test]
    fn test_select_on_single_object() {
        let mut m = IndexMap::new();
        m.insert("name".to_string(), Value::String("Alice".to_string()));
        m.insert("age".to_string(), Value::Integer(30));
        m.insert("city".to_string(), Value::String("Seoul".to_string()));
        let data = Value::Object(m);

        let result = apply_select(data, &["name".to_string(), "city".to_string()]).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("city"));
    }

    #[test]
    fn test_select_on_non_object_array() {
        // 배열 안에 비-오브젝트 요소는 그대로 반환
        let data = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let result = apply_select(data, &["name".to_string()]).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr[0], Value::Integer(1));
    }

    #[test]
    fn test_select_on_non_array_non_object_error() {
        let data = Value::Integer(42);
        let result = apply_select(data, &["name".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_select_empty_array() {
        let data = Value::Array(vec![]);
        let result = apply_select(data, &["name".to_string()]).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    // --- 통합: where + select ---

    #[test]
    fn test_integration_where_then_select() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 25 | select name, city").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(30), Charlie(35)
        for item in arr {
            let obj = item.as_object().unwrap();
            assert_eq!(obj.len(), 2);
            assert!(obj.contains_key("name"));
            assert!(obj.contains_key("city"));
            assert!(!obj.contains_key("age"));
        }
    }

    #[test]
    fn test_integration_select_only() {
        let data = sample_users();
        let query = parse_query(".[] | select name").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        for item in arr {
            let obj = item.as_object().unwrap();
            assert_eq!(obj.len(), 1);
            assert!(obj.contains_key("name"));
        }
    }

    // --- sort 절 ---

    #[test]
    fn test_sort_asc_integer() {
        let data = sample_users();
        let result = apply_sort(data, "age", false).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Bob".to_string()))
        ); // 25
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        ); // 30
        assert_eq!(
            arr[2].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        ); // 35
    }

    #[test]
    fn test_sort_desc_integer() {
        let data = sample_users();
        let result = apply_sort(data, "age", true).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        ); // 35
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        ); // 30
        assert_eq!(
            arr[2].as_object().unwrap().get("name"),
            Some(&Value::String("Bob".to_string()))
        ); // 25
    }

    #[test]
    fn test_sort_asc_string() {
        let data = sample_users();
        let result = apply_sort(data, "name", false).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Bob".to_string()))
        );
        assert_eq!(
            arr[2].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        );
    }

    #[test]
    fn test_sort_desc_string() {
        let data = sample_users();
        let result = apply_sort(data, "name", true).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Bob".to_string()))
        );
        assert_eq!(
            arr[2].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
    }

    #[test]
    fn test_sort_missing_field() {
        // 일부 요소에 필드가 없으면 뒤로 정렬
        let data = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("age".to_string(), Value::Integer(25));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("age".to_string(), Value::Integer(30));
                m
            }),
        ]);
        let result = apply_sort(data, "age", false).unwrap();
        let arr = result.as_array().unwrap();
        // Alice has no age → goes to end
        assert_eq!(
            arr[0].as_object().unwrap().get("age"),
            Some(&Value::Integer(25))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("age"),
            Some(&Value::Integer(30))
        );
        assert_eq!(arr[2].as_object().unwrap().get("age"), None);
    }

    #[test]
    fn test_sort_empty_array() {
        let data = Value::Array(vec![]);
        let result = apply_sort(data, "age", false).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_sort_on_non_array() {
        let data = Value::Integer(42);
        let result = apply_sort(data, "age", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_sort_float() {
        let data = Value::Array(vec![
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("score".to_string(), Value::Float(3.14));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("score".to_string(), Value::Float(1.41));
                m
            }),
            Value::Object({
                let mut m = IndexMap::new();
                m.insert("score".to_string(), Value::Float(2.71));
                m
            }),
        ]);
        let result = apply_sort(data, "score", false).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap().get("score"),
            Some(&Value::Float(1.41))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("score"),
            Some(&Value::Float(2.71))
        );
        assert_eq!(
            arr[2].as_object().unwrap().get("score"),
            Some(&Value::Float(3.14))
        );
    }

    // --- limit 절 ---

    #[test]
    fn test_limit_basic() {
        let data = sample_users();
        let result = apply_limit(data, 2).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Bob".to_string()))
        );
    }

    #[test]
    fn test_limit_larger_than_array() {
        let data = sample_users();
        let result = apply_limit(data, 100).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_limit_zero() {
        let data = sample_users();
        let result = apply_limit(data, 0).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_limit_one() {
        let data = sample_users();
        let result = apply_limit(data, 1).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn test_limit_empty_array() {
        let data = Value::Array(vec![]);
        let result = apply_limit(data, 5).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_limit_on_non_array() {
        let data = Value::Integer(42);
        let result = apply_limit(data, 5);
        assert!(result.is_err());
    }

    // --- 통합: sort + limit ---

    #[test]
    fn test_integration_sort_then_limit() {
        let data = sample_users();
        let query = parse_query(".[] | sort age desc | limit 2").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        ); // 35
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        ); // 30
    }

    #[test]
    fn test_integration_where_sort_limit() {
        let data = sample_users();
        let query = parse_query(".[] | where city == \"Seoul\" | sort age | limit 1").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        ); // Alice(30, Seoul) is younger than Charlie(35, Seoul)
    }

    #[test]
    fn test_integration_where_select_sort() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 25 | select name, age | sort name").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(
            arr[0].as_object().unwrap().get("name"),
            Some(&Value::String("Alice".to_string()))
        );
        assert_eq!(
            arr[1].as_object().unwrap().get("name"),
            Some(&Value::String("Charlie".to_string()))
        );
    }
}
