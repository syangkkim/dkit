use crate::error::DkitError;
use crate::query::functions::{evaluate_expr, expr_default_key};
use crate::query::parser::{
    AggregateFunc, CompareOp, Comparison, Condition, GroupAggregate, LiteralValue, Operation,
    SelectExpr,
};
use crate::value::Value;
use indexmap::IndexMap;

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
        Operation::Count { field } => apply_count(value, field.as_deref()),
        Operation::Sum { field } => apply_sum(value, field),
        Operation::Avg { field } => apply_avg(value, field),
        Operation::Min { field } => apply_min(value, field),
        Operation::Max { field } => apply_max(value, field),
        Operation::Distinct { field } => apply_distinct(value, field),
        Operation::GroupBy {
            fields,
            having,
            aggregates,
        } => apply_group_by(value, fields, having.as_ref(), aggregates),
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

/// select 절: 배열의 각 요소에 표현식을 적용하여 새 객체를 생성
fn apply_select(value: Value, exprs: &[SelectExpr]) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let projected: Result<Vec<Value>, DkitError> = arr
                .into_iter()
                .map(|item| select_exprs(&item, exprs))
                .collect();
            Ok(Value::Array(projected?))
        }
        Value::Object(_) => select_exprs(&value, exprs),
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

// --- 집계 함수 ---

/// count: 배열의 전체 요소 수 또는 특정 필드의 비null 요소 수를 반환
fn apply_count(value: Value, field: Option<&str>) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let count = match field {
                None => arr.len() as i64,
                Some(f) => arr
                    .iter()
                    .filter(|item| match item {
                        Value::Object(map) => map.get(f).is_some_and(|v| !matches!(v, Value::Null)),
                        _ => false,
                    })
                    .count() as i64,
            };
            Ok(Value::Integer(count))
        }
        _ => Err(DkitError::QueryError(
            "count requires an array input".to_string(),
        )),
    }
}

/// sum: 배열에서 지정된 숫자 필드의 합계를 반환
fn apply_sum(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut sum_int: i64 = 0;
            let mut sum_float: f64 = 0.0;
            let mut has_float = false;

            for item in &arr {
                match item {
                    Value::Object(map) => match map.get(field) {
                        Some(Value::Integer(n)) => {
                            if has_float {
                                sum_float += *n as f64;
                            } else {
                                sum_int = sum_int.saturating_add(*n);
                            }
                        }
                        Some(Value::Float(f)) => {
                            if !has_float {
                                sum_float = sum_int as f64;
                                has_float = true;
                            }
                            sum_float += f;
                        }
                        Some(Value::Null) | None => {}
                        Some(v) => {
                            return Err(DkitError::QueryError(format!(
                                "sum: field '{}' is not numeric (got {})",
                                field, v
                            )));
                        }
                    },
                    _ => {
                        return Err(DkitError::QueryError(
                            "sum requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            if has_float {
                Ok(Value::Float(sum_float))
            } else {
                Ok(Value::Integer(sum_int))
            }
        }
        _ => Err(DkitError::QueryError(
            "sum requires an array input".to_string(),
        )),
    }
}

/// avg: 배열에서 지정된 숫자 필드의 평균을 반환
fn apply_avg(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut sum: f64 = 0.0;
            let mut count: usize = 0;

            for item in &arr {
                match item {
                    Value::Object(map) => match map.get(field) {
                        Some(Value::Integer(n)) => {
                            sum += *n as f64;
                            count += 1;
                        }
                        Some(Value::Float(f)) => {
                            sum += f;
                            count += 1;
                        }
                        Some(Value::Null) | None => {}
                        Some(v) => {
                            return Err(DkitError::QueryError(format!(
                                "avg: field '{}' is not numeric (got {})",
                                field, v
                            )));
                        }
                    },
                    _ => {
                        return Err(DkitError::QueryError(
                            "avg requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            if count == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Float(sum / count as f64))
            }
        }
        _ => Err(DkitError::QueryError(
            "avg requires an array input".to_string(),
        )),
    }
}

/// min: 배열에서 지정된 필드의 최솟값을 반환
fn apply_min(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut min_val: Option<Value> = None;

            for item in &arr {
                match item {
                    Value::Object(map) => {
                        if let Some(v) = map.get(field) {
                            if matches!(v, Value::Null) {
                                continue;
                            }
                            min_val = Some(match &min_val {
                                None => v.clone(),
                                Some(current) => {
                                    if compare_value_ordering(v, current)
                                        == std::cmp::Ordering::Less
                                    {
                                        v.clone()
                                    } else {
                                        current.clone()
                                    }
                                }
                            });
                        }
                    }
                    _ => {
                        return Err(DkitError::QueryError(
                            "min requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            Ok(min_val.unwrap_or(Value::Null))
        }
        _ => Err(DkitError::QueryError(
            "min requires an array input".to_string(),
        )),
    }
}

/// max: 배열에서 지정된 필드의 최댓값을 반환
fn apply_max(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut max_val: Option<Value> = None;

            for item in &arr {
                match item {
                    Value::Object(map) => {
                        if let Some(v) = map.get(field) {
                            if matches!(v, Value::Null) {
                                continue;
                            }
                            max_val = Some(match &max_val {
                                None => v.clone(),
                                Some(current) => {
                                    if compare_value_ordering(v, current)
                                        == std::cmp::Ordering::Greater
                                    {
                                        v.clone()
                                    } else {
                                        current.clone()
                                    }
                                }
                            });
                        }
                    }
                    _ => {
                        return Err(DkitError::QueryError(
                            "max requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            Ok(max_val.unwrap_or(Value::Null))
        }
        _ => Err(DkitError::QueryError(
            "max requires an array input".to_string(),
        )),
    }
}

/// distinct: 배열에서 지정된 필드의 고유값 목록을 반환
fn apply_distinct(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut seen: Vec<Value> = Vec::new();

            for item in &arr {
                match item {
                    Value::Object(map) => {
                        if let Some(v) = map.get(field) {
                            if matches!(v, Value::Null) {
                                continue;
                            }
                            if !seen.contains(v) {
                                seen.push(v.clone());
                            }
                        }
                    }
                    _ => {
                        return Err(DkitError::QueryError(
                            "distinct requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            Ok(Value::Array(seen))
        }
        _ => Err(DkitError::QueryError(
            "distinct requires an array input".to_string(),
        )),
    }
}

/// group_by: 배열을 지정된 필드 기준으로 그룹화하고 집계
fn apply_group_by(
    value: Value,
    fields: &[String],
    having: Option<&Condition>,
    aggregates: &[GroupAggregate],
) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            // Group items by key fields
            let mut groups: Vec<(IndexMap<String, Value>, Vec<Value>)> = Vec::new();

            for item in arr {
                let key = extract_group_key(&item, fields);
                if let Some(pos) = groups.iter().position(|(k, _)| *k == key) {
                    groups[pos].1.push(item);
                } else {
                    groups.push((key, vec![item]));
                }
            }

            // Build result objects for each group
            let mut results = Vec::new();
            for (key, group_items) in groups {
                let mut obj = IndexMap::new();

                // Add group key fields
                for (field_name, field_value) in &key {
                    obj.insert(field_name.clone(), field_value.clone());
                }

                // If no explicit aggregates, add default count
                if aggregates.is_empty() {
                    obj.insert(
                        "count".to_string(),
                        Value::Integer(group_items.len() as i64),
                    );
                } else {
                    // Compute each aggregate
                    for agg in aggregates {
                        let agg_value =
                            compute_group_aggregate(&agg.func, agg.field.as_deref(), &group_items)?;
                        obj.insert(agg.alias.clone(), agg_value);
                    }
                }

                let result_obj = Value::Object(obj);

                // Apply HAVING filter
                if let Some(having_cond) = having {
                    if !evaluate_condition(&result_obj, having_cond).unwrap_or(false) {
                        continue;
                    }
                }

                results.push(result_obj);
            }

            Ok(Value::Array(results))
        }
        _ => Err(DkitError::QueryError(
            "group_by requires an array input".to_string(),
        )),
    }
}

/// 그룹 키 추출: 지정된 필드들의 값을 IndexMap으로 반환
fn extract_group_key(value: &Value, fields: &[String]) -> IndexMap<String, Value> {
    let mut key = IndexMap::new();
    if let Value::Object(map) = value {
        for field in fields {
            let v = map.get(field).cloned().unwrap_or(Value::Null);
            key.insert(field.clone(), v);
        }
    } else {
        for field in fields {
            key.insert(field.clone(), Value::Null);
        }
    }
    key
}

/// 그룹 내 집계 함수 계산
fn compute_group_aggregate(
    func: &AggregateFunc,
    field: Option<&str>,
    items: &[Value],
) -> Result<Value, DkitError> {
    match func {
        AggregateFunc::Count => {
            let count = match field {
                None => items.len() as i64,
                Some(f) => items
                    .iter()
                    .filter(|item| match item {
                        Value::Object(map) => map.get(f).is_some_and(|v| !matches!(v, Value::Null)),
                        _ => false,
                    })
                    .count() as i64,
            };
            Ok(Value::Integer(count))
        }
        AggregateFunc::Sum => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("sum() requires a field argument".to_string())
            })?;
            let mut sum_int: i64 = 0;
            let mut sum_float: f64 = 0.0;
            let mut has_float = false;

            for item in items {
                if let Value::Object(map) = item {
                    match map.get(f) {
                        Some(Value::Integer(n)) => {
                            if has_float {
                                sum_float += *n as f64;
                            } else {
                                sum_int = sum_int.saturating_add(*n);
                            }
                        }
                        Some(Value::Float(fv)) => {
                            if !has_float {
                                sum_float = sum_int as f64;
                                has_float = true;
                            }
                            sum_float += fv;
                        }
                        Some(Value::Null) | None => {}
                        _ => {}
                    }
                }
            }
            if has_float {
                Ok(Value::Float(sum_float))
            } else {
                Ok(Value::Integer(sum_int))
            }
        }
        AggregateFunc::Avg => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("avg() requires a field argument".to_string())
            })?;
            let mut sum: f64 = 0.0;
            let mut count: usize = 0;

            for item in items {
                if let Value::Object(map) = item {
                    match map.get(f) {
                        Some(Value::Integer(n)) => {
                            sum += *n as f64;
                            count += 1;
                        }
                        Some(Value::Float(fv)) => {
                            sum += fv;
                            count += 1;
                        }
                        _ => {}
                    }
                }
            }
            if count == 0 {
                Ok(Value::Null)
            } else {
                Ok(Value::Float(sum / count as f64))
            }
        }
        AggregateFunc::Min => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("min() requires a field argument".to_string())
            })?;
            let mut min_val: Option<Value> = None;

            for item in items {
                if let Value::Object(map) = item {
                    if let Some(v) = map.get(f) {
                        if matches!(v, Value::Null) {
                            continue;
                        }
                        min_val = Some(match &min_val {
                            None => v.clone(),
                            Some(current) => {
                                if compare_value_ordering(v, current) == std::cmp::Ordering::Less {
                                    v.clone()
                                } else {
                                    current.clone()
                                }
                            }
                        });
                    }
                }
            }
            Ok(min_val.unwrap_or(Value::Null))
        }
        AggregateFunc::Max => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("max() requires a field argument".to_string())
            })?;
            let mut max_val: Option<Value> = None;

            for item in items {
                if let Value::Object(map) = item {
                    if let Some(v) = map.get(f) {
                        if matches!(v, Value::Null) {
                            continue;
                        }
                        max_val = Some(match &max_val {
                            None => v.clone(),
                            Some(current) => {
                                if compare_value_ordering(v, current) == std::cmp::Ordering::Greater
                                {
                                    v.clone()
                                } else {
                                    current.clone()
                                }
                            }
                        });
                    }
                }
            }
            Ok(max_val.unwrap_or(Value::Null))
        }
    }
}

/// 오브젝트에서 지정된 필드만 추출
/// 단일 레코드에 SelectExpr 목록을 적용하여 새 객체를 반환
fn select_exprs(value: &Value, exprs: &[SelectExpr]) -> Result<Value, DkitError> {
    use crate::query::parser::Expr;
    match value {
        Value::Object(map) => {
            let mut new_map = IndexMap::new();
            for se in exprs {
                // 단순 필드 참조이고 필드가 없으면 기존 동작대로 스킵
                if let Expr::Field(fname) = &se.expr {
                    if se.alias.is_none() && !map.contains_key(fname) {
                        continue;
                    }
                }
                let val = evaluate_expr(value, &se.expr)?;
                let key = se
                    .alias
                    .clone()
                    .unwrap_or_else(|| expr_default_key(&se.expr));
                new_map.insert(key, val);
            }
            Ok(Value::Object(new_map))
        }
        _ => Ok(value.clone()),
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

    fn sel_fields(names: &[&str]) -> Vec<SelectExpr> {
        use crate::query::parser::Expr;
        names
            .iter()
            .map(|n| SelectExpr {
                expr: Expr::Field(n.to_string()),
                alias: None,
            })
            .collect()
    }

    #[test]
    fn test_select_single_field() {
        let data = sample_users();
        let result = apply_select(data, &sel_fields(&["name"])).unwrap();
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
        let result = apply_select(data, &sel_fields(&["name", "age"])).unwrap();
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
        let result = apply_select(data, &sel_fields(&["city", "name"])).unwrap();
        let arr = result.as_array().unwrap();
        let obj = arr[0].as_object().unwrap();
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec!["city", "name"]);
    }

    #[test]
    fn test_select_missing_field_skipped() {
        let data = sample_users();
        let result = apply_select(data, &sel_fields(&["name", "nonexistent"])).unwrap();
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

        let result = apply_select(data, &sel_fields(&["name", "city"])).unwrap();
        let obj = result.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("city"));
    }

    #[test]
    fn test_select_on_non_object_array() {
        // 배열 안에 비-오브젝트 요소는 그대로 반환
        let data = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let result = apply_select(data, &sel_fields(&["name"])).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr[0], Value::Integer(1));
    }

    #[test]
    fn test_select_on_non_array_non_object_error() {
        let data = Value::Integer(42);
        let result = apply_select(data, &sel_fields(&["name"]));
        assert!(result.is_err());
    }

    #[test]
    fn test_select_empty_array() {
        let data = Value::Array(vec![]);
        let result = apply_select(data, &sel_fields(&["name"])).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_select_with_function() {
        use crate::query::parser::Expr;
        let data = sample_users();
        let exprs = vec![SelectExpr {
            expr: Expr::FuncCall {
                name: "upper".to_string(),
                args: vec![Expr::Field("name".to_string())],
            },
            alias: None,
        }];
        let result = apply_select(data, &exprs).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        let obj = arr[0].as_object().unwrap();
        assert!(obj.contains_key("upper_name"));
        assert_eq!(obj["upper_name"], Value::String("ALICE".to_string()));
    }

    #[test]
    fn test_select_with_alias() {
        use crate::query::parser::Expr;
        let data = sample_users();
        let exprs = vec![SelectExpr {
            expr: Expr::FuncCall {
                name: "upper".to_string(),
                args: vec![Expr::Field("name".to_string())],
            },
            alias: Some("NAME".to_string()),
        }];
        let result = apply_select(data, &exprs).unwrap();
        let arr = result.as_array().unwrap();
        let obj = arr[0].as_object().unwrap();
        assert!(obj.contains_key("NAME"));
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

    // --- 집계 함수 테스트 ---

    #[test]
    fn test_count_all() {
        let data = sample_users();
        let query = parse_query(".[] | count").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_count_field() {
        // name 필드가 있는 요소 수 (모두 있음)
        let data = sample_users();
        let query = parse_query(".[] | count name").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_count_field_with_nulls() {
        // null이 포함된 경우
        let arr = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("email".to_string(), Value::String("a@b.com".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("email".to_string(), Value::Null);
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("email".to_string(), Value::String("c@d.com".to_string()));
                Value::Object(m)
            },
        ]);
        let query = parse_query(".[] | count email").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&arr, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(2));
    }

    #[test]
    fn test_sum_integer() {
        let data = sample_users();
        let query = parse_query(".[] | sum age").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(90)); // 30+25+35
    }

    #[test]
    fn test_sum_float() {
        let arr = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("price".to_string(), Value::Float(1.5));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("price".to_string(), Value::Float(2.5));
                Value::Object(m)
            },
        ]);
        let query = parse_query(".[] | sum price").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&arr, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Float(4.0));
    }

    #[test]
    fn test_avg_integer() {
        let data = sample_users();
        let query = parse_query(".[] | avg age").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Float(30.0)); // (30+25+35)/3
    }

    #[test]
    fn test_avg_empty() {
        let arr = Value::Array(vec![]);
        let result = apply_avg(arr, "age").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_min_integer() {
        let data = sample_users();
        let query = parse_query(".[] | min age").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(25));
    }

    #[test]
    fn test_max_integer() {
        let data = sample_users();
        let query = parse_query(".[] | max age").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(35));
    }

    #[test]
    fn test_min_string() {
        let data = sample_users();
        let result = apply_min(data, "name").unwrap();
        assert_eq!(result, Value::String("Alice".to_string()));
    }

    #[test]
    fn test_max_string() {
        let data = sample_users();
        let result = apply_max(data, "name").unwrap();
        assert_eq!(result, Value::String("Charlie".to_string()));
    }

    #[test]
    fn test_min_empty() {
        let arr = Value::Array(vec![]);
        let result = apply_min(arr, "age").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_distinct() {
        let data = sample_users();
        let query = parse_query(".[] | distinct city").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert!(arr.contains(&Value::String("Seoul".to_string())));
        assert!(arr.contains(&Value::String("Busan".to_string())));
    }

    #[test]
    fn test_count_after_filter() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 28 | count").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(2)); // Alice(30), Charlie(35)
    }

    #[test]
    fn test_sum_after_filter() {
        let data = sample_users();
        let query = parse_query(".[] | where age > 28 | sum age").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::Integer(65)); // 30+35
    }

    // --- GROUP BY tests ---

    fn sample_sales() -> Value {
        Value::Array(vec![
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("electronics".to_string()),
                "product".to_string() => Value::String("phone".to_string()),
                "price".to_string() => Value::Integer(1000),
            }),
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("electronics".to_string()),
                "product".to_string() => Value::String("laptop".to_string()),
                "price".to_string() => Value::Integer(2000),
            }),
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("clothing".to_string()),
                "product".to_string() => Value::String("shirt".to_string()),
                "price".to_string() => Value::Integer(50),
            }),
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("clothing".to_string()),
                "product".to_string() => Value::String("pants".to_string()),
                "price".to_string() => Value::Integer(80),
            }),
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("food".to_string()),
                "product".to_string() => Value::String("apple".to_string()),
                "price".to_string() => Value::Integer(5),
            }),
        ])
    }

    #[test]
    fn test_group_by_basic() {
        let data = sample_sales();
        let query = parse_query(".[] | group_by category").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            assert_eq!(arr.len(), 3);
            // Default count aggregate
            if let Value::Object(map) = &arr[0] {
                assert_eq!(
                    map.get("category"),
                    Some(&Value::String("electronics".to_string()))
                );
                assert_eq!(map.get("count"), Some(&Value::Integer(2)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_with_aggregates() {
        let data = sample_sales();
        let query = parse_query(".[] | group_by category count(), sum(price), avg(price)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            assert_eq!(arr.len(), 3);
            // electronics: count=2, sum=3000, avg=1500.0
            if let Value::Object(map) = &arr[0] {
                assert_eq!(map.get("count"), Some(&Value::Integer(2)));
                assert_eq!(map.get("sum_price"), Some(&Value::Integer(3000)));
                assert_eq!(map.get("avg_price"), Some(&Value::Float(1500.0)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_having() {
        let data = sample_sales();
        let query = parse_query(".[] | group_by category count() having count > 1").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            assert_eq!(arr.len(), 2); // electronics(2) and clothing(2), food(1) filtered out
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_multiple_keys() {
        let data = Value::Array(vec![
            Value::Object(indexmap::indexmap! {
                "region".to_string() => Value::String("east".to_string()),
                "category".to_string() => Value::String("A".to_string()),
                "amount".to_string() => Value::Integer(100),
            }),
            Value::Object(indexmap::indexmap! {
                "region".to_string() => Value::String("east".to_string()),
                "category".to_string() => Value::String("A".to_string()),
                "amount".to_string() => Value::Integer(200),
            }),
            Value::Object(indexmap::indexmap! {
                "region".to_string() => Value::String("east".to_string()),
                "category".to_string() => Value::String("B".to_string()),
                "amount".to_string() => Value::Integer(150),
            }),
            Value::Object(indexmap::indexmap! {
                "region".to_string() => Value::String("west".to_string()),
                "category".to_string() => Value::String("A".to_string()),
                "amount".to_string() => Value::Integer(300),
            }),
        ]);

        let query = parse_query(".[] | group_by region, category count(), sum(amount)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            assert_eq!(arr.len(), 3); // east-A, east-B, west-A
            if let Value::Object(map) = &arr[0] {
                assert_eq!(map.get("region"), Some(&Value::String("east".to_string())));
                assert_eq!(map.get("category"), Some(&Value::String("A".to_string())));
                assert_eq!(map.get("count"), Some(&Value::Integer(2)));
                assert_eq!(map.get("sum_amount"), Some(&Value::Integer(300)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_min_max() {
        let data = sample_sales();
        let query = parse_query(".[] | group_by category min(price), max(price)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            // electronics: min=1000, max=2000
            if let Value::Object(map) = &arr[0] {
                assert_eq!(map.get("min_price"), Some(&Value::Integer(1000)));
                assert_eq!(map.get("max_price"), Some(&Value::Integer(2000)));
            } else {
                panic!("expected object");
            }
            // clothing: min=50, max=80
            if let Value::Object(map) = &arr[1] {
                assert_eq!(map.get("min_price"), Some(&Value::Integer(50)));
                assert_eq!(map.get("max_price"), Some(&Value::Integer(80)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_empty_array() {
        let data = Value::Array(vec![]);
        let query = parse_query(".[] | group_by category").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_group_by_non_array_error() {
        let data = Value::Object(indexmap::indexmap! {
            "name".to_string() => Value::String("test".to_string()),
        });
        let result = apply_group_by(data, &["name".to_string()], None, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_group_by_with_sort() {
        let data = sample_sales();
        let query = parse_query(".[] | group_by category count() | sort count desc").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            // electronics(2), clothing(2) come first, food(1) last
            if let Value::Object(map) = &arr[2] {
                assert_eq!(map.get("count"), Some(&Value::Integer(1)));
                assert_eq!(
                    map.get("category"),
                    Some(&Value::String("food".to_string()))
                );
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_group_by_with_null_keys() {
        let data = Value::Array(vec![
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::String("A".to_string()),
                "val".to_string() => Value::Integer(1),
            }),
            Value::Object(indexmap::indexmap! {
                "category".to_string() => Value::Null,
                "val".to_string() => Value::Integer(2),
            }),
            Value::Object(indexmap::indexmap! {
                "val".to_string() => Value::Integer(3),
            }),
        ]);

        let query = parse_query(".[] | group_by category count()").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();

        if let Value::Array(arr) = &result {
            assert_eq!(arr.len(), 2); // "A" group and null group (null + missing)
        } else {
            panic!("expected array");
        }
    }
}
