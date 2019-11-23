#[macro_use] extern crate log;

mod api;
mod auth;
mod business;
mod database;
mod error;
mod serde_formats;
mod settings;

use actix::{Actor, System};
use actix_web::{web, App, HttpServer};
use api::{get_static, get_v1};
use auth::KeycloakCache;
use settings::Settings;
use actix_web::middleware::Logger;

fn main() {
    env_logger::init();

    info!("retrieving settings ...");
    let settings = Settings::new().unwrap();
    info!("initializing DB ...");
    let db = database::Database::from_settings(&settings.database).unwrap();

    info!("initializing keycloak ...");
    let kc: KeycloakCache = KeycloakCache::new();
    let kc_actor = auth::Keycloak::from_settings(&settings.keycloak, kc.clone());

    let state = api::AppState {
        db,
        kc: kc.clone(),
    };

    let sys = System::new("server");
    kc_actor.start();

    let serve_static_files = settings.serve_static_files;
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default())
            .register_data(web::Data::new(state.clone()))
            .service(get_v1());
        if serve_static_files {
            app = app.service(get_static());
        }
        app
    })
    .bind(format!("0.0.0.0:{}", settings.port))
    .unwrap()
    .start();

    info!("liberation ready");
    sys.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
