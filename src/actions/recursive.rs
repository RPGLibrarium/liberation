// inner_joins break IntelliJ. Putting all inner joins into a single file is an attempt to contain
// the problems

pub mod titles {
    use crate::actions::handle_db_errors;
    use crate::error::UserFacingError as UE;
    use crate::models::{RpgSystem, Title, TitleWithRpgSystem};
    use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

    pub fn recursive_list(conn: &MysqlConnection) -> Result<Vec<TitleWithRpgSystem>, UE> {
        use crate::schema::rpg_systems::dsl::*;
        use crate::schema::titles::dsl::*;
        let recursive = titles
            .inner_join(rpg_systems)
            .load::<(Title, RpgSystem)>(conn)
            .map_err(handle_db_errors)?
            .into_iter()
            .map(TitleWithRpgSystem::from)
            .collect();
        Ok(recursive)
    }
}

pub mod books {
    use crate::actions::handle_db_errors;
    use crate::error::UserFacingError as UE;
    use crate::models::{
        Book, BookWithTitle, BookWithTitleWithRpgSystem, RpgSystem, Title, TitleWithRpgSystem,
    };
    use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

    pub fn double_recursive_list(
        conn: &MysqlConnection,
    ) -> Result<Vec<BookWithTitleWithRpgSystem>, UE> {
        use crate::schema::books::dsl::*;
        use crate::schema::rpg_systems::dsl::*;
        use crate::schema::titles::dsl::*;

        let recursive = books
            .inner_join(titles.inner_join(rpg_systems))
            .load::<(Book, (Title, RpgSystem))>(conn)
            .map_err(handle_db_errors)?
            .into_iter()
            .map(|(book, title)| {
                BookWithTitleWithRpgSystem::from((book, TitleWithRpgSystem::from(title)))
            })
            .collect();
        Ok(recursive)
    }

    pub fn recursive_list(conn: &MysqlConnection) -> Result<Vec<BookWithTitle>, UE> {
        use crate::schema::books::dsl::*;
        use crate::schema::titles::dsl::*;

        let recursive = books
            .inner_join(titles)
            .load::<(Book, Title)>(conn)
            .map_err(handle_db_errors)?
            .into_iter()
            .map(|(book, title)| BookWithTitle::from((book, title)))
            .collect();
        Ok(recursive)
    }
}
