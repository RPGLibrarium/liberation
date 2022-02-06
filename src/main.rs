#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;

use std::time::Duration;
use actix_web::web::Data;
use clap::{App, AppSettings};
use futures::TryFutureExt;
use log::{debug, error, info, warn};
use tokio::time;
use liberation::auth::Authenticator;
use liberation::error::InternalError;
use crate::settings::Settings;

mod api;
mod settings;

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    env_logger::init();

    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(arg!(-c --config <CONFIG> "define the config file"))
        .subcommand(App::new("serve").about("start the liberation service"))
        .subcommand(App::new("test").about("run whatever was programed"))
        .get_matches();

    let settings = Settings::with_file(matches.value_of_t_or_exit("config"))?;

    match matches.subcommand() {
        Some(("serve", _submatches)) => {
            use actix_web::{App, HttpServer, middleware};
            use actix_web::rt::spawn;
            use liberation::AppState;

            info!("Creating database pool.");
            let pool = {
                use diesel::{MysqlConnection, r2d2};
                use diesel::r2d2::ConnectionManager;

                let manager = ConnectionManager::<MysqlConnection>::new(&settings.database);
                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create db pool.")
            };

            debug!("Creating authenticator.");
            let authenticator = if let Some(jwt_public_key) = settings.jwt_public_key {
                Authenticator::with_static_key(jwt_public_key)
            } else if let Some(keycloak) = &settings.keycloak {
                Authenticator::with_rotating_keys(&keycloak.url, &keycloak.realm).await
            } else {
                todo!("keycloak or jwt_public_key are mandatory.");
            };

            debug!("Creating live user provider.");
            let live_users = if let Some(keycloak) = settings.keycloak {
                if let (Some(client_id), Some(client_secret)) = (&keycloak.client_id, &keycloak.client_secret) {
                    use liberation::user::LiveUsers;
                    debug!("Creating live users.");
                    LiveUsers::new(&keycloak.url, &keycloak.realm, client_id.clone(), client_secret.clone()).await?
                } else {
                    warn!("Missing keycloak.client_id, or keycloak.client_secret. Live user update is deactivated.");
                    todo!("not implemented")
                }
            } else {
                warn!("Missing keycloak. Live user update is deactivated.");
                todo!("not implemented")
            };

            debug!("Creating app state.");
            let app_state = Data::new(AppState::new(
                pool,
                authenticator,
                live_users,
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


            info!("Starting Server...");
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .configure(api::v1)
            }).bind(settings.bind)
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
