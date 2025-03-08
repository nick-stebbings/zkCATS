// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}