#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use clap::{App, AppSettings};
use futures::TryFutureExt;
use log::{debug, error, info, warn};
use authentication::Authenticator;
use error::InternalError;
use crate::settings::{AuthenticationSettings, Settings};

mod schema;
mod models;
mod error;
mod authentication;
mod actions;
mod user;
mod keycloak;
mod api;
mod settings;
mod app;

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    env_logger::init();

    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(arg!(-c --config <CONFIG> "Define the config file"))
        .subcommand(App::new("serve").about("start the liberation service"))
        .subcommand(App::new("test").about("run whatever was programed"))
        .get_matches();

    let settings = Settings::with_file(matches.value_of_t_or_exit("config"))?;

    match matches.subcommand() {
        Some(("serve", _submatches)) => {
            use actix_web::{App, HttpServer, middleware};
            use app::AppState;

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
            let (authenticator, worker) = match settings.authentication {
                AuthenticationSettings::Static { public_key } =>
                    (Authenticator::with_static_key(public_key), None),
                AuthenticationSettings::Keycloak { url, realm, renew_interval_s } => {
                    Authenticator::with_rotating_keys(url, realm, renew_interval_s).await
                }
            };

            debug!("Creating app state.");
            let app_state = Data::new(AppState::new(
                pool,
                authenticator,
            ));

            info!("Starting Server.");
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .wrap(middleware::NormalizePath::trim())
                    .configure(api::v1)
            }).bind(settings.bind).map_err(InternalError::IOError)?
                .run().map_err(InternalError::IOError).await?;

            if let Some(handle) = worker {
                info!("Stopping update worker");
                handle.abort();
                debug!("Update worker stopped with {:?}", handle.await);
            }

            info!("Bye!");
            Ok(())
        }
        Some(("test", _submatches)) => {
            Ok(())
        }
        _ => unreachable!()
    }
}
