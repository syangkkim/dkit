use crate::error::DkitError;
use crate::query::functions::{evaluate_expr, expr_default_key};
use crate::query::parser::{
    AggregateFunc, CompareOp, Comparison, Condition, Expr, GroupAggregate, LiteralValue, Operation,
    SelectExpr, WindowFunc, WindowSpec,
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
        Operation::Median { field } => apply_median(value, field),
        Operation::Percentile { field, p } => apply_percentile(value, field, *p),
        Operation::Stddev { field } => apply_stddev(value, field),
        Operation::Variance { field } => apply_variance(value, field),
        Operation::Mode { field } => apply_mode(value, field),
        Operation::GroupConcat { field, separator } => apply_group_concat(value, field, separator),
        Operation::GroupBy {
            fields,
            having,
            aggregates,
        } => apply_group_by(value, fields, having.as_ref(), aggregates),
        Operation::Unique => apply_unique(value),
        Operation::UniqueBy { field } => apply_unique_by(value, field),
        Operation::AddField { name, expr } => apply_add_field(value, name, expr),
        Operation::MapField { name, expr } => apply_map_field(value, name, expr),
        Operation::Explode { field } => apply_explode(value, field),
        Operation::Unpivot {
            value_columns,
            key_name,
            value_name,
        } => apply_unpivot(value, value_columns, key_name, value_name),
        Operation::Pivot {
            index_fields,
            columns_field,
            values_field,
        } => apply_pivot(value, index_fields, columns_field, values_field),
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
    // 윈도우 함수가 포함되어 있는지 확인
    let has_window = exprs
        .iter()
        .any(|se| matches!(&se.expr, Expr::Window { .. }));

    match value {
        Value::Array(arr) => {
            if has_window {
                apply_select_with_window(arr, exprs)
            } else {
                let projected: Result<Vec<Value>, DkitError> = arr
                    .into_iter()
                    .map(|item| select_exprs(&item, exprs))
                    .collect();
                Ok(Value::Array(projected?))
            }
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

/// 배열에서 숫자 필드 값을 추출하는 헬퍼
fn collect_numeric_values(arr: &[Value], field: &str) -> Result<Vec<f64>, DkitError> {
    let mut values = Vec::new();
    for item in arr {
        match item {
            Value::Object(map) => match map.get(field) {
                Some(Value::Integer(n)) => values.push(*n as f64),
                Some(Value::Float(f)) => values.push(*f),
                Some(Value::Null) | None => {}
                Some(v) => {
                    return Err(DkitError::QueryError(format!(
                        "field '{}' is not numeric (got {})",
                        field, v
                    )));
                }
            },
            _ => {
                return Err(DkitError::QueryError(
                    "requires an array of objects".to_string(),
                ));
            }
        }
    }
    Ok(values)
}

/// 아이템 슬라이스에서 숫자 필드 값을 추출하는 헬퍼 (group aggregate용)
fn collect_numeric_values_from_items(items: &[Value], field: &str) -> Vec<f64> {
    let mut values = Vec::new();
    for item in items {
        if let Value::Object(map) = item {
            match map.get(field) {
                Some(Value::Integer(n)) => values.push(*n as f64),
                Some(Value::Float(f)) => values.push(*f),
                _ => {}
            }
        }
    }
    values
}

/// percentile 계산 헬퍼 (linear interpolation)
fn compute_percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.len() == 1 {
        return sorted[0];
    }
    let index = p * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = index - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

/// median: 배열에서 지정된 숫자 필드의 중앙값을 반환
fn apply_median(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut values = collect_numeric_values(&arr, field)?;
            if values.is_empty() {
                return Ok(Value::Null);
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                Ok(Value::Float((values[mid - 1] + values[mid]) / 2.0))
            } else {
                Ok(Value::Float(values[mid]))
            }
        }
        _ => Err(DkitError::QueryError(
            "median requires an array input".to_string(),
        )),
    }
}

/// percentile: 배열에서 지정된 숫자 필드의 p번째 백분위수를 반환
fn apply_percentile(value: Value, field: &str, p: f64) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut values = collect_numeric_values(&arr, field)?;
            if values.is_empty() {
                return Ok(Value::Null);
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Value::Float(compute_percentile(&values, p)))
        }
        _ => Err(DkitError::QueryError(
            "percentile requires an array input".to_string(),
        )),
    }
}

/// stddev: 배열에서 지정된 숫자 필드의 표준편차(모집단)를 반환
fn apply_stddev(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let values = collect_numeric_values(&arr, field)?;
            if values.is_empty() {
                return Ok(Value::Null);
            }
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            Ok(Value::Float(variance.sqrt()))
        }
        _ => Err(DkitError::QueryError(
            "stddev requires an array input".to_string(),
        )),
    }
}

/// variance: 배열에서 지정된 숫자 필드의 분산을 반환
fn apply_variance(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let values = collect_numeric_values(&arr, field)?;
            if values.is_empty() {
                return Ok(Value::Null);
            }
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            Ok(Value::Float(variance))
        }
        _ => Err(DkitError::QueryError(
            "variance requires an array input".to_string(),
        )),
    }
}

/// mode: 배열에서 지정된 필드의 최빈값을 반환
fn apply_mode(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut freq: Vec<(Value, usize)> = Vec::new();
            for item in &arr {
                match item {
                    Value::Object(map) => {
                        if let Some(v) = map.get(field) {
                            if matches!(v, Value::Null) {
                                continue;
                            }
                            if let Some(entry) = freq.iter_mut().find(|(val, _)| val == v) {
                                entry.1 += 1;
                            } else {
                                freq.push((v.clone(), 1));
                            }
                        }
                    }
                    _ => {
                        return Err(DkitError::QueryError(
                            "mode requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            if freq.is_empty() {
                return Ok(Value::Null);
            }
            let max_count = freq.iter().map(|(_, c)| *c).max().unwrap_or(0);
            let mode_val = freq.into_iter().find(|(_, c)| *c == max_count).unwrap().0;
            Ok(mode_val)
        }
        _ => Err(DkitError::QueryError(
            "mode requires an array input".to_string(),
        )),
    }
}

/// group_concat: 배열에서 지정된 필드의 값을 문자열로 연결
fn apply_group_concat(value: Value, field: &str, separator: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut parts: Vec<String> = Vec::new();
            for item in &arr {
                match item {
                    Value::Object(map) => {
                        if let Some(v) = map.get(field) {
                            match v {
                                Value::Null => {}
                                Value::String(s) => parts.push(s.clone()),
                                Value::Integer(n) => parts.push(n.to_string()),
                                Value::Float(f) => parts.push(f.to_string()),
                                Value::Bool(b) => parts.push(b.to_string()),
                                _ => parts.push(format!("{}", v)),
                            }
                        }
                    }
                    _ => {
                        return Err(DkitError::QueryError(
                            "group_concat requires an array of objects".to_string(),
                        ));
                    }
                }
            }
            Ok(Value::String(parts.join(separator)))
        }
        _ => Err(DkitError::QueryError(
            "group_concat requires an array input".to_string(),
        )),
    }
}

/// unique: 전체 레코드 동일성 기준으로 중복 제거 (첫 번째 등장 유지)
fn apply_unique(value: Value) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut seen: Vec<Value> = Vec::new();
            let mut result = Vec::new();
            for item in arr {
                if !seen.contains(&item) {
                    seen.push(item.clone());
                    result.push(item);
                }
            }
            Ok(Value::Array(result))
        }
        _ => Err(DkitError::QueryError(
            "unique requires an array input".to_string(),
        )),
    }
}

/// unique_by: 특정 필드 기준으로 중복 제거 (첫 번째 등장 레코드 유지)
fn apply_unique_by(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut seen: Vec<Value> = Vec::new();
            let mut result = Vec::new();
            for item in arr {
                let key = match &item {
                    Value::Object(map) => map.get(field).cloned().unwrap_or(Value::Null),
                    _ => {
                        return Err(DkitError::QueryError(
                            "unique-by requires an array of objects".to_string(),
                        ));
                    }
                };
                if !seen.contains(&key) {
                    seen.push(key);
                    result.push(item);
                }
            }
            Ok(Value::Array(result))
        }
        _ => Err(DkitError::QueryError(
            "unique-by requires an array input".to_string(),
        )),
    }
}

/// add_field: 각 레코드에 새 필드를 추가 (computed column)
fn apply_add_field(value: Value, name: &str, expr: &Expr) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    Value::Object(mut map) => {
                        let computed = evaluate_expr(&Value::Object(map.clone()), expr)?;
                        map.insert(name.to_string(), computed);
                        result.push(Value::Object(map));
                    }
                    other => {
                        // Non-object items are passed through unchanged
                        result.push(other);
                    }
                }
            }
            Ok(Value::Array(result))
        }
        Value::Object(mut map) => {
            let computed = evaluate_expr(&Value::Object(map.clone()), expr)?;
            map.insert(name.to_string(), computed);
            Ok(Value::Object(map))
        }
        _ => Err(DkitError::QueryError(
            "add-field requires an array or object input".to_string(),
        )),
    }
}

/// map_field: 기존 필드의 값을 표현식 결과로 변환 (값 덮어쓰기)
fn apply_map_field(value: Value, name: &str, expr: &Expr) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    Value::Object(mut map) => {
                        let computed = evaluate_expr(&Value::Object(map.clone()), expr)?;
                        map.insert(name.to_string(), computed);
                        result.push(Value::Object(map));
                    }
                    other => {
                        result.push(other);
                    }
                }
            }
            Ok(Value::Array(result))
        }
        Value::Object(mut map) => {
            let computed = evaluate_expr(&Value::Object(map.clone()), expr)?;
            map.insert(name.to_string(), computed);
            Ok(Value::Object(map))
        }
        _ => Err(DkitError::QueryError(
            "map requires an array or object input".to_string(),
        )),
    }
}

/// explode: 배열 필드를 개별 행으로 펼침 (unnest/flatten)
/// 예: [{name:"a", tags:["x","y"]}] → [{name:"a", tags:"x"}, {name:"a", tags:"y"}]
/// 빈 배열인 경우 해당 레코드 제외
fn apply_explode(value: Value, field: &str) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                match item {
                    Value::Object(ref map) => {
                        match map.get(field) {
                            Some(Value::Array(elements)) => {
                                if elements.is_empty() {
                                    // 빈 배열: 해당 레코드 제외
                                    continue;
                                }
                                for element in elements {
                                    let mut new_map = map.clone();
                                    new_map.insert(field.to_string(), element.clone());
                                    result.push(Value::Object(new_map));
                                }
                            }
                            Some(_) => {
                                // 배열이 아닌 값: 그대로 유지
                                result.push(item.clone());
                            }
                            None => {
                                // 필드가 없는 경우: 해당 레코드 제외
                                continue;
                            }
                        }
                    }
                    other => {
                        result.push(other);
                    }
                }
            }
            Ok(Value::Array(result))
        }
        _ => Err(DkitError::QueryError(
            "explode requires an array input".to_string(),
        )),
    }
}

/// unpivot (wide → long): 지정 컬럼들을 key-value 쌍으로 변환
/// 예: [{name:"a", jan:100, feb:200}] → [{name:"a", variable:"jan", value:100}, {name:"a", variable:"feb", value:200}]
fn apply_unpivot(
    value: Value,
    value_columns: &[String],
    key_name: &str,
    value_name: &str,
) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                match item {
                    Value::Object(ref map) => {
                        for col in value_columns {
                            let mut new_map = IndexMap::new();
                            // 먼저 unpivot 대상이 아닌 필드(id 변수)를 복사
                            for (k, v) in map {
                                if !value_columns.contains(k) {
                                    new_map.insert(k.clone(), v.clone());
                                }
                            }
                            // key 컬럼에 원래 컬럼명 저장
                            new_map.insert(key_name.to_string(), Value::String(col.clone()));
                            // value 컬럼에 해당 값 저장 (없으면 Null)
                            let val = map.get(col).cloned().unwrap_or(Value::Null);
                            new_map.insert(value_name.to_string(), val);
                            result.push(Value::Object(new_map));
                        }
                    }
                    other => {
                        result.push(other);
                    }
                }
            }
            Ok(Value::Array(result))
        }
        _ => Err(DkitError::QueryError(
            "unpivot requires an array input".to_string(),
        )),
    }
}

/// pivot (long → wide): key-value 쌍을 컬럼으로 변환
/// 예: [{name:"a", month:"jan", sales:100}, {name:"a", month:"feb", sales:200}]
///   → [{name:"a", jan:100, feb:200}]
fn apply_pivot(
    value: Value,
    index_fields: &[String],
    columns_field: &str,
    values_field: &str,
) -> Result<Value, DkitError> {
    match value {
        Value::Array(arr) => {
            // 1단계: index 키별로 그룹화
            let mut groups: IndexMap<String, IndexMap<String, Value>> = IndexMap::new();
            // 모든 pivot 컬럼명 수집 (출력 순서 보존)
            let mut all_pivot_cols: Vec<String> = Vec::new();

            for item in &arr {
                if let Value::Object(map) = item {
                    // index 키 생성
                    let index_key = index_fields
                        .iter()
                        .map(|f| map.get(f).map(|v| format!("{v}")).unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join("\x1f");

                    // 새 컬럼명 (columns_field의 값)
                    let col_name = match map.get(columns_field) {
                        Some(v) => match v {
                            Value::String(s) => s.clone(),
                            other => format!("{other}"),
                        },
                        None => continue,
                    };

                    // 새 컬럼에 채울 값
                    let col_value = map.get(values_field).cloned().unwrap_or(Value::Null);

                    // 그룹 엔트리 가져오기 또는 생성
                    let entry = groups.entry(index_key.clone()).or_insert_with(|| {
                        let mut base = IndexMap::new();
                        for f in index_fields {
                            if let Some(v) = map.get(f) {
                                base.insert(f.clone(), v.clone());
                            }
                        }
                        base
                    });

                    if !all_pivot_cols.contains(&col_name) {
                        all_pivot_cols.push(col_name.clone());
                    }

                    entry.insert(col_name, col_value);
                }
            }

            // 2단계: 그룹을 결과 배열로 변환
            // 빈 pivot 컬럼은 Null로 채움
            let result: Vec<Value> = groups
                .into_values()
                .map(|mut map| {
                    for col in &all_pivot_cols {
                        map.entry(col.clone()).or_insert(Value::Null);
                    }
                    Value::Object(map)
                })
                .collect();

            Ok(Value::Array(result))
        }
        _ => Err(DkitError::QueryError(
            "pivot requires an array input".to_string(),
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
        AggregateFunc::Median => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("median() requires a field argument".to_string())
            })?;
            let mut values = collect_numeric_values_from_items(items, f);
            if values.is_empty() {
                return Ok(Value::Null);
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let mid = values.len() / 2;
            if values.len() % 2 == 0 {
                Ok(Value::Float((values[mid - 1] + values[mid]) / 2.0))
            } else {
                Ok(Value::Float(values[mid]))
            }
        }
        AggregateFunc::Percentile(p) => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("percentile() requires a field argument".to_string())
            })?;
            let mut values = collect_numeric_values_from_items(items, f);
            if values.is_empty() {
                return Ok(Value::Null);
            }
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Value::Float(compute_percentile(&values, *p)))
        }
        AggregateFunc::Stddev => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("stddev() requires a field argument".to_string())
            })?;
            let values = collect_numeric_values_from_items(items, f);
            if values.is_empty() {
                return Ok(Value::Null);
            }
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            Ok(Value::Float(variance.sqrt()))
        }
        AggregateFunc::Variance => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("variance() requires a field argument".to_string())
            })?;
            let values = collect_numeric_values_from_items(items, f);
            if values.is_empty() {
                return Ok(Value::Null);
            }
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance =
                values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            Ok(Value::Float(variance))
        }
        AggregateFunc::Mode => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("mode() requires a field argument".to_string())
            })?;
            let mut freq: Vec<(Value, usize)> = Vec::new();
            for item in items {
                if let Value::Object(map) = item {
                    if let Some(v) = map.get(f) {
                        if matches!(v, Value::Null) {
                            continue;
                        }
                        if let Some(entry) = freq.iter_mut().find(|(val, _)| val == v) {
                            entry.1 += 1;
                        } else {
                            freq.push((v.clone(), 1));
                        }
                    }
                }
            }
            if freq.is_empty() {
                return Ok(Value::Null);
            }
            let max_count = freq.iter().map(|(_, c)| *c).max().unwrap_or(0);
            let mode_val = freq.into_iter().find(|(_, c)| *c == max_count).unwrap().0;
            Ok(mode_val)
        }
        AggregateFunc::GroupConcat(separator) => {
            let f = field.ok_or_else(|| {
                DkitError::QueryError("group_concat() requires a field argument".to_string())
            })?;
            let mut parts: Vec<String> = Vec::new();
            for item in items {
                if let Value::Object(map) = item {
                    if let Some(v) = map.get(f) {
                        match v {
                            Value::Null => {}
                            Value::String(s) => parts.push(s.clone()),
                            Value::Integer(n) => parts.push(n.to_string()),
                            Value::Float(fv) => parts.push(fv.to_string()),
                            Value::Bool(b) => parts.push(b.to_string()),
                            _ => parts.push(format!("{}", v)),
                        }
                    }
                }
            }
            Ok(Value::String(parts.join(separator)))
        }
    }
}

/// 윈도우 함수가 포함된 SELECT를 처리: 전체 배열을 한꺼번에 처리
fn apply_select_with_window(arr: Vec<Value>, exprs: &[SelectExpr]) -> Result<Value, DkitError> {
    let len = arr.len();

    // 각 SelectExpr에 대해 윈도우 함수면 미리 전체 배열에 대한 결과를 계산
    let mut window_results: Vec<Option<Vec<Value>>> = Vec::with_capacity(exprs.len());

    for se in exprs {
        if let Expr::Window { func, over } = &se.expr {
            let results = evaluate_window_func(&arr, func, over)?;
            window_results.push(Some(results));
        } else {
            window_results.push(None);
        }
    }

    // 행별로 결과 객체 조합
    let mut output = Vec::with_capacity(len);
    for (i, item) in arr.iter().enumerate() {
        match item {
            Value::Object(map) => {
                let mut new_map = IndexMap::new();
                for (j, se) in exprs.iter().enumerate() {
                    let key = se
                        .alias
                        .clone()
                        .unwrap_or_else(|| expr_default_key(&se.expr));
                    if let Some(ref win_vals) = window_results[j] {
                        new_map.insert(key, win_vals[i].clone());
                    } else {
                        // 일반 표현식: 필드가 없으면 스킵
                        if let Expr::Field(fname) = &se.expr {
                            if se.alias.is_none() && !map.contains_key(fname) {
                                continue;
                            }
                        }
                        let val = evaluate_expr(item, &se.expr)?;
                        new_map.insert(key, val);
                    }
                }
                output.push(Value::Object(new_map));
            }
            _ => output.push(item.clone()),
        }
    }

    Ok(Value::Array(output))
}

/// 윈도우 함수를 전체 배열에 대해 평가하여 각 행의 결과값 벡터를 반환
fn evaluate_window_func(
    arr: &[Value],
    func: &WindowFunc,
    spec: &WindowSpec,
) -> Result<Vec<Value>, DkitError> {
    let len = arr.len();

    // 파티션별 인덱스 그룹 생성
    let partitions = build_partitions(arr, &spec.partition_by);

    // 각 파티션 내에서 ORDER BY로 정렬된 인덱스 계산
    let sorted_partitions: Vec<Vec<usize>> = partitions
        .iter()
        .map(|indices| sort_partition(arr, indices, &spec.order_by))
        .collect();

    let mut results = vec![Value::Null; len];

    for sorted_indices in &sorted_partitions {
        match func {
            WindowFunc::RowNumber => {
                for (rank, &idx) in sorted_indices.iter().enumerate() {
                    results[idx] = Value::Integer((rank + 1) as i64);
                }
            }
            WindowFunc::Rank => {
                compute_rank(arr, sorted_indices, &spec.order_by, false, &mut results);
            }
            WindowFunc::DenseRank => {
                compute_rank(arr, sorted_indices, &spec.order_by, true, &mut results);
            }
            WindowFunc::Lag { expr, offset } => {
                for (pos, &idx) in sorted_indices.iter().enumerate() {
                    if pos >= *offset {
                        let prev_idx = sorted_indices[pos - offset];
                        results[idx] = evaluate_expr(&arr[prev_idx], expr)?;
                    }
                    // else remains Null
                }
            }
            WindowFunc::Lead { expr, offset } => {
                for (pos, &idx) in sorted_indices.iter().enumerate() {
                    if pos + offset < sorted_indices.len() {
                        let next_idx = sorted_indices[pos + offset];
                        results[idx] = evaluate_expr(&arr[next_idx], expr)?;
                    }
                    // else remains Null
                }
            }
            WindowFunc::FirstValue { expr } => {
                if !sorted_indices.is_empty() {
                    let first_idx = sorted_indices[0];
                    let first_val = evaluate_expr(&arr[first_idx], expr)?;
                    for &idx in sorted_indices {
                        results[idx] = first_val.clone();
                    }
                }
            }
            WindowFunc::LastValue { expr } => {
                if !sorted_indices.is_empty() {
                    let last_idx = sorted_indices[sorted_indices.len() - 1];
                    let last_val = evaluate_expr(&arr[last_idx], expr)?;
                    for &idx in sorted_indices {
                        results[idx] = last_val.clone();
                    }
                }
            }
            WindowFunc::Aggregate { func: agg, expr } => {
                compute_window_aggregate(arr, sorted_indices, agg, expr, &mut results)?;
            }
        }
    }

    Ok(results)
}

/// 파티션 구축: PARTITION BY 필드 값 기준으로 행 인덱스를 그룹화
fn build_partitions(arr: &[Value], partition_by: &[String]) -> Vec<Vec<usize>> {
    if partition_by.is_empty() {
        // 파티션 없음: 전체 배열이 하나의 파티션
        return vec![(0..arr.len()).collect()];
    }

    let mut groups: Vec<(Vec<Value>, Vec<usize>)> = Vec::new();

    for (i, item) in arr.iter().enumerate() {
        let key: Vec<Value> = partition_by
            .iter()
            .map(|f| {
                if let Value::Object(map) = item {
                    map.get(f).cloned().unwrap_or(Value::Null)
                } else {
                    Value::Null
                }
            })
            .collect();

        if let Some(group) = groups.iter_mut().find(|(k, _)| *k == key) {
            group.1.push(i);
        } else {
            groups.push((key, vec![i]));
        }
    }

    groups.into_iter().map(|(_, indices)| indices).collect()
}

/// 파티션 내 행들을 ORDER BY 기준으로 정렬
fn sort_partition(
    arr: &[Value],
    indices: &[usize],
    order_by: &[crate::query::parser::WindowOrderBy],
) -> Vec<usize> {
    let mut sorted = indices.to_vec();
    if order_by.is_empty() {
        return sorted;
    }
    sorted.sort_by(|&a, &b| {
        for ob in order_by {
            let va = extract_sort_key(&arr[a], &ob.field);
            let vb = extract_sort_key(&arr[b], &ob.field);
            let cmp = compare_sort_keys(&va, &vb);
            let cmp = if ob.descending { cmp.reverse() } else { cmp };
            if cmp != std::cmp::Ordering::Equal {
                return cmp;
            }
        }
        std::cmp::Ordering::Equal
    });
    sorted
}

/// RANK / DENSE_RANK 계산
fn compute_rank(
    arr: &[Value],
    sorted_indices: &[usize],
    order_by: &[crate::query::parser::WindowOrderBy],
    dense: bool,
    results: &mut [Value],
) {
    if sorted_indices.is_empty() {
        return;
    }

    let get_order_values = |idx: usize| -> Vec<Option<Value>> {
        order_by
            .iter()
            .map(|ob| extract_sort_key(&arr[idx], &ob.field))
            .collect()
    };

    let mut current_rank: i64 = 1;
    let mut prev_values = get_order_values(sorted_indices[0]);
    results[sorted_indices[0]] = Value::Integer(1);

    for (pos, &idx) in sorted_indices.iter().enumerate().skip(1) {
        let current_values = get_order_values(idx);
        if current_values == prev_values {
            // 동점: 같은 순위
            results[idx] = Value::Integer(current_rank);
        } else {
            // 새 순위
            if dense {
                current_rank += 1;
            } else {
                current_rank = (pos + 1) as i64;
            }
            results[idx] = Value::Integer(current_rank);
            prev_values = current_values;
        }
    }
}

/// 윈도우 집계 함수 (running aggregate over the entire partition)
fn compute_window_aggregate(
    arr: &[Value],
    sorted_indices: &[usize],
    agg: &AggregateFunc,
    expr: &Expr,
    results: &mut [Value],
) -> Result<(), DkitError> {
    // 전체 파티션 집계를 계산하여 모든 행에 할당
    let mut values = Vec::with_capacity(sorted_indices.len());
    for &idx in sorted_indices {
        values.push(evaluate_expr(&arr[idx], expr)?);
    }

    let agg_result = match agg {
        AggregateFunc::Count => {
            let count = values.iter().filter(|v| !matches!(v, Value::Null)).count();
            Value::Integer(count as i64)
        }
        AggregateFunc::Sum => {
            let mut sum_int: i64 = 0;
            let mut sum_float: f64 = 0.0;
            let mut has_float = false;
            for v in &values {
                match v {
                    Value::Integer(n) => {
                        sum_int += n;
                        sum_float += *n as f64;
                    }
                    Value::Float(f) => {
                        sum_float += f;
                        has_float = true;
                    }
                    _ => {}
                }
            }
            if has_float {
                Value::Float(sum_float)
            } else {
                Value::Integer(sum_int)
            }
        }
        AggregateFunc::Avg => {
            let mut sum: f64 = 0.0;
            let mut count = 0;
            for v in &values {
                match v {
                    Value::Integer(n) => {
                        sum += *n as f64;
                        count += 1;
                    }
                    Value::Float(f) => {
                        sum += f;
                        count += 1;
                    }
                    _ => {}
                }
            }
            if count > 0 {
                Value::Float(sum / count as f64)
            } else {
                Value::Null
            }
        }
        AggregateFunc::Min => {
            let mut min_val = Value::Null;
            for v in &values {
                if matches!(v, Value::Null) {
                    continue;
                }
                if matches!(min_val, Value::Null)
                    || compare_value_ordering(v, &min_val) == std::cmp::Ordering::Less
                {
                    min_val = v.clone();
                }
            }
            min_val
        }
        AggregateFunc::Max => {
            let mut max_val = Value::Null;
            for v in &values {
                if matches!(v, Value::Null) {
                    continue;
                }
                if matches!(max_val, Value::Null)
                    || compare_value_ordering(v, &max_val) == std::cmp::Ordering::Greater
                {
                    max_val = v.clone();
                }
            }
            max_val
        }
        _ => {
            return Err(DkitError::QueryError(format!(
                "unsupported window aggregate function: {:?}",
                agg
            )));
        }
    };

    for &idx in sorted_indices {
        results[idx] = agg_result.clone();
    }

    Ok(())
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
pub(crate) fn evaluate_condition(value: &Value, condition: &Condition) -> Result<bool, DkitError> {
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
    // IN / NOT IN: 리스트 내 포함 여부 확인
    if let LiteralValue::List(list) = literal {
        let contained = list
            .iter()
            .any(|item| compare_values(field, &CompareOp::Eq, item).unwrap_or(false));
        return match op {
            CompareOp::In => Ok(contained),
            CompareOp::NotIn => Ok(!contained),
            _ => Err(DkitError::QueryError(
                "list values only support 'in' and 'not in' operators".to_string(),
            )),
        };
    }

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
            CompareOp::Matches => {
                let re = regex::Regex::new(b).map_err(|e| {
                    DkitError::QueryError(format!("invalid regex pattern '{}': {}", b, e))
                })?;
                Ok(re.is_match(a))
            }
            CompareOp::NotMatches => {
                let re = regex::Regex::new(b).map_err(|e| {
                    DkitError::QueryError(format!("invalid regex pattern '{}': {}", b, e))
                })?;
                Ok(!re.is_match(a))
            }
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
        CompareOp::In | CompareOp::NotIn => false,
        CompareOp::Matches | CompareOp::NotMatches => false,
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

    #[test]
    fn test_where_matches() {
        let data = sample_files();
        let cond = make_condition(
            "email",
            CompareOp::Matches,
            LiteralValue::String(".*@gmail\\.com$".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2); // alice@gmail.com, charlie@gmail.com
    }

    #[test]
    fn test_where_matches_pattern() {
        let data = sample_files();
        let cond = make_condition(
            "name",
            CompareOp::Matches,
            LiteralValue::String("^.*\\.json$".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 2); // config.json, data.json
    }

    #[test]
    fn test_where_not_matches() {
        let data = sample_files();
        let cond = make_condition(
            "email",
            CompareOp::NotMatches,
            LiteralValue::String(".*@gmail\\.com$".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 1); // bob@yahoo.com
    }

    #[test]
    fn test_where_matches_no_match() {
        let data = sample_files();
        let cond = make_condition(
            "email",
            CompareOp::Matches,
            LiteralValue::String("^admin@".to_string()),
        );
        let result = run_where(&data, &cond).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_where_matches_invalid_regex() {
        let result = compare_values(
            &Value::String("test@example.com".to_string()),
            &CompareOp::Matches,
            &LiteralValue::String("[invalid".to_string()),
        );
        assert!(result.is_err());
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

    // --- unique 테스트 ---

    #[test]
    fn test_unique_removes_exact_duplicates() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("city".to_string(), Value::String("Seoul".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("city".to_string(), Value::String("Tokyo".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("city".to_string(), Value::String("Seoul".to_string()));
                Value::Object(m)
            },
        ]);
        let result = apply_unique(data).unwrap();
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
    fn test_unique_no_duplicates() {
        let data = sample_users();
        let result = apply_unique(data).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_unique_with_primitives() {
        let data = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(1),
            Value::Integer(3),
            Value::Integer(2),
        ]);
        let result = apply_unique(data).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], Value::Integer(1));
        assert_eq!(arr[1], Value::Integer(2));
        assert_eq!(arr[2], Value::Integer(3));
    }

    #[test]
    fn test_unique_non_array_error() {
        let data = Value::String("hello".to_string());
        assert!(apply_unique(data).is_err());
    }

    // --- unique_by 테스트 ---

    #[test]
    fn test_unique_by_field() {
        let data = sample_users(); // Alice(Seoul), Bob(Busan), Charlie(Seoul)
        let result = apply_unique_by(data, "city").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice(Seoul), Bob(Busan) — Charlie dropped
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
    fn test_unique_by_all_different() {
        let data = sample_users();
        let result = apply_unique_by(data, "name").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3); // all unique names
    }

    #[test]
    fn test_unique_by_missing_field() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                Value::Object(m)
            },
        ]);
        let result = apply_unique_by(data, "nonexistent").unwrap();
        let arr = result.as_array().unwrap();
        // Both have Null for the key, so only the first is kept
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn test_unique_by_non_array_error() {
        let data = Value::String("hello".to_string());
        assert!(apply_unique_by(data, "field").is_err());
    }

    #[test]
    fn test_unique_by_non_object_elements_error() {
        let data = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        assert!(apply_unique_by(data, "field").is_err());
    }

    // --- add_field tests ---

    #[test]
    fn test_add_field_arithmetic() {
        use crate::query::parser::parse_add_field_expr;

        let data = Value::Array(vec![
            Value::Object(IndexMap::from([
                ("amount".to_string(), Value::Integer(10)),
                ("quantity".to_string(), Value::Integer(3)),
            ])),
            Value::Object(IndexMap::from([
                ("amount".to_string(), Value::Integer(20)),
                ("quantity".to_string(), Value::Integer(5)),
            ])),
        ]);

        let (name, expr) = parse_add_field_expr("total = amount * quantity").unwrap();
        let result = apply_add_field(data, &name, &expr).unwrap();

        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            if let Value::Object(map) = &arr[0] {
                assert_eq!(map.get("total"), Some(&Value::Integer(30)));
            } else {
                panic!("expected object");
            }
            if let Value::Object(map) = &arr[1] {
                assert_eq!(map.get("total"), Some(&Value::Integer(100)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_add_field_string_concat() {
        use crate::query::parser::parse_add_field_expr;

        let data = Value::Array(vec![Value::Object(IndexMap::from([
            ("first".to_string(), Value::String("John".to_string())),
            ("last".to_string(), Value::String("Doe".to_string())),
        ]))]);

        let (name, expr) = parse_add_field_expr("full = first + \" \" + last").unwrap();
        let result = apply_add_field(data, &name, &expr).unwrap();

        if let Value::Array(arr) = result {
            if let Value::Object(map) = &arr[0] {
                assert_eq!(
                    map.get("full"),
                    Some(&Value::String("John Doe".to_string()))
                );
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_add_field_with_function() {
        use crate::query::parser::parse_add_field_expr;

        let data = Value::Array(vec![Value::Object(IndexMap::from([(
            "name".to_string(),
            Value::String("hello".to_string()),
        )]))]);

        let (name, expr) = parse_add_field_expr("upper_name = upper(name)").unwrap();
        let result = apply_add_field(data, &name, &expr).unwrap();

        if let Value::Array(arr) = result {
            if let Value::Object(map) = &arr[0] {
                assert_eq!(
                    map.get("upper_name"),
                    Some(&Value::String("HELLO".to_string()))
                );
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_add_field_on_single_object() {
        use crate::query::parser::parse_add_field_expr;

        let data = Value::Object(IndexMap::from([("price".to_string(), Value::Float(100.0))]));

        let (name, expr) = parse_add_field_expr("tax = price * 0.1").unwrap();
        let result = apply_add_field(data, &name, &expr).unwrap();

        if let Value::Object(map) = result {
            assert_eq!(map.get("tax"), Some(&Value::Float(10.0)));
        } else {
            panic!("expected object");
        }
    }

    #[test]
    fn test_add_field_preserves_existing_fields() {
        use crate::query::parser::parse_add_field_expr;

        let data = Value::Array(vec![Value::Object(IndexMap::from([
            ("a".to_string(), Value::Integer(1)),
            ("b".to_string(), Value::Integer(2)),
        ]))]);

        let (name, expr) = parse_add_field_expr("c = a + b").unwrap();
        let result = apply_add_field(data, &name, &expr).unwrap();

        if let Value::Array(arr) = result {
            if let Value::Object(map) = &arr[0] {
                assert_eq!(map.len(), 3);
                assert_eq!(map.get("a"), Some(&Value::Integer(1)));
                assert_eq!(map.get("b"), Some(&Value::Integer(2)));
                assert_eq!(map.get("c"), Some(&Value::Integer(3)));
            } else {
                panic!("expected object");
            }
        } else {
            panic!("expected array");
        }
    }

    // --- IN / NOT IN ---

    #[test]
    fn test_where_in_string() {
        let data = sample_users();
        let cond = make_condition(
            "city",
            CompareOp::In,
            LiteralValue::List(vec![
                LiteralValue::String("Seoul".to_string()),
                LiteralValue::String("Tokyo".to_string()),
            ]),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice, Charlie
    }

    #[test]
    fn test_where_not_in_string() {
        let data = sample_users();
        let cond = make_condition(
            "city",
            CompareOp::NotIn,
            LiteralValue::List(vec![LiteralValue::String("Seoul".to_string())]),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1); // Bob
        assert_eq!(
            arr[0].as_object().unwrap().get("name").unwrap(),
            &Value::String("Bob".to_string())
        );
    }

    #[test]
    fn test_where_in_integer() {
        let data = sample_users();
        let cond = make_condition(
            "age",
            CompareOp::In,
            LiteralValue::List(vec![LiteralValue::Integer(25), LiteralValue::Integer(35)]),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Bob, Charlie
    }

    #[test]
    fn test_where_not_in_integer() {
        let data = sample_users();
        let cond = make_condition(
            "age",
            CompareOp::NotIn,
            LiteralValue::List(vec![LiteralValue::Integer(25), LiteralValue::Integer(35)]),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1); // Alice (age 30)
    }

    #[test]
    fn test_where_in_empty_list() {
        let data = sample_users();
        let cond = make_condition("city", CompareOp::In, LiteralValue::List(vec![]));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_where_not_in_empty_list() {
        let data = sample_users();
        let cond = make_condition("city", CompareOp::NotIn, LiteralValue::List(vec![]));
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3); // all users
    }

    #[test]
    fn test_where_in_no_match() {
        let data = sample_users();
        let cond = make_condition(
            "city",
            CompareOp::In,
            LiteralValue::List(vec![
                LiteralValue::String("Tokyo".to_string()),
                LiteralValue::String("Osaka".to_string()),
            ]),
        );
        let result = run_where(&data, &cond).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_parse_in_query() {
        let query = parse_query(".[] | where city in (\"Seoul\", \"Busan\")").unwrap();
        let data = sample_users();
        let result = {
            let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
            apply_operations(path_result, &query.operations)
        }
        .unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3); // all users are in Seoul or Busan
    }

    #[test]
    fn test_parse_not_in_query() {
        let query = parse_query(".[] | where city not in (\"Seoul\")").unwrap();
        let data = sample_users();
        let result = {
            let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
            apply_operations(path_result, &query.operations)
        }
        .unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1); // only Bob (Busan)
    }

    #[test]
    fn test_parse_in_with_integers() {
        let query = parse_query(".[] | where age in (25, 35)").unwrap();
        let data = sample_users();
        let result = {
            let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
            apply_operations(path_result, &query.operations)
        }
        .unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Bob, Charlie
    }

    #[test]
    fn test_in_combined_with_and() {
        let query = parse_query(".[] | where city in (\"Seoul\", \"Busan\") and age > 28").unwrap();
        let data = sample_users();
        let result = {
            let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
            apply_operations(path_result, &query.operations)
        }
        .unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2); // Alice (30, Seoul), Charlie (35, Seoul)
    }

    // --- explode tests ---

    fn sample_with_tags() -> Value {
        Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert(
                    "tags".to_string(),
                    Value::Array(vec![
                        Value::String("x".to_string()),
                        Value::String("y".to_string()),
                    ]),
                );
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert(
                    "tags".to_string(),
                    Value::Array(vec![Value::String("z".to_string())]),
                );
                Value::Object(m)
            },
        ])
    }

    #[test]
    fn test_explode_basic() {
        let result = apply_explode(sample_with_tags(), "tags").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(
            arr[0].as_object().unwrap()["name"],
            Value::String("Alice".to_string())
        );
        assert_eq!(
            arr[0].as_object().unwrap()["tags"],
            Value::String("x".to_string())
        );
        assert_eq!(
            arr[1].as_object().unwrap()["tags"],
            Value::String("y".to_string())
        );
        assert_eq!(
            arr[2].as_object().unwrap()["name"],
            Value::String("Bob".to_string())
        );
        assert_eq!(
            arr[2].as_object().unwrap()["tags"],
            Value::String("z".to_string())
        );
    }

    #[test]
    fn test_explode_empty_array_excluded() {
        let value = Value::Array(vec![{
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("tags".to_string(), Value::Array(vec![]));
            Value::Object(m)
        }]);
        let result = apply_explode(value, "tags").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_explode_missing_field_excluded() {
        let value = Value::Array(vec![{
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            Value::Object(m)
        }]);
        let result = apply_explode(value, "tags").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_explode_non_array_field_kept() {
        let value = Value::Array(vec![{
            let mut m = IndexMap::new();
            m.insert("name".to_string(), Value::String("Alice".to_string()));
            m.insert("tags".to_string(), Value::String("single".to_string()));
            Value::Object(m)
        }]);
        let result = apply_explode(value, "tags").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].as_object().unwrap()["tags"],
            Value::String("single".to_string())
        );
    }

    #[test]
    fn test_explode_non_array_input_error() {
        let value = Value::String("not an array".to_string());
        let result = apply_explode(value, "tags");
        assert!(result.is_err());
    }

    // --- unpivot tests ---

    #[test]
    fn test_unpivot_basic() {
        let data = Value::Array(vec![
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Alice".to_string())),
                ("jan".to_string(), Value::Integer(100)),
                ("feb".to_string(), Value::Integer(200)),
                ("mar".to_string(), Value::Integer(300)),
            ])),
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Bob".to_string())),
                ("jan".to_string(), Value::Integer(150)),
                ("feb".to_string(), Value::Integer(250)),
                ("mar".to_string(), Value::Integer(350)),
            ])),
        ]);

        let cols = vec!["jan".to_string(), "feb".to_string(), "mar".to_string()];
        let result = apply_unpivot(data, &cols, "month", "sales").unwrap();
        let arr = result.as_array().unwrap();

        // 2 rows × 3 columns = 6 rows
        assert_eq!(arr.len(), 6);

        // First row: Alice, jan, 100
        let first = arr[0].as_object().unwrap();
        assert_eq!(first["name"], Value::String("Alice".to_string()));
        assert_eq!(first["month"], Value::String("jan".to_string()));
        assert_eq!(first["sales"], Value::Integer(100));

        // Fourth row: Bob, jan, 150
        let fourth = arr[3].as_object().unwrap();
        assert_eq!(fourth["name"], Value::String("Bob".to_string()));
        assert_eq!(fourth["month"], Value::String("jan".to_string()));
        assert_eq!(fourth["sales"], Value::Integer(150));
    }

    #[test]
    fn test_unpivot_missing_column() {
        let data = Value::Array(vec![Value::Object(IndexMap::from([
            ("name".to_string(), Value::String("Alice".to_string())),
            ("jan".to_string(), Value::Integer(100)),
        ]))]);

        let cols = vec!["jan".to_string(), "feb".to_string()];
        let result = apply_unpivot(data, &cols, "variable", "value").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        // Missing column → Null
        assert_eq!(arr[1].as_object().unwrap()["value"], Value::Null);
    }

    #[test]
    fn test_unpivot_non_array_error() {
        let value = Value::String("not an array".to_string());
        let result = apply_unpivot(value, &["a".to_string()], "key", "val");
        assert!(result.is_err());
    }

    // --- pivot tests ---

    #[test]
    fn test_pivot_basic() {
        let data = Value::Array(vec![
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Alice".to_string())),
                ("month".to_string(), Value::String("jan".to_string())),
                ("sales".to_string(), Value::Integer(100)),
            ])),
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Alice".to_string())),
                ("month".to_string(), Value::String("feb".to_string())),
                ("sales".to_string(), Value::Integer(200)),
            ])),
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Bob".to_string())),
                ("month".to_string(), Value::String("jan".to_string())),
                ("sales".to_string(), Value::Integer(150)),
            ])),
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Bob".to_string())),
                ("month".to_string(), Value::String("feb".to_string())),
                ("sales".to_string(), Value::Integer(250)),
            ])),
        ]);

        let index = vec!["name".to_string()];
        let result = apply_pivot(data, &index, "month", "sales").unwrap();
        let arr = result.as_array().unwrap();

        assert_eq!(arr.len(), 2);

        let alice = arr[0].as_object().unwrap();
        assert_eq!(alice["name"], Value::String("Alice".to_string()));
        assert_eq!(alice["jan"], Value::Integer(100));
        assert_eq!(alice["feb"], Value::Integer(200));

        let bob = arr[1].as_object().unwrap();
        assert_eq!(bob["name"], Value::String("Bob".to_string()));
        assert_eq!(bob["jan"], Value::Integer(150));
        assert_eq!(bob["feb"], Value::Integer(250));
    }

    #[test]
    fn test_pivot_missing_values_filled_with_null() {
        let data = Value::Array(vec![
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Alice".to_string())),
                ("month".to_string(), Value::String("jan".to_string())),
                ("sales".to_string(), Value::Integer(100)),
            ])),
            Value::Object(IndexMap::from([
                ("name".to_string(), Value::String("Bob".to_string())),
                ("month".to_string(), Value::String("feb".to_string())),
                ("sales".to_string(), Value::Integer(250)),
            ])),
        ]);

        let index = vec!["name".to_string()];
        let result = apply_pivot(data, &index, "month", "sales").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);

        // Alice has jan but not feb → feb is Null
        let alice = arr[0].as_object().unwrap();
        assert_eq!(alice["jan"], Value::Integer(100));
        assert_eq!(alice["feb"], Value::Null);

        // Bob has feb but not jan → jan is Null
        let bob = arr[1].as_object().unwrap();
        assert_eq!(bob["jan"], Value::Null);
        assert_eq!(bob["feb"], Value::Integer(250));
    }

    #[test]
    fn test_pivot_non_array_error() {
        let value = Value::String("not an array".to_string());
        let result = apply_pivot(value, &["name".to_string()], "col", "val");
        assert!(result.is_err());
    }

    #[test]
    fn test_pivot_unpivot_roundtrip() {
        // Start with wide data
        let wide = Value::Array(vec![Value::Object(IndexMap::from([
            ("name".to_string(), Value::String("Alice".to_string())),
            ("jan".to_string(), Value::Integer(100)),
            ("feb".to_string(), Value::Integer(200)),
        ]))]);

        // Unpivot → long
        let cols = vec!["jan".to_string(), "feb".to_string()];
        let long = apply_unpivot(wide, &cols, "month", "sales").unwrap();
        let long_arr = long.as_array().unwrap();
        assert_eq!(long_arr.len(), 2);

        // Pivot back → wide
        let index = vec!["name".to_string()];
        let wide_again = apply_pivot(long, &index, "month", "sales").unwrap();
        let arr = wide_again.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let row = arr[0].as_object().unwrap();
        assert_eq!(row["name"], Value::String("Alice".to_string()));
        assert_eq!(row["jan"], Value::Integer(100));
        assert_eq!(row["feb"], Value::Integer(200));
    }

    // --- if() / case expression integration tests ---

    #[test]
    fn test_select_with_if_expr() {
        let data = sample_users();
        let q = parse_query(".[] | select name, if(age < 30, \"young\", \"senior\") as category")
            .unwrap();
        let result = apply_operations(data, &q.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        // Alice: age 30, not < 30 → "senior"
        assert_eq!(
            arr[0].as_object().unwrap()["category"],
            Value::String("senior".to_string())
        );
        // Bob: age 25, < 30 → "young"
        assert_eq!(
            arr[1].as_object().unwrap()["category"],
            Value::String("young".to_string())
        );
        // Charlie: age 35, not < 30 → "senior"
        assert_eq!(
            arr[2].as_object().unwrap()["category"],
            Value::String("senior".to_string())
        );
    }

    #[test]
    fn test_select_with_nested_if() {
        let data = sample_users();
        let q = parse_query(
            ".[] | select name, if(age < 28, \"young\", if(age < 33, \"mid\", \"senior\")) as cat",
        )
        .unwrap();
        let result = apply_operations(data, &q.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Alice: 30 → mid
        assert_eq!(
            arr[0].as_object().unwrap()["cat"],
            Value::String("mid".to_string())
        );
        // Bob: 25 → young
        assert_eq!(
            arr[1].as_object().unwrap()["cat"],
            Value::String("young".to_string())
        );
        // Charlie: 35 → senior
        assert_eq!(
            arr[2].as_object().unwrap()["cat"],
            Value::String("senior".to_string())
        );
    }

    #[test]
    fn test_select_with_case() {
        let data = sample_users();
        let q = parse_query(
            ".[] | select name, case when city == \"Seoul\" then \"capital\" when city == \"Busan\" then \"port\" else \"other\" end as city_type",
        )
        .unwrap();
        let result = apply_operations(data, &q.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap()["city_type"],
            Value::String("capital".to_string())
        );
        assert_eq!(
            arr[1].as_object().unwrap()["city_type"],
            Value::String("port".to_string())
        );
        assert_eq!(
            arr[2].as_object().unwrap()["city_type"],
            Value::String("capital".to_string())
        );
    }

    #[test]
    fn test_select_case_no_else_returns_null() {
        let data = sample_users();
        let q = parse_query(
            ".[] | select name, case when city == \"Tokyo\" then \"japan\" end as country",
        )
        .unwrap();
        let result = apply_operations(data, &q.operations).unwrap();
        let arr = result.as_array().unwrap();
        // None match → null
        for item in arr {
            assert_eq!(item.as_object().unwrap()["country"], Value::Null);
        }
    }

    #[test]
    fn test_add_field_with_if() {
        let data = sample_users();
        let (name, expr) =
            crate::query::parser::parse_add_field_expr("group = if(age >= 30, \"A\", \"B\")")
                .unwrap();
        let op = Operation::AddField { name, expr };
        let result = apply_operation(data, &op).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(
            arr[0].as_object().unwrap()["group"],
            Value::String("A".to_string())
        );
        assert_eq!(
            arr[1].as_object().unwrap()["group"],
            Value::String("B".to_string())
        );
        assert_eq!(
            arr[2].as_object().unwrap()["group"],
            Value::String("A".to_string())
        );
    }

    // --- 통계 집계 함수 테스트 ---

    fn sample_scores() -> Value {
        Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("score".to_string(), Value::Integer(90));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("score".to_string(), Value::Integer(80));
                m.insert("dept".to_string(), Value::String("B".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Charlie".to_string()));
                m.insert("score".to_string(), Value::Integer(70));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Diana".to_string()));
                m.insert("score".to_string(), Value::Integer(85));
                m.insert("dept".to_string(), Value::String("B".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Eve".to_string()));
                m.insert("score".to_string(), Value::Integer(95));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                Value::Object(m)
            },
        ])
    }

    #[test]
    fn test_median_odd_count() {
        let data = sample_scores();
        let query = parse_query(".[] | median score").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        // sorted: 70, 80, 85, 90, 95 → median = 85.0
        assert_eq!(result, Value::Float(85.0));
    }

    #[test]
    fn test_median_even_count() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("val".to_string(), Value::Integer(10));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("val".to_string(), Value::Integer(20));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("val".to_string(), Value::Integer(30));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("val".to_string(), Value::Integer(40));
                Value::Object(m)
            },
        ]);
        let query = parse_query(".[] | median val").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        // sorted: 10, 20, 30, 40 → median = (20+30)/2 = 25.0
        assert_eq!(result, Value::Float(25.0));
    }

    #[test]
    fn test_median_empty() {
        let data = Value::Array(vec![]);
        let result = apply_median(data, "val").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_percentile_basic() {
        let data = sample_scores();
        let query = parse_query(".[] | percentile score 0.5").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        // sorted: 70, 80, 85, 90, 95 → p50 = 85.0
        assert_eq!(result, Value::Float(85.0));
    }

    #[test]
    fn test_percentile_p0_and_p1() {
        let data = sample_scores();
        // p=0.0 → minimum
        let result = apply_percentile(data.clone(), "score", 0.0).unwrap();
        assert_eq!(result, Value::Float(70.0));

        // p=1.0 → maximum
        let result = apply_percentile(data, "score", 1.0).unwrap();
        assert_eq!(result, Value::Float(95.0));
    }

    #[test]
    fn test_percentile_interpolation() {
        // Values: 10, 20, 30, 40 → p=0.25 → index=0.75 → lerp(10,20,0.75) = 17.5
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(10));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(20));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(30));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(40));
                Value::Object(m)
            },
        ]);
        let result = apply_percentile(data, "v", 0.25).unwrap();
        assert_eq!(result, Value::Float(17.5));
    }

    #[test]
    fn test_percentile_empty() {
        let data = Value::Array(vec![]);
        let result = apply_percentile(data, "val", 0.5).unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_stddev_basic() {
        // values: 70, 80, 85, 90, 95 → mean=84, variance=((14^2+4^2+1+6^2+11^2)/5)=74, stddev=sqrt(74)
        let data = sample_scores();
        let query = parse_query(".[] | stddev score").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        match result {
            Value::Float(f) => {
                let expected = (74.0_f64).sqrt(); // ~8.602
                assert!(
                    (f - expected).abs() < 0.001,
                    "stddev was {}, expected {}",
                    f,
                    expected
                );
            }
            _ => panic!("expected Float"),
        }
    }

    #[test]
    fn test_stddev_empty() {
        let data = Value::Array(vec![]);
        let result = apply_stddev(data, "score").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_variance_basic() {
        let data = sample_scores();
        let query = parse_query(".[] | variance score").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        match result {
            Value::Float(f) => {
                // mean=84, var = (196+16+1+36+121)/5 = 370/5 = 74
                assert!(
                    (f - 74.0).abs() < 0.001,
                    "variance was {}, expected 74.0",
                    f
                );
            }
            _ => panic!("expected Float"),
        }
    }

    #[test]
    fn test_variance_empty() {
        let data = Value::Array(vec![]);
        let result = apply_variance(data, "score").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_mode_basic() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("color".to_string(), Value::String("red".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("color".to_string(), Value::String("blue".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("color".to_string(), Value::String("red".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("color".to_string(), Value::String("green".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("color".to_string(), Value::String("red".to_string()));
                Value::Object(m)
            },
        ]);
        let query = parse_query(".[] | mode color").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(result, Value::String("red".to_string()));
    }

    #[test]
    fn test_mode_numeric() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(1));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(2));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(2));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("v".to_string(), Value::Integer(3));
                Value::Object(m)
            },
        ]);
        let result = apply_mode(data, "v").unwrap();
        assert_eq!(result, Value::Integer(2));
    }

    #[test]
    fn test_mode_empty() {
        let data = Value::Array(vec![]);
        let result = apply_mode(data, "v").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_group_concat_basic() {
        let data = sample_scores();
        let query = parse_query(".[] | group_concat name \", \"").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(
            result,
            Value::String("Alice, Bob, Charlie, Diana, Eve".to_string())
        );
    }

    #[test]
    fn test_group_concat_default_separator() {
        let data = sample_scores();
        let query = parse_query(".[] | group_concat name").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        assert_eq!(
            result,
            Value::String("Alice, Bob, Charlie, Diana, Eve".to_string())
        );
    }

    #[test]
    fn test_group_concat_with_integers() {
        let data = sample_scores();
        let result = apply_group_concat(data, "score", "-").unwrap();
        assert_eq!(result, Value::String("90-80-70-85-95".to_string()));
    }

    #[test]
    fn test_group_concat_empty() {
        let data = Value::Array(vec![]);
        let result = apply_group_concat(data, "name", ", ").unwrap();
        assert_eq!(result, Value::String("".to_string()));
    }

    // --- group_by with new aggregate functions ---

    #[test]
    fn test_group_by_median() {
        let data = sample_scores();
        let query = parse_query(".[] | group_by dept median(score)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        // dept A: scores 90, 70, 95 → sorted: 70, 90, 95 → median = 90.0
        let dept_a = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("dept") == Some(&Value::String("A".to_string())))
            .unwrap();
        assert_eq!(
            dept_a.as_object().unwrap().get("median_score"),
            Some(&Value::Float(90.0))
        );
        // dept B: scores 80, 85 → median = 82.5
        let dept_b = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("dept") == Some(&Value::String("B".to_string())))
            .unwrap();
        assert_eq!(
            dept_b.as_object().unwrap().get("median_score"),
            Some(&Value::Float(82.5))
        );
    }

    #[test]
    fn test_group_by_percentile() {
        let data = sample_scores();
        let query = parse_query(".[] | group_by dept percentile(score, 0.5)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_group_by_stddev() {
        let data = sample_scores();
        let query = parse_query(".[] | group_by dept stddev(score)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        // Just verify it returns Float values
        for item in arr {
            let obj = item.as_object().unwrap();
            assert!(matches!(obj.get("stddev_score"), Some(Value::Float(_))));
        }
    }

    #[test]
    fn test_group_by_variance() {
        let data = sample_scores();
        let query = parse_query(".[] | group_by dept variance(score)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        for item in arr {
            let obj = item.as_object().unwrap();
            assert!(matches!(obj.get("variance_score"), Some(Value::Float(_))));
        }
    }

    #[test]
    fn test_group_by_mode() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("grade".to_string(), Value::String("A".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("grade".to_string(), Value::String("B".to_string()));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("grade".to_string(), Value::String("A".to_string()));
                Value::Object(m)
            },
        ]);
        let query = parse_query(".[] | group_by dept mode(grade)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(
            arr[0].as_object().unwrap().get("mode_grade"),
            Some(&Value::String("A".to_string()))
        );
    }

    #[test]
    fn test_group_by_group_concat() {
        let data = sample_scores();
        let query = parse_query(".[] | group_by dept group_concat(name, \", \")").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        // dept A: Alice, Charlie, Eve
        let dept_a = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("dept") == Some(&Value::String("A".to_string())))
            .unwrap();
        assert_eq!(
            dept_a.as_object().unwrap().get("group_concat_name"),
            Some(&Value::String("Alice, Charlie, Eve".to_string()))
        );
    }

    #[test]
    fn test_group_by_multiple_new_aggregates() {
        let data = sample_scores();
        let query =
            parse_query(".[] | group_by dept median(score), stddev(score), mode(score)").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        for item in arr {
            let obj = item.as_object().unwrap();
            assert!(obj.contains_key("median_score"));
            assert!(obj.contains_key("stddev_score"));
            assert!(obj.contains_key("mode_score"));
        }
    }

    // --- 윈도우 함수 평가 테스트 ---

    fn sample_window_data() -> Value {
        Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Alice".to_string()));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("score".to_string(), Value::Integer(90));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Bob".to_string()));
                m.insert("dept".to_string(), Value::String("B".to_string()));
                m.insert("score".to_string(), Value::Integer(80));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Charlie".to_string()));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("score".to_string(), Value::Integer(85));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Dave".to_string()));
                m.insert("dept".to_string(), Value::String("B".to_string()));
                m.insert("score".to_string(), Value::Integer(95));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("name".to_string(), Value::String("Eve".to_string()));
                m.insert("dept".to_string(), Value::String("A".to_string()));
                m.insert("score".to_string(), Value::Integer(85));
                Value::Object(m)
            },
        ])
    }

    #[test]
    fn test_window_row_number() {
        let data = sample_window_data();
        let query = parse_query(".[] | select name, row_number() over (order by score desc) as rn")
            .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        // Score order desc: Dave(95), Alice(90), Charlie(85), Eve(85), Bob(80)
        // row_number for each original position:
        let get_rn = |name: &str| -> i64 {
            arr.iter()
                .find(|o| {
                    o.as_object().unwrap().get("name") == Some(&Value::String(name.to_string()))
                })
                .unwrap()
                .as_object()
                .unwrap()
                .get("rn")
                .unwrap()
                .as_i64()
                .unwrap()
        };
        assert_eq!(get_rn("Dave"), 1);
        assert_eq!(get_rn("Alice"), 2);
        // Charlie and Eve both have 85, row_number assigns sequentially
        let charlie_rn = get_rn("Charlie");
        let eve_rn = get_rn("Eve");
        assert!(charlie_rn == 3 || charlie_rn == 4);
        assert!(eve_rn == 3 || eve_rn == 4);
        assert_ne!(charlie_rn, eve_rn);
        assert_eq!(get_rn("Bob"), 5);
    }

    #[test]
    fn test_window_rank() {
        let data = sample_window_data();
        let query =
            parse_query(".[] | select name, rank() over (order by score desc) as r").unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        let get_r = |name: &str| -> i64 {
            arr.iter()
                .find(|o| {
                    o.as_object().unwrap().get("name") == Some(&Value::String(name.to_string()))
                })
                .unwrap()
                .as_object()
                .unwrap()
                .get("r")
                .unwrap()
                .as_i64()
                .unwrap()
        };
        assert_eq!(get_r("Dave"), 1);
        assert_eq!(get_r("Alice"), 2);
        // Charlie and Eve both 85 → same rank 3
        assert_eq!(get_r("Charlie"), 3);
        assert_eq!(get_r("Eve"), 3);
        // Bob gets rank 5 (not 4) because rank skips
        assert_eq!(get_r("Bob"), 5);
    }

    #[test]
    fn test_window_dense_rank() {
        let data = sample_window_data();
        let query = parse_query(".[] | select name, dense_rank() over (order by score desc) as dr")
            .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        let get_dr = |name: &str| -> i64 {
            arr.iter()
                .find(|o| {
                    o.as_object().unwrap().get("name") == Some(&Value::String(name.to_string()))
                })
                .unwrap()
                .as_object()
                .unwrap()
                .get("dr")
                .unwrap()
                .as_i64()
                .unwrap()
        };
        assert_eq!(get_dr("Dave"), 1);
        assert_eq!(get_dr("Alice"), 2);
        assert_eq!(get_dr("Charlie"), 3);
        assert_eq!(get_dr("Eve"), 3);
        // Bob gets dense_rank 4 (not 5)
        assert_eq!(get_dr("Bob"), 4);
    }

    #[test]
    fn test_window_partition_by_row_number() {
        let data = sample_window_data();
        let query = parse_query(
            ".[] | select name, dept, row_number() over (partition by dept order by score desc) as dept_rn",
        )
        .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        let get_rn = |name: &str| -> i64 {
            arr.iter()
                .find(|o| {
                    o.as_object().unwrap().get("name") == Some(&Value::String(name.to_string()))
                })
                .unwrap()
                .as_object()
                .unwrap()
                .get("dept_rn")
                .unwrap()
                .as_i64()
                .unwrap()
        };
        // Dept A: Alice(90)=1, Charlie(85)=2, Eve(85)=3
        assert_eq!(get_rn("Alice"), 1);
        let charlie_rn = get_rn("Charlie");
        let eve_rn = get_rn("Eve");
        assert!(charlie_rn == 2 || charlie_rn == 3);
        assert!(eve_rn == 2 || eve_rn == 3);
        // Dept B: Dave(95)=1, Bob(80)=2
        assert_eq!(get_rn("Dave"), 1);
        assert_eq!(get_rn("Bob"), 2);
    }

    #[test]
    fn test_window_lag() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(1));
                m.insert("value".to_string(), Value::Integer(100));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(2));
                m.insert("value".to_string(), Value::Integer(200));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(3));
                m.insert("value".to_string(), Value::Integer(300));
                Value::Object(m)
            },
        ]);
        let query =
            parse_query(".[] | select date, value, lag(value) over (order by date) as prev_value")
                .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // date=1 → prev_value=Null
        assert_eq!(
            arr[0].as_object().unwrap().get("prev_value"),
            Some(&Value::Null)
        );
        // date=2 → prev_value=100
        assert_eq!(
            arr[1].as_object().unwrap().get("prev_value"),
            Some(&Value::Integer(100))
        );
        // date=3 → prev_value=200
        assert_eq!(
            arr[2].as_object().unwrap().get("prev_value"),
            Some(&Value::Integer(200))
        );
    }

    #[test]
    fn test_window_lead() {
        let data = Value::Array(vec![
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(1));
                m.insert("value".to_string(), Value::Integer(100));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(2));
                m.insert("value".to_string(), Value::Integer(200));
                Value::Object(m)
            },
            {
                let mut m = IndexMap::new();
                m.insert("date".to_string(), Value::Integer(3));
                m.insert("value".to_string(), Value::Integer(300));
                Value::Object(m)
            },
        ]);
        let query =
            parse_query(".[] | select date, value, lead(value) over (order by date) as next_value")
                .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // date=1 → next_value=200
        assert_eq!(
            arr[0].as_object().unwrap().get("next_value"),
            Some(&Value::Integer(200))
        );
        // date=2 → next_value=300
        assert_eq!(
            arr[1].as_object().unwrap().get("next_value"),
            Some(&Value::Integer(300))
        );
        // date=3 → next_value=Null
        assert_eq!(
            arr[2].as_object().unwrap().get("next_value"),
            Some(&Value::Null)
        );
    }

    #[test]
    fn test_window_first_last_value() {
        let data = sample_window_data();
        let query = parse_query(
            ".[] | select name, first_value(name) over (partition by dept order by score desc) as top, last_value(name) over (partition by dept order by score desc) as bottom",
        )
        .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Dept A: sorted by score desc → Alice(90), then Charlie/Eve(85)
        let alice = arr
            .iter()
            .find(|o| {
                o.as_object().unwrap().get("name") == Some(&Value::String("Alice".to_string()))
            })
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(alice.get("top"), Some(&Value::String("Alice".to_string())));
        // Dept B: Dave(95), Bob(80) → first=Dave, last=Bob
        let bob = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("name") == Some(&Value::String("Bob".to_string())))
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(bob.get("top"), Some(&Value::String("Dave".to_string())));
        assert_eq!(bob.get("bottom"), Some(&Value::String("Bob".to_string())));
    }

    #[test]
    fn test_window_aggregate_sum() {
        let data = sample_window_data();
        let query = parse_query(
            ".[] | select name, dept, sum(score) over (partition by dept) as dept_total",
        )
        .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Dept A: 90+85+85=260, Dept B: 80+95=175
        let alice = arr
            .iter()
            .find(|o| {
                o.as_object().unwrap().get("name") == Some(&Value::String("Alice".to_string()))
            })
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(alice.get("dept_total"), Some(&Value::Integer(260)));

        let bob = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("name") == Some(&Value::String("Bob".to_string())))
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(bob.get("dept_total"), Some(&Value::Integer(175)));
    }

    #[test]
    fn test_window_aggregate_avg() {
        let data = sample_window_data();
        let query =
            parse_query(".[] | select name, avg(score) over (partition by dept) as dept_avg")
                .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Dept A avg: (90+85+85)/3 ≈ 86.666...
        let alice = arr
            .iter()
            .find(|o| {
                o.as_object().unwrap().get("name") == Some(&Value::String("Alice".to_string()))
            })
            .unwrap()
            .as_object()
            .unwrap();
        if let Value::Float(avg) = alice.get("dept_avg").unwrap() {
            assert!((avg - 260.0 / 3.0).abs() < 0.001);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn test_window_aggregate_count() {
        let data = sample_window_data();
        let query =
            parse_query(".[] | select name, count(score) over (partition by dept) as dept_count")
                .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Dept A: 3, Dept B: 2
        let alice = arr
            .iter()
            .find(|o| {
                o.as_object().unwrap().get("name") == Some(&Value::String("Alice".to_string()))
            })
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(alice.get("dept_count"), Some(&Value::Integer(3)));
    }

    #[test]
    fn test_window_aggregate_min_max() {
        let data = sample_window_data();
        let query = parse_query(
            ".[] | select name, min(score) over (partition by dept) as dept_min, max(score) over (partition by dept) as dept_max",
        )
        .unwrap();
        let path_result = crate::query::evaluator::evaluate_path(&data, &query.path).unwrap();
        let result = apply_operations(path_result, &query.operations).unwrap();
        let arr = result.as_array().unwrap();
        // Dept A: min=85, max=90
        let alice = arr
            .iter()
            .find(|o| {
                o.as_object().unwrap().get("name") == Some(&Value::String("Alice".to_string()))
            })
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(alice.get("dept_min"), Some(&Value::Integer(85)));
        assert_eq!(alice.get("dept_max"), Some(&Value::Integer(90)));
        // Dept B: min=80, max=95
        let bob = arr
            .iter()
            .find(|o| o.as_object().unwrap().get("name") == Some(&Value::String("Bob".to_string())))
            .unwrap()
            .as_object()
            .unwrap();
        assert_eq!(bob.get("dept_min"), Some(&Value::Integer(80)));
        assert_eq!(bob.get("dept_max"), Some(&Value::Integer(95)));
    }
}
