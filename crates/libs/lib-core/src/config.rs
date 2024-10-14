use lib_utils::envs::get_env;
use std::sync::OnceLock;
use dotenv::dotenv;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        dotenv().ok();
        CoreConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            // -- Db
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
