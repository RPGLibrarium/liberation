use actix_web::{HttpRequest, Responder, web};


pub fn configure_v1(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/rpgsystems")
                .service(
                    web::resource("")
                        .route(web::get().to(get_rpg_systems))
                        .route(web::post().to(post_rpg_system)),
                )
                .service(
                    web::resource("/{systemid}")
                        .route(web::get().to(get_rpg_system))
                        .route(web::put().to(put_rpg_system))
                        .route(web::delete().to(delete_rpg_system)),
                ),
        )
        .service(
            web::scope("/titles")
                .service(
                    web::resource("")
                        .route(web::get().to(get_titles))
                        .route(web::post().to(post_rpg_system)),
                )
                .service(
                    web::resource("/{titleid}")
                        .route(web::get().to(get_title))
                        .route(web::put().to(put_title))
                        .route(web::delete().to(delete_title)),
                ),
        )
        .service(
            web::scope("/books")
                .service(
                    web::resource("")
                        .route(web::get().to(get_books))
                        .route(web::post().to(post_book)),
                )
                .service(
                    web::resource("/{bookid}")
                        .route(web::get().to(get_book))
                        .route(web::put().to(put_book))
                        .route(web::delete().to(delete_book)),
                ),
        )
        .service(
            web::scope("/guilds")
                .service(
                    web::resource("")
                        .route(web::get().to(get_guilds))
                        .route(web::post().to(post_guild)),
                )
                .service(
                    web::scope("/{guildid}")
                        .service(
                            web::resource("")
                                .route(web::get().to(get_guild))
                                .route(web::put().to(put_guild)),
                        )
                        .service(
                            web::resource("/inventory")
                                .route(web::get().to(get_guild_inventory))
                                .route(web::post().to(post_guild_inventory)),
                        ),
                ),
        )
        .service(
            web::scope("members")
                .service(web::resource("").route(web::get().to(get_members)))
                .service(
                    web::scope("/{memberid}")
                        .service(web::resource("/").route(web::get().to(get_member)))
                        .service(
                            web::resource("/inventory")
                                .route(web::get().to(get_member_inventory))
                                .route(web::post().to(post_member_inventory)),
                        ),
                ),
        );
}


fn get_rpg_system(state: web::Data<AppState>, _req: HttpRequest) -> impl Responder {
    claims.requires_nothing()
        .and_then(|()| rpg_systems
            .find(rpg_system_id)
            .first(conn)
            .map_err(|e|
                match e {
                    NotFound => LError::NotFound,
                    e => LError::DatabaseError(e)
                }
            )
        )

    let claims: Option<Claims> = assert_roles(&_req, vec![])?;

    let id: RpgSystemId = _req.match_info().query("systemid").parse::<RpgSystemId>()?;

    bus::get_rpgsystem(&state.db, claims, id).and_then(|system| Ok(HttpResponse::Ok().json(system)))
}
