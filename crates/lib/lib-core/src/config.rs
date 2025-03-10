use lib_util::env::{get_env, get_env_b64u_as_u8s, get_env_parse};
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
    // -- Crypt
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: String,
    // -- Web
    pub WEB_FOLDER: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_util::env::Result<CoreConfig> {
        Ok(CoreConfig {
            DB_URL: get_env("SERVICE_DB_URL")?,
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}
