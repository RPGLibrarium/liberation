#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;

use crate::settings::{AuthenticationSettings, Settings};
use actix_web::web::Data;
use authentication::Authentication;
use clap::Command;
use error::InternalError;
use futures::TryFutureExt;
use log::{debug, info};

mod actions;
mod api;
mod app;
mod authentication;
mod error;
mod keycloak;
mod models;
mod schema;
mod settings;

#[actix_web::main]
async fn main() -> Result<(), InternalError> {
    env_logger::init();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg(arg!(-c --config <CONFIG> "set the config file"))
        .subcommand(Command::new("serve").about("start the liberation service"))
        .subcommand(Command::new("test").about("run whatever was programed"))
        .get_matches();

    let settings = Settings::with_file(matches.value_of_t_or_exit("config"))?;

    match matches.subcommand() {
        Some(("serve", _submatches)) => {
            use actix_web::{middleware, App, HttpServer};
            use app::AppState;

            info!("Creating database pool.");
            let pool = {
                use diesel::r2d2::ConnectionManager;
                use diesel::{r2d2, MysqlConnection};

                let manager = ConnectionManager::<MysqlConnection>::new(&settings.database);
                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create db pool.")
            };

            debug!("Creating authenticator.");
            let (authenticator, worker) = match settings.authentication {
                AuthenticationSettings::Static { public_key } => {
                    (Authentication::with_static_key(public_key), None)
                }
                AuthenticationSettings::Keycloak {
                    url,
                    realm,
                    renew_interval_s,
                } => Authentication::with_rotating_keys(url, realm, renew_interval_s).await,
            };

            debug!("Creating app state.");
            let app_state = Data::new(AppState::new(pool, authenticator));

            info!("Starting Server.");
            HttpServer::new(move || {
                App::new()
                    .app_data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .wrap(middleware::NormalizePath::trim())
                    .configure(api::v1)
            })
            .bind(settings.bind)
            .map_err(InternalError::IOError)?
            .run()
            .map_err(InternalError::IOError)
            .await?;

            if let Some(handle) = worker {
                info!("Stopping update worker");
                handle.abort();
                debug!("Update worker stopped with {:?}", handle.await);
            }

            info!("Bye!");
            Ok(())
        }
        Some(("test", _submatches)) => Ok(()),
        _ => unreachable!(),
    }
}
