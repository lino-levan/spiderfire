use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use ion::{Context, FromValue, Object, Result, Value};
use ion::conversions::{ConversionBehavior, FromValue};

#[derive(FromValue)]
struct Complex<'cx> {
	#[ion(inherit)]
	raw: Object<'cx>,
	truth: bool,
	#[ion(convert = ConversionBehavior::EnforceRange, strict)]
	mode: u32,
	#[ion(default = String::from("string"))]
	text: String,
	#[ion(strict, default = true)]
	other: bool,
	#[ion(default, convert = ConversionBehavior::Clamp)]
	optional: Option<i32>,
	#[ion(default = Arc::new(AtomicU64::new(16)), parser = |v| parse_as_atomic_arc(cx, v))]
	parsed: Arc<AtomicU64>,
}

unsafe fn parse_as_atomic_arc<'cx: 'v, 'v>(cx: &'cx Context, value: Value<'v>) -> Result<Arc<AtomicU64>> {
	u64::from_value(cx, &value, true, ConversionBehavior::Default).map(|num| Arc::new(AtomicU64::new(num)))
}