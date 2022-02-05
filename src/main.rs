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
            .arg(arg!(-k --keycloak [URL] "set keycloak url, ex. 'https://sso.rpg-librarium.de/").requires("realm"))
            .arg(arg!(-r --realm [REALM] "set keycloak realm, ex. 'liberation'").requires("keycloak"))
            .arg(arg!(-K --"static-key" [KEY] "set the key manually"))
            .group(
                ArgGroup::new("authenticator")
                    .required(true)
                    //.multiple(false)
                    .args(&["keycloak", "static-key"])
            )
            .arg(arg!(-b --bind [ADDR] "bind on this address and port").default_value("127.0.0.1:8080"))
            .arg(arg!(-c --"client-id" [ID] "authenticated access to keycloak").requires_all(&["keycloak", "client-secret"]))
            .arg(arg!(-s --"client-secret" [ID] "authenticated access to keycloak").requires_all(&["keycloak", "client-id"]))
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
                let realm = submatches.value_of_t_or_exit("realm");
                Authenticator::with_rotating_keys(keycloak_url.to_string(), realm).await
            } else {
                let static_key = submatches.value_of_t_or_exit::<String>("static-key");
                Authenticator::with_static_key(static_key)
            };

            let live_users = if let Some(client_id) = submatches.value_of("client-id") {
                use liberation::user::LiveUsers;
                debug!("Creating live users.");
                let keycloak_url = submatches.value_of_t_or_exit("keycloak");
                let realm = submatches.value_of_t_or_exit("realm");
                let client_secret = submatches.value_of_t_or_exit("client-secret");
                LiveUsers::new(keycloak_url, realm, client_id.to_string(), client_secret).await?
            } else {
                debug!("No live users configured");
                todo!()
            };


            debug!("Creating app state.");
            let app_state = Data::new(AppState::new(
                pool,
                authenticator,
                live_users
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
            Ok(())
        }
        _ => unreachable!()
    }
}
