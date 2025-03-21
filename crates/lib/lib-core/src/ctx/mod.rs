// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

// Constructors.
impl Ctx {
    pub fn new(user_id: i64) -> Result<Self> {
        if user_id != 0 {
            Ok(Self { user_id })
        } else {
            Err(Error::CtxCannotNewRootCtx)
        }
    }

    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
