use auth::Token;
use database::*;
use dto::*;

pub fn get_rpgsystems(db: &Database, token: Token) -> Result<GetRpgSystems, Error> {
    //TODO: authentication

    //TODO Error mapping
    let rawsystems = db.get_all::<RpgSystem>()?;
    Ok(GetRpgSystems {
        rpgsystems: rawsystems,
    })
}
