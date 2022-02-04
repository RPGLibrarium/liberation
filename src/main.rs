#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;

use std::time::Duration;
use actix_web::web::Data;
use clap::{App, AppSettings, ArgGroup};
use futures::TryFutureExt;
use log::{debug, error, info};
use tokio::time;
use liberation::auth::Authenticator;
use liberation::error::InternalError;

mod api;

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    env_logger::init();

    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .arg(arg!(-d --database <DB> "set database, ex. 'mysql://USER:PASSWORD@localhost:3306/DATABASE'"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("serve")
            .about("start the liberation service")
            .arg(arg!(-k --keycloak [URL] "use keycloak, ex. 'https://sso.rpg-librarium.de/auth/realms/Liberation/"))
            .arg(arg!(-K --"static-key" [KEY] "set the key manually"))
            .group(
                ArgGroup::new("authenticator")
                    .required(true)
                    //.multiple(false)
                    .args(&["keycloak", "static-key"])
            )
            .arg(arg!(-b --bind [ADDR] "bind on this address and port").default_value("127.0.0.1:8080"))
        )
        .subcommand(App::new("test")
            .about("run whatever was programed")
        )
        .get_matches();

    info!("Creating database pool.");
    let pool = {
        use diesel::{MysqlConnection, r2d2};
        use diesel::r2d2::ConnectionManager;

        let database_url = matches.value_of_t_or_exit::<String>("database");
        let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create db pool.")
    };

    match matches.subcommand() {
        Some(("serve", submatches)) => {
            use actix_web::{App, HttpServer, middleware};
            use actix_web::rt::spawn;
            use liberation::AppState;

            debug!("Creating authenticator.");
            let authenticator = if let Some(keycloak_url) = submatches.value_of("keycloak") {
                Authenticator::with_rotating_keys(keycloak_url.to_string()).await
            } else {
                let static_key = submatches.value_of_t_or_exit::<String>("static-key");
                Authenticator::with_static_key(static_key)
            };

            debug!("Creating app state.");
            let app_state = Data::new(AppState::new(
                pool,
                authenticator,
            ));

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

            let bind_address = submatches.value_of_t_or_exit::<String>("bind");

            info!("Starting Server...");
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .configure(api::v1)
            }).bind(bind_address)
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
        Some(("test", _submatches)) => {
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
        _ => unreachable!()
    }
}
