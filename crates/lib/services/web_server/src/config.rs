use lib_util::env::get_env;
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    // Since this is an educational project wich will only be run locally we will treat this as the front-end domain
    pub DEV_SERVER_URL: String,
}

impl WebConfig {
    fn load_from_env() -> lib_util::env::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            DEV_SERVER_URL: get_env("SERVICE_DEV_SERVER_URL")?,
        })
    }
}
