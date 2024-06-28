use clap::Parser;
use dotenv;
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short, long, default_value = "info")]
    pub log_level: String,

    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    #[clap(short, long, default_value = "postgres")]
    pub db_user: String,

    #[clap(short, long, default_value = "localhost")]
    pub db_host: String,

    #[clap(short, long, default_value = "5432")]
    pub db_port: u16,

    #[clap(short, long, default_value = "postgres")]
    pub db_name: String,

    #[clap(long, default_value = "need_to_set")]
    pub db_password: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        dotenv::dotenv().ok();
        let config = Config::parse();

        if let Err(_) = env::var("APILAYER_API_KEY") {
            panic!("APILAYER_API_KEY not set");
        }

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user =
            env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());

        let db_password = env::var("POSTGRES_PASSWORD").unwrap();

        let db_host =
            env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());

        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or(config.db_port.to_string());

        let db_name =
            env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name,
            db_password,
        })
    }
}
