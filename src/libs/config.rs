use clap::{App, Arg};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub log_level: String,
    pub db_uri: String,
    pub suffix_filter_to: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let get = |name: &'static str| env::var(name).unwrap_or_default();
        let log_level = get("LOG_LEVEL");
        let db_uri = get("DATABASE_URL");
        let suffix_filter_to = get("SUFFIX_FILTER_EMAIL_TO");
        let port = get("PORT");
        Self { log_level, db_uri, suffix_filter_to, port }
    }

    pub fn get_address(&self) -> String {
        format!("127.0.0.1:{}", self.port.as_str())
    }

    pub fn merge_args(&mut self) {
        let matches = App::new("Email Inbound Webhook")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Receive and query inbound emails via sendgrid webhook")
            .arg(
                Arg::with_name("database")
                    .short("d")
                    .long("database")
                    .value_name("DATABASE_URL")
                    .help("Sets sqlite database uri")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("suffix_filter")
                    .short("s")
                    .long("suffix_filter")
                    .value_name("SUFFIX_FILTER_EMAIL_TO")
                    .help("Sets suffix filter for where email sent to")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("log_level")
                    .short("l")
                    .long("log_level")
                    .value_name("LOG_LEVEL")
                    .help("Sets log level: DEBUG, INFO, WARN, ... Default: INFO")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .help("Sets http server listening port. Default: 3200")
                    .takes_value(true),
            )
            .get_matches();

        if let Some(o) = matches.value_of("database") {
            self.db_uri = o.to_owned();
        }

        if let Some(o) = matches.value_of("port") {
            self.port = o.to_owned();
        }

        if let Some(o) = matches.value_of("log_level") {
            self.log_level = o.to_owned();
        }

        if let Some(o) = matches.value_of("suffix_filter") {
            self.suffix_filter_to = o.to_owned();
        }

        if self.log_level.is_empty() {
            self.log_level = String::from("INFO");
        }

        if self.port.is_empty() {
            self.port = String::from("3200");
        }

        if self.db_uri.is_empty() {
            println!("Miss env var DATABASE_URL, try --help for more information.");
            std::process::exit(1);
        }

        if self.suffix_filter_to.is_empty() {
            println!("Miss env var SUFFIX_FILTER_EMAIL_TO, try --help for more information.");
            std::process::exit(1);
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut config = Config::new();
        config.merge_args();
        config
    };
}
