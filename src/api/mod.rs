use crate::error::UserFacingError;
use actix_web::{HttpResponse, web};
use crate::models::Id;

mod accounts;
mod books;
mod guilds;
mod me;
mod rpg_systems;
mod titles;
mod users;

type MyResponder = Result<HttpResponse, UserFacingError>;

pub async fn redirect_collection_to_books(book_id: web::Path<Id>) -> MyResponder {
    Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", format!("../../books/{}", *book_id)))
        .finish())
}

pub async fn redirect_guild_to_books(path: web::Path<(Id, Id)>) -> MyResponder {
    let (_, book_id) = *path;
    Ok(HttpResponse::PermanentRedirect()
        .append_header(("Location", format!("../../../books/{}", book_id)))
        .finish())
}

// @formatter:off
#[rustfmt::skip]
pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
        .service(web::scope("/me")
            .service(web::resource("")
                .route(web::get().to(me::get))
                .route(web::post().to(me::post))
                .route(web::put().to(me::put))
                // .route(web::delete().to(me::delete))
            )
            .service(web::scope("/collection")
                .service(web::resource("")
                    .route(web::get().to(me::collection::get_all))
                    .route(web::post().to(me::collection::post))
                )
                .service(web::resource("/{id}")
                    .route(web::get().to(redirect_collection_to_books))
                    .route(web::delete().to(redirect_collection_to_books))
                )
            )
            // .service(web::scope("/inventory")
            //     .service(web::resource("")
            //         .route(web::get().to(inventory::get_all))
            //     )
            //     .service(web::resource("/{id}")
            //         .route(web::get().to(inventory::get_one))
            //         .route(web::put().to(inventory::put))
            //         .route(web::delete().to(inventory::delete))
            //     )
            // )
        )
        .service(web::scope("/accounts")
            .service(web::resource("").route(web::get().to(accounts::get_all)))
            .service(web::resource("/{id}")
                .route(web::get().to(accounts::get_one))
                .route(web::put().to(accounts::put))
                .route(web::delete().to(accounts::delete))
            )
        )
        .service(web::scope("/user")
            .service(web::resource("").route(web::get().to(users::get_all)))
            .service(web::resource("/{id}").route(web::get().to(users::get_one)))
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
                 .service(web::scope("/collection")
                     .service(web::resource("")
                         .route(web::get().to(guilds::collection::get_all))
                         .route(web::post().to(guilds::collection::post))
                     )
                     .service(web::resource("/titles")
                         .route(web::get().to(guilds::collection::titles::get_all))
                     )
                     .service(web::resource("/rpgsystems")
                         .route(web::get().to(guilds::collection::rpgsystems::get_all))
                     )
                     .service(web::resource("/{id}")
                         .route(web::get().to(redirect_guild_to_books))
                         .route(web::delete().to(redirect_guild_to_books))
                     )
                 )
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
                    .route(web::delete().to(rpg_systems::delete)),
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
                .route(web::delete().to(titles::delete)),
            ),
        )
        .service(web::scope("/books")
            .service(web::resource("")
                .route(web::get().to(books::get_all))
                .route(web::post().to(books::post)),
            )
            .service(web::resource("/{bookid}")
                .route(web::get().to(books::get_one))
                .route(web::put().to(books::put))
                .route(web::delete().to(books::delete)),
            ),
        )
    );
}
// @formatter:on
