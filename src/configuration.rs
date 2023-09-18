use config::{Config, File};
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub application_port: u16,
    pub database_name: String,
    pub application_addr: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let builder = Config::builder().add_source(File::with_name("configuration"));

    let config = builder.build()?;
    config.try_deserialize()
}

impl Settings {
    pub fn db_connection_string(&self) -> String {
        format!("sqlite://{}.db", self.database_name)
    }

    pub async fn db_check(&self) {
        let db_url = self.db_connection_string();

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("->> {:<12} - CREATING DATABASE {}", "STARTUP", &db_url);
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("->> {:<12} - DATABASE CREATED", "STARTUP"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("->> {:<12} - DATABASE EXISTS {}", "STARTUP", &db_url);
        }
    }
}
