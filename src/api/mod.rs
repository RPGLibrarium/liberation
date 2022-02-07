use actix_web::{HttpResponse, web};
use crate::error::UserFacingError;

mod me;
mod accounts;
mod guilds;
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
                .route(web::put().to(accounts::put))
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
        .service(web::scope("/guilds")
            .service(web::resource("")
                 .route(web::get().to(guilds::get_all))
                 .route(web::post().to(guilds::post)),
            )
            .service(web::scope("/{guildid}")
                 .service(web::resource("")
                     .route(web::get().to(guilds::get_one))
                     .route(web::put().to(guilds::put)),
                 )
             // .service(
             //     web::resource("/inventory")
             //         .route(web::get().to(get_guild_inventory))
             //         .route(web::post().to(post_guild_inventory)),
             // ),
            ),
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
    );
}
// @formatter:on



