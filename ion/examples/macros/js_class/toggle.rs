use ion::{Context, Function, js_class, Object, Result, Value};
use ion::class::Reflector;
use ion::conversions::FromValue;

#[js_class]
#[derive(Debug, Default)]
pub struct Toggle {
	reflector: Reflector,
	toggle: bool,
	toggled: i32,
}

#[js_class]
impl Toggle {
	pub const DEFAULT_TOGGLED: i32 = 0;

	#[ion(constructor)]
	pub fn constructor() -> Toggle {
		Toggle::default()
	}

	#[ion(skip)]
	pub fn reset(&mut self) -> bool {
		self.toggled = 0;
		true
	}

	#[ion(name = "callback", alias = ["if_toggled", "if_switched"])]
	pub fn if_toggled(&self, cx: &Context, function: Function) -> Result<String> {
		let value = function.call(cx, &Object::null(cx), &[Value::i32(cx, self.toggled)]).unwrap();
		String::from_value(cx, &value, false, ())
	}

	#[ion(get, alias = ["switch"])]
	pub fn get_toggle(&self) -> i32 {
		self.toggled * self.toggle as i32
	}

	#[ion(set)]
	pub fn set_toggle(&mut self, toggle: bool) -> bool {
		let toggled = self.toggled;
		if !self.reset() {
			return false;
		}
		self.toggled = toggled + toggle as i32;
		self.toggle = toggle;
		toggle
	}
}

pub fn ensure_callable() {
	let mut toggle = Toggle::constructor();
	toggle.get_toggle();
	toggle.set_toggle(true);
	toggle.reset();
}
