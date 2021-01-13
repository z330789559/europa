//! Util
use crate::Value;
use wasmtime::Val;

pub fn from_val(v: Val) -> Option<Value> {
	match v {
		Val::F32(v) => Some(Value::F32(v)),
		Val::I32(v) => Some(Value::I32(v)),
		Val::F64(v) => Some(Value::F64(v)),
		Val::I64(v) => Some(Value::I64(v)),
		_ => None,
	}
}

pub fn to_val(v: Value) -> Val {
	match v {
		Value::F32(v) => Val::F32(v),
		Value::F64(v) => Val::F64(v),
		Value::I32(v) => Val::I32(v),
		Value::I64(v) => Val::I64(v),
	}
}
