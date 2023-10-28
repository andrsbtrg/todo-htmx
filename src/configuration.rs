use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub addr: String,
    pub port: u16,
    pub database: DatabaseConfig,
}

#[derive(serde::Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let builder = Config::builder().add_source(File::with_name("configuration"));

    let config = builder.build()?;
    config.try_deserialize()
}

impl Settings {
    pub fn postgres_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.db_name
        )
    }
}
