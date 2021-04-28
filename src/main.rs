use env_logger::{Builder, Target};

use libs::ctx;

mod routes;
mod email;
mod libs;

#[macro_use]
extern crate log;
extern crate dotenv;
#[macro_use]
extern crate clap;

use libs::config::CONFIG;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut builder = Builder::from_default_env();
    let log_level = match CONFIG.log_level.as_str() {
        "DEBUG" => log::LevelFilter::Debug,
        "INFO" => log::LevelFilter::Info,
        "WARN" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Error,
    };
    builder.target(Target::Stdout);
    builder.filter_level(log_level);
    builder.init();

    let state = ctx::Context::new().await;
    let mut app = tide::with_state(state);

    app.at("/").get(|_| async { Ok("Ok /") });
    app.at("sendgrid").post(routes::sendgrid_webhook::parse_inbound_payload);
    app.at("emails").get(routes::emails::search_latest_emails);

    app.listen(CONFIG.get_address()).await?;

    Ok(())
}
