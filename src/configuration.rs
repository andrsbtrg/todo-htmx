use config::{Config, File};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub application_port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub application_addr: String,
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
            self.username, self.password, self.host, self.database_name
        )
    }

    pub async fn db_check(&self) {
        let _db_url = self.postgres_connection_string();
        println!("implement a measure to test if database is up");

        // if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        //     println!("->> {:<12} - CREATING DATABASE {}", "STARTUP", &db_url);
        //     match Sqlite::create_database(&db_url).await {
        //         Ok(_) => println!("->> {:<12} - DATABASE CREATED", "STARTUP"),
        //         Err(error) => panic!("error: {}", error),
        //     }
        // } else {
        //     println!("->> {:<12} - DATABASE EXISTS {}", "STARTUP", &db_url);
        // }
    }
}
