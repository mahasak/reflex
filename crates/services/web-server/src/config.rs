use lib_utils::envs::get_env;
use std::sync::OnceLock;
use dotenv::dotenv;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        dotenv().ok();
        WebConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub VERIFY_TOKEN: String,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            VERIFY_TOKEN: get_env("FACEBOOK_WEBHOOK_VERIFY_TOKEN")?,
        })
    }
}
