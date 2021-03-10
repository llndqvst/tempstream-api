use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub bind_address: String,
    pub redis_url: String,
    pub srs_web: String,
    pub srs_rtmp: String,
}

pub fn get_app_config() -> AppConfig {
    match envy::from_env::<AppConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error)
    }
}
