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

/// 조건식 평가
fn evaluate_condition(value: &Value, condition: &Condition) -> Result<bool, DkitError> {
    match condition {
        Condition::Comparison(cmp) => evaluate_comparison(value, cmp),
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
        (Value::String(a), LiteralValue::String(b)) => {
            Ok(apply_compare_op(a.as_str(), op, b.as_str()))
        }
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
}
