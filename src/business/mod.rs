use api::*;
use auth::Token;
use database::*;

pub fn get_rpgsystems(db: &Database, token: Token) -> Result<GetRpgSystems, Error> {
    //TODO: authentication

    //TODO Error mapping
    let rpgsystems = db.get_all::<RpgSystem>()?;
    Ok(GetRpgSystems { rpgsystems })
}

pub fn get_rpgsystem(
    db: &Database,
    token: Token,
    system_id: RpgSystemId,
) -> Result<GetRpgSystem, Error> {
    //TODO: Handle None()
    let system = db.get::<RpgSystem>(system_id)?.unwrap();
    let titles = db.get_titles_by_rpg_system(system_id)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetRpgSystem::new(system, titles))
}

pub fn post_rpgsystem(
    db: &Database,
    token: Token,
    system: &mut PutPostRpgSystem,
) -> Result<RpgSystemId, Error> {
    //TODO: Error handling
    //TODO: Assert Id is unset
    Ok(db.insert::<RpgSystem>(&mut system.rpgsystem)?)
}

pub fn put_rpgsystem(db: &Database, token: Token, system: &PutPostRpgSystem) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<RpgSystem>(&system.rpgsystem)?)
}

pub fn delete_rpgsystem(db: &Database, token: Token, systemid: RpgSystemId) -> Result<(), Error> {
    //TODO: Errorhandling, 404
    db.delete::<RpgSystem>(systemid)?;
    Ok(())
}

pub fn get_titles(db: &Database, token: Token) -> Result<GetTitles, Error> {
    //TODO: authentication

    //TODO Error mapping
    let tuples = db.get_titles_with_details()?;

    Ok(GetTitles {
        titles: tuples
            .into_iter()
            .map(|(title, system, stock, available)| {
                TitleWithSystem::new(title, system, stock, available)
            })
            .collect(),
    })
}

pub fn get_title(
    db: &Database,
    token: Token,
    system_id: RpgSystemId,
) -> Result<GetRpgSystem, Error> {
    let system = db.get::<RpgSystem>(system_id)?.unwrap();
    let titles = db.get_titles_by_rpg_system(system_id)?;
    //TODO: Error handling
    //Map Errors to API Errors
    //404 Not found

    Ok(GetRpgSystem::new(system, titles))
}

pub fn post_title(db: &Database, system: &mut PutPostRpgSystem) -> Result<RpgSystemId, Error> {
    //TODO: Error handling
    Ok(db.insert::<RpgSystem>(&mut system.rpgsystem)?)
}

pub fn put_title(db: &Database, system: &mut PutPostRpgSystem) -> Result<(), Error> {
    //TODO: Error handling
    Ok(db.update::<RpgSystem>(&system.rpgsystem)?)
}

pub fn delete_title(db: &Database, systemid: RpgSystemId) -> Result<bool, Error> {
    //TODO: Errorhandling
    Ok(db.delete::<RpgSystem>(systemid)?)
}
