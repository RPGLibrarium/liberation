use crate::actions;
use crate::actions::AccountAssertions;
use crate::authentication::scopes::{
    ARISTOCRAT_BOOKS_MODIFY, ARISTOCRAT_BOOKS_READ, COLLECTION_MODIFY, COLLECTION_READ,
    GUILDS_COLLECTION_MODIFY, GUILDS_READ,
};
use crate::authentication::Claims;
use crate::error::UserFacingError as UE;
use crate::models::{Account, Guild, Owner};
use diesel::MysqlConnection;

pub fn can_read_book_of_owner(
    conn: &MysqlConnection,
    claims: &Claims,
    owner: Owner,
) -> Result<(), UE> {
    match owner {
        _ if claims.contains_scope(ARISTOCRAT_BOOKS_READ) => Ok(()),
        Owner::Guild { .. } if claims.contains_scope(GUILDS_READ) => {
            let external_id = claims.external_account_id()?;
            actions::account::try_find_by_external_id(conn, external_id)?.assert_active()?;
            Ok(())
        }
        Owner::Member { id: owner_id } if claims.contains_scope(COLLECTION_READ) => {
            let external_id = claims.external_account_id()?;
            let account =
                actions::account::try_find_by_external_id(conn, external_id)?.assert_active()?;
            if account.id == owner_id {
                Ok(())
            } else {
                Err(UE::YouShallNotPass)
            }
        }
        _ => Err(UE::YouShallNotPass),
    }
}

pub fn can_modify_book_of_owner(
    conn: &MysqlConnection,
    claims: &Claims,
    owner: Owner,
) -> Result<(), UE> {
    match owner {
        _ if claims.contains_scope(ARISTOCRAT_BOOKS_MODIFY) => Ok(()),
        Owner::Guild { id } if claims.contains_scope(GUILDS_COLLECTION_MODIFY) => {
            let external_id = claims.external_account_id()?;
            let account =
                actions::account::try_find_by_external_id(conn, external_id)?.assert_active()?;
            let guild = actions::guild::find(conn, id)?;
            assert_librarian_for_guild(conn, &guild, &account)?;
            Ok(())
        }
        Owner::Member { id: owner_id } if claims.contains_scope(COLLECTION_MODIFY) => {
            let external_id = claims.external_account_id()?;
            let account =
                actions::account::try_find_by_external_id(conn, external_id)?.assert_active()?;
            if account.id == owner_id {
                Ok(())
            } else {
                Err(UE::YouShallNotPass)
            }
        }
        _ => Err(UE::YouShallNotPass),
    }
}

pub fn assert_librarian_for_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    account: &Account,
) -> Result<(), UE> {
    use diesel::result::Error as DE;
    use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
    use crate::models::Librarian;
    use crate::schema::librarians::dsl::*;
    let permission = librarians
        .filter(
            guild_id
                .eq(guild.id)
                .and(account_id.eq(account.id)),
        )
        .first::<Librarian>(conn)
        .map_err(|e| match e {
            // In this case no finding a result means the permission is missing. We can't use
            // the error handler.
            DE::NotFound => UE::YouShallNotPass,
            _ => actions::handle_db_errors(e),
        })?;
    assert_eq!(permission.account_id, account.id);
    assert_eq!(permission.guild_id, guild.id);
    Ok(())
}
