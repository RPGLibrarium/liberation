#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;

use clap::{AppSettings, Parser};
use crate::api::get_rpg_systems;

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
        kc_pub_key: String,
        #[clap(short, long, default_value_t = String::from("127.0.0.1:8080"))]
        bind: String,
    },
    Test,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { bind, kc_pub_key } => {
            use actix_web::{App, HttpServer, middleware, web};
            use liberation::AppState;

            let pool = {
                use diesel::{MysqlConnection, r2d2};
                use diesel::r2d2::ConnectionManager;

                let manager = ConnectionManager::<MysqlConnection>::new(&cli.database);
                r2d2::Pool::builder()
                    .build(manager)
                    .expect("Failed to create db pool.")
            };

            let key = {
                use jsonwebtoken::DecodingKey;
                let der_key = base64::decode(kc_pub_key).expect("No base64 key");
                DecodingKey::from_rsa_der(der_key.as_slice())
            };

            let app_state = AppState {
                database: pool,
                kc_public_key: key,
            };

            // Start HTTP server
            HttpServer::new(move || {
                App::new()
                    // set up DB pool to be used with web::Data<Pool> extractor
                    .data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .service(
                        web::scope("/rpgsystems")
                            .service(
                                web::resource("")
                                    .route(web::get().to(get_rpg_systems))
                            )
                    )
            }).bind(&bind)?
                .run()
                .await
        }
        Commands::Test => {
            // Function used for println debugging
            use diesel::prelude::*;
            use liberation::claims::Authentication;
            use liberation::get_rpg_systems;
            use liberation::models::Title;

            let connection = MysqlConnection::establish(&cli.database)
                .expect(&format!("Error connecting to {}", &cli.database));

            let claims = Authentication::authorized(false, vec![], 1);
            let system = get_rpg_systems(claims, &connection, 2)
                .expect("loading rpg system failed");

            let titles = Title::belonging_to(&system)
                .load::<Title>(&connection)
                .expect("Error loading titles");

            println!("Displaying {} titles", titles.len());
            for title in titles {
                println!("{:?}", title);
            }
            Ok(())
        }
    }
}

mod api {
    use actix_web::{Responder, web};
    use actix_web::web::Json;
    use diesel::RunQueryDsl;
    use liberation::AppState;
    use liberation::claims::Authentication;
    use liberation::error::{InternalError};
    use liberation::error::UserFacingError::Internal;
    use liberation::models::RpgSystem;
    use liberation::schema::rpg_systems::dsl::rpg_systems;

    // Don't ask to many questions about the arguments. With typing magic actix allows us to get the
    // state or arguments from the request. We can use up to function arguments to get data auto
    // magically out of the request.
    // https://github.com/actix/actix-web/blob/2a12b41456f40b28c1efe0ec6947e8f50ba22006/src/handler.rs
    // https://actix.rs/docs/extractors/
    pub async fn get_rpg_systems(app: web::Data<AppState>, authentication: Authentication) -> impl Responder {
        authentication.requires_nothing()
            .and_then(|()| app.database.get()
                .map_err(|e| Internal(InternalError::DatabasePoolingError(e)))
            )
            .and_then(|conn|
                rpg_systems.load::<RpgSystem>(&conn)
                    .map_err(|e| Internal(InternalError::DatabaseError(e)))
            ).map(Json)
    }
}
