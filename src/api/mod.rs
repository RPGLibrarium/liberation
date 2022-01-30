use actix_web::{HttpResponse, web};
use liberation::error::UserFacingError;

pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/rpgsystems")
                    .service(
                        web::resource("")
                            .route(web::get().to(rpg_systems::get_all))
                            .route(web::post().to(rpg_systems::post)),
                    )
                    .service(
                        web::scope("/{systemid}")
                            .service(
                                web::resource("")
                                    .route(web::get().to(rpg_systems::get_one))
                                    .route(web::put().to(rpg_systems::put))
                                // TODO: deletes would be nice, but seem complex
                                //.route(web::delete().to(delete_rpg_system)),
                            )
                        // TODO: could be useful
                        // .service(
                        //     web::resource("/titles")
                        //         .route(web::get().to(get_titles_in_rpg_system))
                        // )
                    )
            )
        // .service(
        //     web::scope("/titles")
        //         .service(
        //             web::resource("")
        //                 .route(web::get().to(get_titles))
        //                 .route(web::post().to(post_title)),
        //         )
        //         .service(
        //             web::resource("/{titleid}")
        //                 .route(web::get().to(get_title))
        //                 .route(web::put().to(put_title))
        //                 .route(web::delete().to(delete_title)),
        //         ),
        // )
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
        //         .service(
        //             web::resource("")
        //                 .route(web::get().to(get_guilds))
        //                 .route(web::post().to(post_guild)),
        //         )
        //         .service(
        //             web::scope("/{guildid}")
        //                 .service(
        //                     web::resource("")
        //                         .route(web::get().to(get_guild))
        //                         .route(web::put().to(put_guild)),
        //                 )
        //                 .service(
        //                     web::resource("/inventory")
        //                         .route(web::get().to(get_guild_inventory))
        //                         .route(web::post().to(post_guild_inventory)),
        //                 ),
        //         ),
        // )
        // .service(
        //     web::scope("members")
        //         .service(web::resource("").route(web::get().to(get_members)))
        //         .service(
        //             web::scope("/{memberid}")
        //                 .service(web::resource("/").route(web::get().to(get_member)))
        //                 .service(
        //                     web::resource("/inventory")
        //                         .route(web::get().to(get_member_inventory))
        //                         .route(web::post().to(post_member_inventory)),
        //                 ),
        //         ),
        // )
    );
}

type MyResponder = Result<HttpResponse, UserFacingError>;

mod rpg_systems;
