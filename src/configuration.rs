use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSetting,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub db_name: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSetting {
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine tu current directory.");
    let configuration_directory = base_path.join("configuration");

    // Read "default" configuration file
    // Default to `local` if unspecified
    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let builder = Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(File::from(configuration_directory.join("base")))
        .add_source(File::from(configuration_directory.join(env.as_str())).required(true));
    let config = builder.build()?;

    config.try_deserialize()
}

pub enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

impl Settings {
    pub fn postgres_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.db_name
        )
    }
}
