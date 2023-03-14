use std::net::TcpListener;
use env_logger::Env;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool};
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use zero2prod1::configuration::get_configuration;
//use std::error::Error;
use zero2prod1::run;
use zero2prod1::telemetry::{get_subscriber, init_subscriber};
#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    let subscriber= get_subscriber("zero2prod1".into(),"info".into(),std::io::stdout);

    zero2prod1::telemetry::init_subscriber(subscriber);

    LogTracer::init().expect("Failed to initialize logger");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings! let address = format!("127.0.0.1:{}", configuration.application_port);
   let connection_pool=PgPool::connect(
       &configuration.database.connection_string().expose_secret()
   )
       .await
      .expect("Failed to connect to database.");

    let address = format!("{}:{}",configuration.application.host,configuration.application.port);
    let listener = TcpListener::bind(address)?;
run(listener,connection_pool)?.await?;
    Ok(())
}
 


