use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn messenger_config() -> &'static MessengerConfig {
    static INSTANCE: OnceLock<MessengerConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        MessengerConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct MessengerConfig {
    pub VERIFY_TOKEN: String,
}

impl MessengerConfig {
    fn load_from_env() -> lib_utils::envs::Result<MessengerConfig> {
        Ok(MessengerConfig {
            VERIFY_TOKEN: get_env("FACEBOOK_WEBHOOK_VERIFY_TOKEN")?,
        })
    }
}
