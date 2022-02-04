#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;

use std::time::Duration;
use actix_web::web::Data;
use clap::{AppSettings, Parser};
use futures::TryFutureExt;
use log::{debug, error, info};
use tokio::time;
use liberation::auth::Authenticator;
use liberation::error::InternalError;

mod api;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(short, long)]
    pub database: String,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        public_key: String,
        #[clap(short, long, default_value_t = String::from("127.0.0.1:8080"))]
        bind: String,
    },
    Test,
}

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { bind, public_key } => {
            use actix_web::{App, HttpServer, middleware};
            use actix_web::rt::spawn;
            use liberation::AppState;

            let pool = {
                use diesel::{MysqlConnection, r2d2};
                use diesel::r2d2::ConnectionManager;

                let manager = ConnectionManager::<MysqlConnection>::new(&cli.database);
                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create db pool.")
            };
            info!("Started connection pool.");

            let authenticator = {
                use jsonwebtoken::DecodingKey;
                let der_key = base64::decode(public_key).expect("No base64 key");
                let static_key = DecodingKey::from_rsa_der(der_key.as_slice());
                Authenticator::OauthStatic { public_key: static_key }
            };
            info!("Created key provider.");

            let app_state = Data::new(AppState::new(
                pool,
                authenticator,
            ));
            info!("Created app state.");

            info!("Starting Keycloak Worker.");
            let keycloak_worker = {
                let app_state = app_state.clone();
                spawn(async move {
                    let mut interval = time::interval(Duration::from_secs(320));
                    loop {
                        interval.tick().await;
                        if let Err(e) = app_state.update().await {
                            error!("Could not update state {:?}", e)
                        }
                    };
                })
            };


            info!("Starting Server...");
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .configure(api::v1)
            }).bind(&bind)
                .map_err(InternalError::IOError)?
                .run()
                .map_err(InternalError::IOError)
                .await?;

            info!("Stopping update worker");
            keycloak_worker.abort();
            debug!("Update worker stopped with {:?}", keycloak_worker.await);

            info!("Bye!");
            Ok(())
        }
        Commands::Test => {
            // use oauth2::basic::BasicClient;
            // use oauth2::reqwest::async_http_client;
            // use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl};

            // // TODO: load from settings
            // let keycloak_url = "https://sso.rpg-librarium.de/";
            // let realm = "Liberation";
            // let client_id = "liberation-backend";
            // let client_secret = "f7948706-ed9a-4107-9da6-f0076d444cbe";

            // let client = BasicClient::new(
            //     ClientId::new(client_id.to_string()),
            //     Some(ClientSecret::new(client_secret.to_string())),
            //     AuthUrl::new(format!("{}/realms/{}/protocol/openid-connect/auth", keycloak_url, realm)).expect("Invalid url"),
            //     Some(TokenUrl::new(format!("{}/realms/{}/protocol/openid-connect/auth", keycloak_url, realm)).expect("Invalid url"))
            // );

            // let token_result = client
            //     .exchange_client_credentials()
            //     .request_async(async_http_client)
            //     .await?;
            Ok(())
        }
    }
}
