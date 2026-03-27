use crate::error::DkitError;
use crate::query::parser::{ArithmeticOp, Expr, LiteralValue};
use crate::value::Value;

/// 표현식을 주어진 레코드(행)에 대해 평가하여 Value를 반환
pub fn evaluate_expr(row: &Value, expr: &Expr) -> Result<Value, DkitError> {
    match expr {
        Expr::Field(name) => match row {
            Value::Object(map) => Ok(map.get(name).cloned().unwrap_or(Value::Null)),
            _ => Err(DkitError::QueryError(format!(
                "cannot access field '{}' on non-object value",
                name
            ))),
        },
        Expr::Literal(lit) => Ok(literal_to_value(lit)),
        Expr::FuncCall { name, args } => {
            let evaluated: Result<Vec<Value>, DkitError> =
                args.iter().map(|a| evaluate_expr(row, a)).collect();
            call_function(name, evaluated?)
        }
        Expr::BinaryOp { op, left, right } => {
            let lv = evaluate_expr(row, left)?;
            let rv = evaluate_expr(row, right)?;
            evaluate_binary_op(op, &lv, &rv)
        }
    }
}

/// 이항 산술 연산 평가
fn evaluate_binary_op(op: &ArithmeticOp, left: &Value, right: &Value) -> Result<Value, DkitError> {
    // String concatenation with +
    if matches!(op, ArithmeticOp::Add) {
        if let (Value::String(a), Value::String(b)) = (left, right) {
            return Ok(Value::String(format!("{}{}", a, b)));
        }
        // String + non-string or non-string + String → string concat
        if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
            let a = value_to_display_string(left);
            let b = value_to_display_string(right);
            return Ok(Value::String(format!("{}{}", a, b)));
        }
    }

    // Null propagation
    if matches!(left, Value::Null) || matches!(right, Value::Null) {
        return Ok(Value::Null);
    }

    // Numeric operations
    let (lf, li) = to_numeric(left)?;
    let (rf, ri) = to_numeric(right)?;

    // Both integers → integer result (except division)
    if let (Some(a), Some(b)) = (li, ri) {
        return match op {
            ArithmeticOp::Add => Ok(Value::Integer(a.wrapping_add(b))),
            ArithmeticOp::Sub => Ok(Value::Integer(a.wrapping_sub(b))),
            ArithmeticOp::Mul => Ok(Value::Integer(a.wrapping_mul(b))),
            ArithmeticOp::Div => {
                if b == 0 {
                    return Err(DkitError::QueryError("division by zero".to_string()));
                }
                // Use float division if not evenly divisible
                if a % b == 0 {
                    Ok(Value::Integer(a / b))
                } else {
                    Ok(Value::Float(lf / rf))
                }
            }
        };
    }

    // At least one float → float result
    match op {
        ArithmeticOp::Add => Ok(Value::Float(lf + rf)),
        ArithmeticOp::Sub => Ok(Value::Float(lf - rf)),
        ArithmeticOp::Mul => Ok(Value::Float(lf * rf)),
        ArithmeticOp::Div => {
            if rf == 0.0 {
                return Err(DkitError::QueryError("division by zero".to_string()));
            }
            Ok(Value::Float(lf / rf))
        }
    }
}

/// Value를 (f64, Option<i64>) 로 변환. i64로 표현 가능하면 Some(i64).
fn to_numeric(v: &Value) -> Result<(f64, Option<i64>), DkitError> {
    match v {
        Value::Integer(n) => Ok((*n as f64, Some(*n))),
        Value::Float(f) => Ok((*f, None)),
        Value::Bool(b) => {
            let n = if *b { 1 } else { 0 };
            Ok((n as f64, Some(n)))
        }
        _ => Err(DkitError::QueryError(format!(
            "cannot perform arithmetic on {} value",
            value_type_name(v)
        ))),
    }
}

/// Value를 문자열로 변환 (string concat용)
fn value_to_display_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Integer(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => format!("{}", v),
    }
}

/// 표현식에서 출력 키(필드명)를 자동으로 결정
pub fn expr_default_key(expr: &Expr) -> String {
    match expr {
        Expr::Field(f) => f.clone(),
        Expr::Literal(_) => "value".to_string(),
        Expr::FuncCall { name, args } => {
            if let Some(first) = args.first() {
                format!("{}_{}", name, expr_default_key(first))
            } else {
                name.clone()
            }
        }
        Expr::BinaryOp { left, .. } => expr_default_key(left),
    }
}

fn literal_to_value(lit: &LiteralValue) -> Value {
    match lit {
        LiteralValue::String(s) => Value::String(s.clone()),
        LiteralValue::Integer(n) => Value::Integer(*n),
        LiteralValue::Float(f) => Value::Float(*f),
        LiteralValue::Bool(b) => Value::Bool(*b),
        LiteralValue::Null => Value::Null,
        LiteralValue::List(items) => Value::Array(items.iter().map(literal_to_value).collect()),
    }
}

/// 내장 함수 호출
fn call_function(name: &str, args: Vec<Value>) -> Result<Value, DkitError> {
    match name {
        // --- 문자열 함수 ---
        "upper" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.to_uppercase()))
        }
        "lower" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.to_lowercase()))
        }
        "trim" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.trim().to_string()))
        }
        "ltrim" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.trim_start().to_string()))
        }
        "rtrim" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.trim_end().to_string()))
        }
        "length" => match args.as_slice() {
            [Value::String(s)] => Ok(Value::Integer(s.chars().count() as i64)),
            [Value::Array(a)] => Ok(Value::Integer(a.len() as i64)),
            [Value::Null] => Ok(Value::Integer(0)),
            [v] => Err(DkitError::QueryError(format!(
                "length() requires a string or array argument, got {}",
                value_type_name(v)
            ))),
            _ => Err(DkitError::QueryError(format!(
                "length() takes 1 argument, got {}",
                args.len()
            ))),
        },
        "substr" => {
            if args.len() < 2 || args.len() > 3 {
                return Err(DkitError::QueryError(format!(
                    "substr() takes 2 or 3 arguments, got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Null),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "substr() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let start = require_integer_arg("substr", &args[1], "start")? as usize;
            let chars: Vec<char> = s.chars().collect();
            let start = start.min(chars.len());
            if args.len() == 3 {
                let len = require_integer_arg("substr", &args[2], "length")? as usize;
                let end = (start + len).min(chars.len());
                Ok(Value::String(chars[start..end].iter().collect()))
            } else {
                Ok(Value::String(chars[start..].iter().collect()))
            }
        }
        "concat" => {
            if args.is_empty() {
                return Ok(Value::String(String::new()));
            }
            let mut result = String::new();
            for arg in &args {
                match arg {
                    Value::String(s) => result.push_str(s),
                    Value::Null => {}
                    v => result.push_str(&v.to_string()),
                }
            }
            Ok(Value::String(result))
        }
        "replace" => {
            if args.len() != 3 {
                return Err(DkitError::QueryError(format!(
                    "replace() takes 3 arguments (string, from, to), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Null),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "replace() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let from = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "replace() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let to = match &args[2] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "replace() third argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            Ok(Value::String(s.replace(&*from, &to)))
        }
        "split" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "split() takes 2 arguments (string, separator), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Array(vec![])),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "split() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let sep = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "split() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            Ok(Value::Array(
                s.split(&*sep)
                    .map(|p| Value::String(p.to_string()))
                    .collect(),
            ))
        }

        "index_of" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "index_of() takes 2 arguments (string, substr), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Integer(-1)),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "index_of() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let substr = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "index_of() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            match s.find(&*substr) {
                Some(pos) => {
                    let char_pos = s[..pos].chars().count() as i64;
                    Ok(Value::Integer(char_pos))
                }
                None => Ok(Value::Integer(-1)),
            }
        }
        "rindex_of" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "rindex_of() takes 2 arguments (string, substr), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Integer(-1)),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "rindex_of() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let substr = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "rindex_of() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            match s.rfind(&*substr) {
                Some(pos) => {
                    let char_pos = s[..pos].chars().count() as i64;
                    Ok(Value::Integer(char_pos))
                }
                None => Ok(Value::Integer(-1)),
            }
        }
        "starts_with" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "starts_with() takes 2 arguments (string, prefix), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Bool(false)),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "starts_with() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let prefix = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "starts_with() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            Ok(Value::Bool(s.starts_with(&*prefix)))
        }
        "ends_with" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "ends_with() takes 2 arguments (string, suffix), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Bool(false)),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "ends_with() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let suffix = match &args[1] {
                Value::String(s) => s.clone(),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "ends_with() second argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            Ok(Value::Bool(s.ends_with(&*suffix)))
        }
        "reverse" => {
            let s = require_one_string(name, &args)?;
            Ok(Value::String(s.chars().rev().collect()))
        }
        "repeat" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "repeat() takes 2 arguments (string, count), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Null),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "repeat() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let n = require_integer_arg("repeat", &args[1], "count")?;
            if n < 0 {
                return Err(DkitError::QueryError(
                    "repeat() count must be non-negative".to_string(),
                ));
            }
            Ok(Value::String(s.repeat(n as usize)))
        }
        "pad_left" => {
            if args.len() != 3 {
                return Err(DkitError::QueryError(format!(
                    "pad_left() takes 3 arguments (string, width, char), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Null),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "pad_left() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let width = require_integer_arg("pad_left", &args[1], "width")? as usize;
            let pad_char = match &args[2] {
                Value::String(s) => {
                    let mut chars = s.chars();
                    match (chars.next(), chars.next()) {
                        (Some(c), None) => c,
                        _ => {
                            return Err(DkitError::QueryError(
                                "pad_left() third argument must be a single character".to_string(),
                            ))
                        }
                    }
                }
                v => {
                    return Err(DkitError::QueryError(format!(
                        "pad_left() third argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let char_count = s.chars().count();
            if char_count >= width {
                Ok(Value::String(s))
            } else {
                let padding: String = std::iter::repeat(pad_char).take(width - char_count).collect();
                Ok(Value::String(format!("{}{}", padding, s)))
            }
        }
        "pad_right" => {
            if args.len() != 3 {
                return Err(DkitError::QueryError(format!(
                    "pad_right() takes 3 arguments (string, width, char), got {}",
                    args.len()
                )));
            }
            let s = match &args[0] {
                Value::String(s) => s.clone(),
                Value::Null => return Ok(Value::Null),
                v => {
                    return Err(DkitError::QueryError(format!(
                        "pad_right() first argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let width = require_integer_arg("pad_right", &args[1], "width")? as usize;
            let pad_char = match &args[2] {
                Value::String(s) => {
                    let mut chars = s.chars();
                    match (chars.next(), chars.next()) {
                        (Some(c), None) => c,
                        _ => {
                            return Err(DkitError::QueryError(
                                "pad_right() third argument must be a single character".to_string(),
                            ))
                        }
                    }
                }
                v => {
                    return Err(DkitError::QueryError(format!(
                        "pad_right() third argument must be a string, got {}",
                        value_type_name(v)
                    )))
                }
            };
            let char_count = s.chars().count();
            if char_count >= width {
                Ok(Value::String(s))
            } else {
                let padding: String = std::iter::repeat(pad_char).take(width - char_count).collect();
                Ok(Value::String(format!("{}{}", s, padding)))
            }
        }

        // --- 수학 함수 ---
        "round" => {
            let n = require_numeric_arg(name, &args)?;
            if args.len() == 2 {
                let decimals = require_integer_arg("round", &args[1], "decimals")?;
                let factor = 10_f64.powi(decimals as i32);
                Ok(Value::Float((n * factor).round() / factor))
            } else if args.len() == 1 {
                Ok(Value::Integer(n.round() as i64))
            } else {
                Err(DkitError::QueryError(format!(
                    "round() takes 1 or 2 arguments, got {}",
                    args.len()
                )))
            }
        }
        "ceil" => {
            let n = require_numeric_arg(name, &args)?;
            Ok(Value::Integer(n.ceil() as i64))
        }
        "floor" => {
            let n = require_numeric_arg(name, &args)?;
            Ok(Value::Integer(n.floor() as i64))
        }
        "abs" => match args.as_slice() {
            [Value::Integer(n)] => Ok(Value::Integer(n.abs())),
            [Value::Float(f)] => Ok(Value::Float(f.abs())),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(DkitError::QueryError(format!(
                "abs() requires a numeric argument, got {}",
                value_type_name(v)
            ))),
            _ => Err(DkitError::QueryError(format!(
                "abs() takes 1 argument, got {}",
                args.len()
            ))),
        },
        "sqrt" => {
            let n = require_numeric_arg(name, &args)?;
            if n < 0.0 {
                return Err(DkitError::QueryError(
                    "sqrt() requires a non-negative argument".to_string(),
                ));
            }
            Ok(Value::Float(n.sqrt()))
        }
        "pow" => {
            if args.len() != 2 {
                return Err(DkitError::QueryError(format!(
                    "pow() takes 2 arguments, got {}",
                    args.len()
                )));
            }
            let base = require_numeric_arg("pow base", &args[..1])?;
            let exp = require_numeric_arg("pow exp", &args[1..])?;
            Ok(Value::Float(base.powf(exp)))
        }

        // --- 날짜 함수 ---
        "now" => {
            if !args.is_empty() {
                return Err(DkitError::QueryError(
                    "now() takes no arguments".to_string(),
                ));
            }
            Ok(Value::String(current_datetime_utc()))
        }
        "date" => {
            let s = require_one_string(name, &args)?;
            // 날짜 파싱 검증 (ISO 8601 형식 기대)
            let normalized = normalize_date_str(&s)?;
            Ok(Value::String(normalized))
        }
        "year" => {
            let s = require_one_string(name, &args)?;
            let y = extract_year(&s)?;
            Ok(Value::Integer(y))
        }
        "month" => {
            let s = require_one_string(name, &args)?;
            let m = extract_month(&s)?;
            Ok(Value::Integer(m))
        }
        "day" => {
            let s = require_one_string(name, &args)?;
            let d = extract_day(&s)?;
            Ok(Value::Integer(d))
        }

        // --- 타입 변환 ---
        "to_int" | "int" => match args.as_slice() {
            [Value::Integer(n)] => Ok(Value::Integer(*n)),
            [Value::Float(f)] => Ok(Value::Integer(*f as i64)),
            [Value::String(s)] => s.trim().parse::<i64>().map(Value::Integer).map_err(|_| {
                DkitError::QueryError(format!("to_int(): cannot parse '{}' as integer", s))
            }),
            [Value::Bool(b)] => Ok(Value::Integer(if *b { 1 } else { 0 })),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(DkitError::QueryError(format!(
                "to_int() cannot convert {}",
                value_type_name(v)
            ))),
            _ => Err(DkitError::QueryError(format!(
                "to_int() takes 1 argument, got {}",
                args.len()
            ))),
        },
        "to_float" | "float" => match args.as_slice() {
            [Value::Float(f)] => Ok(Value::Float(*f)),
            [Value::Integer(n)] => Ok(Value::Float(*n as f64)),
            [Value::String(s)] => s.trim().parse::<f64>().map(Value::Float).map_err(|_| {
                DkitError::QueryError(format!("to_float(): cannot parse '{}' as float", s))
            }),
            [Value::Bool(b)] => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
            [Value::Null] => Ok(Value::Null),
            [v] => Err(DkitError::QueryError(format!(
                "to_float() cannot convert {}",
                value_type_name(v)
            ))),
            _ => Err(DkitError::QueryError(format!(
                "to_float() takes 1 argument, got {}",
                args.len()
            ))),
        },
        "to_string" | "str" => match args.as_slice() {
            [Value::String(s)] => Ok(Value::String(s.clone())),
            [Value::Integer(n)] => Ok(Value::String(n.to_string())),
            [Value::Float(f)] => Ok(Value::String(f.to_string())),
            [Value::Bool(b)] => Ok(Value::String(b.to_string())),
            [Value::Null] => Ok(Value::String("null".to_string())),
            [v] => Ok(Value::String(v.to_string())),
            _ => Err(DkitError::QueryError(format!(
                "to_string() takes 1 argument, got {}",
                args.len()
            ))),
        },
        "to_bool" | "bool" => match args.as_slice() {
            [Value::Bool(b)] => Ok(Value::Bool(*b)),
            [Value::Integer(n)] => Ok(Value::Bool(*n != 0)),
            [Value::Float(f)] => Ok(Value::Bool(*f != 0.0)),
            [Value::String(s)] => match s.trim().to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Ok(Value::Bool(true)),
                "false" | "no" | "0" | "off" | "" => Ok(Value::Bool(false)),
                _ => Err(DkitError::QueryError(format!(
                    "to_bool(): cannot parse '{}' as boolean",
                    s
                ))),
            },
            [Value::Null] => Ok(Value::Bool(false)),
            [v] => Err(DkitError::QueryError(format!(
                "to_bool() cannot convert {}",
                value_type_name(v)
            ))),
            _ => Err(DkitError::QueryError(format!(
                "to_bool() takes 1 argument, got {}",
                args.len()
            ))),
        },

        // --- 기타 유틸 ---
        "coalesce" => {
            for arg in &args {
                if !matches!(arg, Value::Null) {
                    return Ok(arg.clone());
                }
            }
            Ok(Value::Null)
        }
        "if_null" => match args.as_slice() {
            [Value::Null, default] => Ok(default.clone()),
            [v, _] => Ok(v.clone()),
            _ => Err(DkitError::QueryError(format!(
                "if_null() takes 2 arguments, got {}",
                args.len()
            ))),
        },

        _ => Err(DkitError::QueryError(format!(
            "unknown function '{}'",
            name
        ))),
    }
}

// --- 헬퍼 함수 ---

fn require_one_string(func: &str, args: &[Value]) -> Result<String, DkitError> {
    match args {
        [Value::String(s)] => Ok(s.clone()),
        [Value::Null] => Err(DkitError::QueryError(format!(
            "{}() argument is null",
            func
        ))),
        [v] => Err(DkitError::QueryError(format!(
            "{}() requires a string argument, got {}",
            func,
            value_type_name(v)
        ))),
        _ => Err(DkitError::QueryError(format!(
            "{}() takes 1 argument, got {}",
            func,
            args.len()
        ))),
    }
}

fn require_numeric_arg(func: &str, args: &[Value]) -> Result<f64, DkitError> {
    match args.first() {
        Some(Value::Float(f)) => Ok(*f),
        Some(Value::Integer(n)) => Ok(*n as f64),
        Some(Value::Null) => Err(DkitError::QueryError(format!(
            "{}() argument is null",
            func
        ))),
        Some(v) => Err(DkitError::QueryError(format!(
            "{}() requires a numeric argument, got {}",
            func,
            value_type_name(v)
        ))),
        None => Err(DkitError::QueryError(format!(
            "{}() takes at least 1 argument",
            func
        ))),
    }
}

fn require_integer_arg(func: &str, val: &Value, param: &str) -> Result<i64, DkitError> {
    match val {
        Value::Integer(n) => Ok(*n),
        Value::Float(f) => Ok(*f as i64),
        v => Err(DkitError::QueryError(format!(
            "{}() '{}' argument must be an integer, got {}",
            func,
            param,
            value_type_name(v)
        ))),
    }
}

fn value_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

// --- 날짜 헬퍼 ---

/// ISO 8601 날짜 문자열에서 연도 추출 (예: "2024-03-15" → 2024)
fn extract_year(s: &str) -> Result<i64, DkitError> {
    let date_part = s.split('T').next().unwrap_or(s);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.is_empty() {
        return Err(DkitError::QueryError(format!(
            "year(): cannot parse date from '{}'",
            s
        )));
    }
    parts[0]
        .parse::<i64>()
        .map_err(|_| DkitError::QueryError(format!("year(): cannot parse year from '{}'", s)))
}

/// ISO 8601 날짜 문자열에서 월 추출 (1-12)
fn extract_month(s: &str) -> Result<i64, DkitError> {
    let date_part = s.split('T').next().unwrap_or(s);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() < 2 {
        return Err(DkitError::QueryError(format!(
            "month(): cannot parse month from '{}'",
            s
        )));
    }
    parts[1]
        .parse::<i64>()
        .map_err(|_| DkitError::QueryError(format!("month(): cannot parse month from '{}'", s)))
}

/// ISO 8601 날짜 문자열에서 일 추출 (1-31)
fn extract_day(s: &str) -> Result<i64, DkitError> {
    let date_part = s.split('T').next().unwrap_or(s);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() < 3 {
        return Err(DkitError::QueryError(format!(
            "day(): cannot parse day from '{}'",
            s
        )));
    }
    // Day part may have trailing time info
    let day_str = parts[2].split(&['T', ' '][..]).next().unwrap_or(parts[2]);
    day_str
        .parse::<i64>()
        .map_err(|_| DkitError::QueryError(format!("day(): cannot parse day from '{}'", s)))
}

/// 날짜 문자열 정규화 (yyyy-MM-dd 형식 확인)
fn normalize_date_str(s: &str) -> Result<String, DkitError> {
    let date_part = s.split('T').next().unwrap_or(s).trim();
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return Err(DkitError::QueryError(format!(
            "date(): expected yyyy-MM-dd format, got '{}'",
            s
        )));
    }
    let year = parts[0]
        .parse::<i32>()
        .map_err(|_| DkitError::QueryError(format!("date(): invalid year in '{}'", s)))?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| DkitError::QueryError(format!("date(): invalid month in '{}'", s)))?;
    let day = parts[2]
        .split(&[' ', 'T'][..])
        .next()
        .unwrap_or(parts[2])
        .parse::<u32>()
        .map_err(|_| DkitError::QueryError(format!("date(): invalid day in '{}'", s)))?;
    if !(1..=12).contains(&month) {
        return Err(DkitError::QueryError(format!(
            "date(): month {} out of range in '{}'",
            month, s
        )));
    }
    if !(1..=31).contains(&day) {
        return Err(DkitError::QueryError(format!(
            "date(): day {} out of range in '{}'",
            day, s
        )));
    }
    Ok(format!("{:04}-{:02}-{:02}", year, month, day))
}

/// 현재 UTC 시각을 ISO 8601 문자열로 반환
fn current_datetime_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // 간단한 UTC 변환 (초 → 날짜/시간)
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let (year, month, day) = days_to_ymd(days_since_epoch);
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hour, minute, second
    )
}

/// Unix epoch 일수를 (year, month, day)로 변환
fn days_to_ymd(days: u64) -> (i64, u64, u64) {
    // Gregorian calendar algorithm
    let z = days as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u64, d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::parser::Expr;
    use indexmap::IndexMap;

    fn obj(fields: &[(&str, Value)]) -> Value {
        let mut map = IndexMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.clone());
        }
        Value::Object(map)
    }

    fn eval(row: &Value, expr: &Expr) -> Result<Value, DkitError> {
        evaluate_expr(row, expr)
    }

    fn func(name: &str, args: Vec<Expr>) -> Expr {
        Expr::FuncCall {
            name: name.to_string(),
            args,
        }
    }

    fn field(name: &str) -> Expr {
        Expr::Field(name.to_string())
    }

    fn lit_str(s: &str) -> Expr {
        Expr::Literal(LiteralValue::String(s.to_string()))
    }

    fn lit_int(n: i64) -> Expr {
        Expr::Literal(LiteralValue::Integer(n))
    }

    // --- 문자열 함수 ---

    #[test]
    fn test_upper() {
        let row = obj(&[("name", Value::String("hello".to_string()))]);
        let result = eval(&row, &func("upper", vec![field("name")])).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_lower() {
        let row = obj(&[("name", Value::String("WORLD".to_string()))]);
        let result = eval(&row, &func("lower", vec![field("name")])).unwrap();
        assert_eq!(result, Value::String("world".to_string()));
    }

    #[test]
    fn test_trim() {
        let row = obj(&[("name", Value::String("  hello  ".to_string()))]);
        let result = eval(&row, &func("trim", vec![field("name")])).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_length_string() {
        let row = obj(&[("name", Value::String("hello".to_string()))]);
        let result = eval(&row, &func("length", vec![field("name")])).unwrap();
        assert_eq!(result, Value::Integer(5));
    }

    #[test]
    fn test_substr() {
        let row = obj(&[("name", Value::String("hello world".to_string()))]);
        let result = eval(
            &row,
            &func("substr", vec![field("name"), lit_int(0), lit_int(5)]),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_concat() {
        let row = obj(&[
            ("first", Value::String("hello".to_string())),
            ("last", Value::String(" world".to_string())),
        ]);
        let result = eval(&row, &func("concat", vec![field("first"), field("last")])).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_replace() {
        let row = obj(&[("name", Value::String("hello world".to_string()))]);
        let result = eval(
            &row,
            &func(
                "replace",
                vec![field("name"), lit_str("world"), lit_str("dkit")],
            ),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello dkit".to_string()));
    }

    #[test]
    fn test_index_of() {
        let row = obj(&[("email", Value::String("user@example.com".to_string()))]);
        let result = eval(&row, &func("index_of", vec![field("email"), lit_str("@")])).unwrap();
        assert_eq!(result, Value::Integer(4));
    }

    #[test]
    fn test_index_of_not_found() {
        let row = obj(&[("s", Value::String("hello".to_string()))]);
        let result = eval(&row, &func("index_of", vec![field("s"), lit_str("xyz")])).unwrap();
        assert_eq!(result, Value::Integer(-1));
    }

    #[test]
    fn test_rindex_of() {
        let row = obj(&[("s", Value::String("abcabc".to_string()))]);
        let result = eval(&row, &func("rindex_of", vec![field("s"), lit_str("abc")])).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_rindex_of_not_found() {
        let row = obj(&[("s", Value::String("hello".to_string()))]);
        let result = eval(&row, &func("rindex_of", vec![field("s"), lit_str("xyz")])).unwrap();
        assert_eq!(result, Value::Integer(-1));
    }

    #[test]
    fn test_starts_with() {
        let row = obj(&[("name", Value::String("Dr. Smith".to_string()))]);
        let result =
            eval(&row, &func("starts_with", vec![field("name"), lit_str("Dr.")])).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_starts_with_false() {
        let row = obj(&[("name", Value::String("Mr. Smith".to_string()))]);
        let result =
            eval(&row, &func("starts_with", vec![field("name"), lit_str("Dr.")])).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_ends_with() {
        let row = obj(&[("file", Value::String("data.json".to_string()))]);
        let result =
            eval(&row, &func("ends_with", vec![field("file"), lit_str(".json")])).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_ends_with_false() {
        let row = obj(&[("file", Value::String("data.csv".to_string()))]);
        let result =
            eval(&row, &func("ends_with", vec![field("file"), lit_str(".json")])).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_reverse() {
        let row = obj(&[("s", Value::String("hello".to_string()))]);
        let result = eval(&row, &func("reverse", vec![field("s")])).unwrap();
        assert_eq!(result, Value::String("olleh".to_string()));
    }

    #[test]
    fn test_repeat() {
        let row = obj(&[("s", Value::String("ab".to_string()))]);
        let result = eval(&row, &func("repeat", vec![field("s"), lit_int(3)])).unwrap();
        assert_eq!(result, Value::String("ababab".to_string()));
    }

    #[test]
    fn test_repeat_zero() {
        let row = obj(&[("s", Value::String("ab".to_string()))]);
        let result = eval(&row, &func("repeat", vec![field("s"), lit_int(0)])).unwrap();
        assert_eq!(result, Value::String(String::new()));
    }

    #[test]
    fn test_pad_left() {
        let row = obj(&[("id", Value::String("42".to_string()))]);
        let result = eval(
            &row,
            &func("pad_left", vec![field("id"), lit_int(5), lit_str("0")]),
        )
        .unwrap();
        assert_eq!(result, Value::String("00042".to_string()));
    }

    #[test]
    fn test_pad_left_no_padding_needed() {
        let row = obj(&[("id", Value::String("12345".to_string()))]);
        let result = eval(
            &row,
            &func("pad_left", vec![field("id"), lit_int(3), lit_str("0")]),
        )
        .unwrap();
        assert_eq!(result, Value::String("12345".to_string()));
    }

    #[test]
    fn test_pad_right() {
        let row = obj(&[("s", Value::String("hi".to_string()))]);
        let result = eval(
            &row,
            &func("pad_right", vec![field("s"), lit_int(5), lit_str(".")]),
        )
        .unwrap();
        assert_eq!(result, Value::String("hi...".to_string()));
    }

    #[test]
    fn test_pad_right_no_padding_needed() {
        let row = obj(&[("s", Value::String("hello".to_string()))]);
        let result = eval(
            &row,
            &func("pad_right", vec![field("s"), lit_int(3), lit_str(".")]),
        )
        .unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    // --- 수학 함수 ---

    #[test]
    fn test_round_no_decimals() {
        let row = obj(&[("price", Value::Float(3.7))]);
        let result = eval(&row, &func("round", vec![field("price")])).unwrap();
        assert_eq!(result, Value::Integer(4));
    }

    #[test]
    fn test_round_with_decimals() {
        let row = obj(&[("price", Value::Float(3.14159))]);
        let result = eval(&row, &func("round", vec![field("price"), lit_int(2)])).unwrap();
        match result {
            Value::Float(v) => assert!((v - 3.14).abs() < 1e-10, "expected ~3.14, got {v}"),
            other => panic!("expected Float, got {other:?}"),
        }
    }

    #[test]
    fn test_ceil() {
        let row = obj(&[("price", Value::Float(3.2))]);
        let result = eval(&row, &func("ceil", vec![field("price")])).unwrap();
        assert_eq!(result, Value::Integer(4));
    }

    #[test]
    fn test_floor() {
        let row = obj(&[("price", Value::Float(3.9))]);
        let result = eval(&row, &func("floor", vec![field("price")])).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_abs_negative() {
        let row = obj(&[("score", Value::Integer(-5))]);
        let result = eval(&row, &func("abs", vec![field("score")])).unwrap();
        assert_eq!(result, Value::Integer(5));
    }

    // --- 날짜 함수 ---

    #[test]
    fn test_year() {
        let row = obj(&[("date", Value::String("2024-03-15".to_string()))]);
        let result = eval(&row, &func("year", vec![field("date")])).unwrap();
        assert_eq!(result, Value::Integer(2024));
    }

    #[test]
    fn test_month() {
        let row = obj(&[("date", Value::String("2024-03-15".to_string()))]);
        let result = eval(&row, &func("month", vec![field("date")])).unwrap();
        assert_eq!(result, Value::Integer(3));
    }

    #[test]
    fn test_day() {
        let row = obj(&[("date", Value::String("2024-03-15".to_string()))]);
        let result = eval(&row, &func("day", vec![field("date")])).unwrap();
        assert_eq!(result, Value::Integer(15));
    }

    #[test]
    fn test_date_normalize() {
        let row = obj(&[("d", Value::String("2024-3-5".to_string()))]);
        let result = eval(&row, &func("date", vec![field("d")])).unwrap();
        assert_eq!(result, Value::String("2024-03-05".to_string()));
    }

    #[test]
    fn test_now_returns_string() {
        let row = obj(&[]);
        let result = eval(&row, &func("now", vec![])).unwrap();
        assert!(matches!(result, Value::String(_)));
        if let Value::String(s) = result {
            assert!(s.contains('T'), "now() should return ISO 8601 format");
        }
    }

    // --- 타입 변환 ---

    #[test]
    fn test_to_int_from_string() {
        let row = obj(&[("n", Value::String("42".to_string()))]);
        let result = eval(&row, &func("to_int", vec![field("n")])).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_to_float_from_int() {
        let row = obj(&[("n", Value::Integer(5))]);
        let result = eval(&row, &func("to_float", vec![field("n")])).unwrap();
        assert_eq!(result, Value::Float(5.0));
    }

    #[test]
    fn test_to_string_from_int() {
        let row = obj(&[("n", Value::Integer(42))]);
        let result = eval(&row, &func("to_string", vec![field("n")])).unwrap();
        assert_eq!(result, Value::String("42".to_string()));
    }

    #[test]
    fn test_to_bool_from_string() {
        let row = obj(&[("flag", Value::String("true".to_string()))]);
        let result = eval(&row, &func("to_bool", vec![field("flag")])).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    // --- 중첩 함수 호출 ---

    #[test]
    fn test_nested_upper_trim() {
        let row = obj(&[("name", Value::String("  hello  ".to_string()))]);
        let result = eval(
            &row,
            &func("upper", vec![func("trim", vec![field("name")])]),
        )
        .unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }

    // --- expr_default_key ---

    #[test]
    fn test_default_key_field() {
        assert_eq!(expr_default_key(&Expr::Field("name".to_string())), "name");
    }

    #[test]
    fn test_default_key_func() {
        let expr = Expr::FuncCall {
            name: "upper".to_string(),
            args: vec![Expr::Field("name".to_string())],
        };
        assert_eq!(expr_default_key(&expr), "upper_name");
    }

    #[test]
    fn test_default_key_nested_func() {
        let expr = Expr::FuncCall {
            name: "upper".to_string(),
            args: vec![Expr::FuncCall {
                name: "trim".to_string(),
                args: vec![Expr::Field("name".to_string())],
            }],
        };
        assert_eq!(expr_default_key(&expr), "upper_trim_name");
    }

    // --- BinaryOp evaluate tests ---

    #[test]
    fn test_binary_add_integers() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("a".to_string(), Value::Integer(10)),
            ("b".to_string(), Value::Integer(20)),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Add,
            left: Box::new(Expr::Field("a".to_string())),
            right: Box::new(Expr::Field("b".to_string())),
        };
        assert_eq!(evaluate_expr(&row, &expr).unwrap(), Value::Integer(30));
    }

    #[test]
    fn test_binary_mul_float() {
        let row = Value::Object(indexmap::IndexMap::from([(
            "price".to_string(),
            Value::Float(100.0),
        )]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Mul,
            left: Box::new(Expr::Field("price".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Float(0.1))),
        };
        let result = evaluate_expr(&row, &expr).unwrap();
        if let Value::Float(f) = result {
            assert!((f - 10.0).abs() < 0.001);
        } else {
            panic!("expected float");
        }
    }

    #[test]
    fn test_binary_string_concat() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("first".to_string(), Value::String("Hello".to_string())),
            ("last".to_string(), Value::String("World".to_string())),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Add,
            left: Box::new(Expr::Field("first".to_string())),
            right: Box::new(Expr::BinaryOp {
                op: ArithmeticOp::Add,
                left: Box::new(Expr::Literal(LiteralValue::String(" ".to_string()))),
                right: Box::new(Expr::Field("last".to_string())),
            }),
        };
        assert_eq!(
            evaluate_expr(&row, &expr).unwrap(),
            Value::String("Hello World".to_string())
        );
    }

    #[test]
    fn test_binary_div_by_zero() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("a".to_string(), Value::Integer(10)),
            ("b".to_string(), Value::Integer(0)),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Div,
            left: Box::new(Expr::Field("a".to_string())),
            right: Box::new(Expr::Field("b".to_string())),
        };
        assert!(evaluate_expr(&row, &expr).is_err());
    }

    #[test]
    fn test_binary_null_propagation() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("a".to_string(), Value::Integer(10)),
            ("b".to_string(), Value::Null),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Add,
            left: Box::new(Expr::Field("a".to_string())),
            right: Box::new(Expr::Field("b".to_string())),
        };
        assert_eq!(evaluate_expr(&row, &expr).unwrap(), Value::Null);
    }

    #[test]
    fn test_binary_int_div_non_exact() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("a".to_string(), Value::Integer(10)),
            ("b".to_string(), Value::Integer(3)),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Div,
            left: Box::new(Expr::Field("a".to_string())),
            right: Box::new(Expr::Field("b".to_string())),
        };
        // 10 / 3 is not exact, should return float
        let result = evaluate_expr(&row, &expr).unwrap();
        assert!(matches!(result, Value::Float(_)));
    }

    #[test]
    fn test_binary_mixed_string_number_concat() {
        let row = Value::Object(indexmap::IndexMap::from([
            ("name".to_string(), Value::String("Item".to_string())),
            ("id".to_string(), Value::Integer(42)),
        ]));
        let expr = Expr::BinaryOp {
            op: ArithmeticOp::Add,
            left: Box::new(Expr::Field("name".to_string())),
            right: Box::new(Expr::Field("id".to_string())),
        };
        assert_eq!(
            evaluate_expr(&row, &expr).unwrap(),
            Value::String("Item42".to_string())
        );
    }
}
