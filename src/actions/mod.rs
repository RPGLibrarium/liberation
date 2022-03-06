use crate::error::UserFacingError as UE;
use crate::models::*;
use crate::InternalError as IE;
use diesel::result::DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation};
use diesel::result::{Error as DE, Error};
use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};
use log::debug;

/// Mapping all the errors is anoying.
fn handle_db_errors(e: Error) -> UE {
    match e {
        DE::DatabaseError(UniqueViolation, cause) => {
            debug!("unique violation: {}", cause.message());
            UE::AlreadyExists
        }
        DE::DatabaseError(ForeignKeyViolation, cause) => {
            debug!("foreign key violation: {}", cause.message());
            UE::InvalidForeignKey
        }
        DE::NotFound => UE::NotFound,
        _ => UE::Internal(IE::DatabaseError(e)),
    }
}

pub mod account;
pub mod book;
pub mod guild;
mod recursive;
pub mod rpg_system;
pub mod title;

// Guilds Access control
pub fn assert_librarian_for_guild(
    conn: &MysqlConnection,
    guild: &Guild,
    account: &Account,
) -> Result<(), UE> {
    use crate::schema::librarians::dsl::*;
    let permission = librarians
        .filter(
            guild_id
                .eq(guild.guild_id)
                .and(account_id.eq(account.account_id)),
        )
        .first::<Librarian>(conn)
        .map_err(|e| match e {
            // In this case no finding a result means the permission is missing. We can't use
            // the error handler.
            DE::NotFound => UE::YouShallNotPass,
            _ => handle_db_errors(e),
        })?;
    assert_eq!(permission.account_id, account.account_id);
    assert_eq!(permission.guild_id, guild.guild_id);
    Ok(())
}

pub trait AccountAssertions {
    fn assert_active(self) -> Result<Account, UE>;
    fn assert_exists(self) -> Result<Account, UE>;
    fn assert_registered(self) -> Result<Account, UE>;
}

impl AccountAssertions for Option<Account> {
    fn assert_active(self) -> Result<Account, UE> {
        let account = self.ok_or(UE::NotRegistered)?;
        match account.active {
            true => Ok(account),
            false => Err(UE::Deactivated),
        }
    }
    fn assert_exists(self) -> Result<Account, UE> {
        self.ok_or(UE::NotFound)
    }
    fn assert_registered(self) -> Result<Account, UE> {
        self.ok_or(UE::NotRegistered)
    }
}

pub trait RowsAffectedAssertions {
    fn assert_row_existed(self) -> Result<usize, UE>;
}

impl RowsAffectedAssertions for usize {
    fn assert_row_existed(self) -> Result<usize, UE> {
        if self == 0 {
            Err(UE::NotFound)
        } else {
            Ok(self)
        }
    }
}
