#![forbid(unsafe_code)]

#[macro_use]
extern crate clap;

use clap::{AppSettings, Parser};

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
        #[clap(default_value_t = String::from("127.0.0.1:8080"))]
        bind: String
    },
    Test,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();


    match cli.command {
        Commands::Serve { bind } => {
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

            let app_state = AppState {
                database: pool
            };

            // Start HTTP server
            HttpServer::new(move || {
                App::new()
                    // set up DB pool to be used with web::Data<Pool> extractor
                    .data(app_state.clone())
                    .wrap(middleware::Logger::default())
                    .service(
                        web::scope("/rpgsystems")
                            .route(web::get().to(api::get_rpg_systems))
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

            let claims = Authentication::is_member(1);
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
    use actix_web::{HttpRequest, web};
    use diesel::RunQueryDsl;
    use liberation::AppState;
    use liberation::claims::Authentication;
    use liberation::error::{InternalError, UserFacingError};
    use liberation::error::UserFacingError::Internal;
    use liberation::models::RpgSystem;
    use liberation::schema::rpg_systems::dsl::rpg_systems;

    // Don't ask to many questions about the arguments. With typing magic actix allows us to get the
    // state or arguments from the request. We can use up to function arguments to get data auto
    // magically out of the request.
    // https://github.com/actix/actix-web/blob/2a12b41456f40b28c1efe0ec6947e8f50ba22006/src/handler.rs
    // https://actix.rs/docs/extractors/
    pub fn get_rpg_systems(app: web::Data<AppState>, authentication: Authentication) -> Result<Vec<RpgSystem>, UserFacingError> {
        authentication.requires_nothing()
            .and_then(|()| app.database.get()
                .map_err(|e| Internal(InternalError::DatabasePoolingError(e)))
            )
            .and_then(|conn|
                rpg_systems.load(&conn)
                    .map_err(|e| Internal(InternalError::DatabaseError(e)))
            )
    }
}
