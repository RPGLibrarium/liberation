use actix_web::{HttpResponse, web};
use crate::error::UserFacingError;

mod me;
mod accounts;
mod rpg_systems;
mod titles;
mod users;

type MyResponder = Result<HttpResponse, UserFacingError>;

// @formatter:off
pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
        .service(web::resource("/me")
            .route(web::get().to(me::get))
            .route(web::post().to(me::post))
            // .route(web::patch().to(me::patch))
            .route(web::delete().to(me::delete))
        )
        // .service(web::resource("/inventory")
        //              .route(web::get().to(member))
        //              .route(web::post().to(post_member_inventory)),
        // ),
        .service(web::scope("/accounts")
            .service(web::resource("")
                .route(web::get().to(accounts::get_all))
            )
            .service(web::resource("/{id}")
                .route(web::get().to(accounts::get_one))
                .route(web::patch().to(accounts::patch))
                .route(web::delete().to(accounts::delete))
            )
                 // .service(web::resource("/inventory")
                 //              .route(web::get().to(member))
                 //              .route(web::post().to(post_member_inventory)),
                 // ),
        )
        .service(web::scope("/user")
            .service(web::resource("")
                .route(web::get().to(users::get_all))
            )
            .service(web::resource("/{id}")
                .route(web::get().to(users::get_one))
            )
        )
        .service(web::scope("/rpgsystems")
            .service(web::resource("")
                .route(web::get().to(rpg_systems::get_all))
                .route(web::post().to(rpg_systems::post)),
            )
            .service(web::scope("/{systemid}")
                .service(web::resource("")
                    .route(web::get().to(rpg_systems::get_one))
                    .route(web::put().to(rpg_systems::put))
                 // TODO: deletes would be nice, but seem complex
                 // .route(web::delete().to(delete_rpg_system)),
                )
             // TODO: could be useful
             // .service(
             //     web::resource("/titles")
             //         .route(web::get().to(get_titles_in_rpg_system))
             // )
            )
        )
        .service(web::scope("/titles")
            .service(web::resource("")
                .route(web::get().to(titles::get_all))
                .route(web::post().to(titles::post)),
            )
            .service(web::resource("/{titleid}")
                .route(web::get().to(titles::get_one))
                .route(web::put().to(titles::put))
             // TODO: deletes would be nice, but seem complex
             // .route(web::delete().to(titles::delete)),
            ),
        )
        // .service(
        //     web::scope("/books")
        //         .service(
        //             web::resource("")
        //                 .route(web::get().to(get_books))
        //                 .route(web::post().to(post_book)),
        //         )
        //         .service(
        //             web::resource("/{bookid}")
        //                 .route(web::get().to(get_book))
        //                 .route(web::put().to(put_book))
        //                 .route(web::delete().to(delete_book)),
        //         ),
        // )
        // .service(
        //     web::scope("/guilds")
        //         .service(web::resource("")
        //                 .route(web::get().to(guilds::get_all))
        //                 .route(web::post().to(guilds::post)),
        //         )
        //         .service(web::scope("/{guildid}")
        //                 .service(
        //                     web::resource("")
        //                         .route(web::get().to(guilds::get_one))
        //                         .route(web::put().to(guilds::put)),
        //                 )
        //                 // .service(
        //                 //     web::resource("/inventory")
        //                 //         .route(web::get().to(get_guild_inventory))
        //                 //         .route(web::post().to(post_guild_inventory)),
        //                 // ),
        //         ),
        // )
    );
}
// @formatter:on

// mod guilds {
//     use actix_web::{HttpResponse, web};
//     use crate::actions;
//     use crate::api::MyResponder;
//     use crate::app::AppState;
//     use crate::auth::Authentication;
//     use crate::auth::roles::GUILDS_READ;
//     use crate::models::{Guild, NewGuild};
//
//     pub async fn get_all(app: web::Data<AppState>, authentication: Authentication) -> MyResponder {
//         authentication.requires_role(GUILDS_READ)?;
//         let conn = app.open_database_connection()?;
//         let guilds = actions::list_guilds(&conn)?;
//         Ok(HttpResponse::Ok().json(guilds))
//     }
//
//     pub async fn post(
//         app: web::Data<AppState>,
//         authentication: Authentication,
//         new_guild: web::Json<NewGuild>,
//     ) -> MyResponder {
//         authentication.requires_aristocrat()?;
//         let conn = app.open_database_connection()?;
//         let created = actions::create_guild(&conn, new_guild.into_inner())?;
//         Ok(HttpResponse::Created().json(created))
//     }
//
//     pub async fn get_one(
//         app: web::Data<AppState>,
//         authentication: Authentication,
//         id: web::Path<i32>,
//     ) -> MyResponder {
//         authentication.requires_aristocrat()
//             .or(authentication.requires_any_librarian().map(|_| ()))
//             .or(authentication.requires_any_member().map(|_| ()))?;
//         let conn = app.open_database_connection()?;
//         let guild = actions::find_guild(&conn, *id)?;
//         Ok(HttpResponse::Ok().json(guild))
//     }
//
//     pub async fn put(
//         app: web::Data<AppState>,
//         authentication: Authentication,
//         updated_guild: web::Json<Guild>,
//     ) -> MyResponder {
//         authentication.requires_aristocrat()?;
//         let conn = app.open_database_connection()?;
//         let updated = actions::update_guild(&conn, updated_guild.into_inner())?;
//         Ok(HttpResponse::Ok().json(updated))
//     }
// }


